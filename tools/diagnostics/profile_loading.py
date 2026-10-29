"""Run fresh-process loading diagnostics sequentially, with personal-file checks."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]


def personal_files() -> dict[str, str]:
    directory = Path(os.environ.get("APPDATA", ROOT / "config")) / "ProjectRL"
    return {
        str(path): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in directory.rglob("*") if path.is_file()
    }


def summarize(reports: list[dict]) -> dict:
    scenes: dict[str, list[dict]] = {}
    stages: dict[tuple[str, str], list[float]] = {}
    for report in reports:
        for row in report["scenes"]:
            scenes.setdefault(row["scene"], []).append(row)
        totals: dict[tuple[str, str], float] = {}
        for sample in report["samples"]:
            phase = sample["phase"].split("/frame-")[0]
            key = (phase, sample["stage"])
            totals[key] = totals.get(key, 0.0) + sample["elapsed_ms"]
        for key, total in totals.items():
            stages.setdefault(key, []).append(total)
    return {
        "scenes": {
            name: {
                "runs": len(rows),
                "first_cpu_ms": [row["first_cpu_ms"] for row in rows],
                "median_first_cpu_ms": statistics.median(row["first_cpu_ms"] for row in rows),
                "steady_cpu_median_ms": statistics.median(
                    row["steady_cpu_excluding_first_10"]["median_ms"] for row in rows),
                "max_cpu_ms": [row["all_cpu"]["max_ms"] for row in rows],
                "states_identical_across_runs": len({
                    (row["engine_fingerprint"], row["terminal_fingerprint"]) for row in rows
                }) == 1,
            }
            for name, rows in scenes.items()
        },
        "stage_totals_ms": [
            {"phase": phase, "stage": stage, "totals": values, "median": statistics.median(values)}
            for (phase, stage), values in sorted(stages.items())
        ],
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", type=Path, default=ROOT / "target/loading/project-rl.exe")
    parser.add_argument("--data-root", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--runs", type=int, default=3)
    parser.add_argument("--build-description", default="Optimization not inferred from the executable")
    parser.add_argument("--fixtures", nargs="+", choices=["route", "sewer", "effects"], default=["route", "effects"])
    args = parser.parse_args()
    if args.runs < 1:
        parser.error("--runs must be positive")
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=False)
    executable = args.executable.resolve()
    before = personal_files()
    environment = os.environ.copy()
    environment["PROJECT_RL_DATA_DIR"] = str(args.data_root.resolve())
    startup = None
    if os.name == "nt":
        startup = subprocess.STARTUPINFO()
        startup.dwFlags |= subprocess.STARTF_USESHOWWINDOW
        startup.wShowWindow = 0
    reports = []
    controls = []
    # Include an untraced control after the first pair, using the identical binary.
    schedule = [True, False] + [True] * (args.runs - 1)
    wall_times = []
    for index, traced in enumerate(schedule):
        for fixture in args.fixtures:
            directory = args.output / f"{index:02d}-{'traced' if traced else 'control'}-{fixture}"
            directory.mkdir()
            command = [str(executable), "--ui-cold-performance-loading", str(directory), f"--loading-fixture={fixture}"]
            if not traced:
                command.append("--no-loading-trace")
            started = time.perf_counter()
            with (directory / "stdout.log").open("w", encoding="utf-8") as out, (directory / "stderr.log").open("w", encoding="utf-8") as err:
                process = subprocess.run(command, cwd=ROOT, env=environment, stdout=out, stderr=err,
                                         startupinfo=startup, timeout=180)
            if process.returncode:
                raise RuntimeError(f"{fixture} failed ({process.returncode}): {(directory / 'stderr.log').read_text(encoding='utf-8')}")
            report = json.loads((directory / "loading.json").read_text(encoding="utf-8"))
            (reports if traced else controls).append(report)
            wall_times.append({"fixture": fixture, "traced": traced, "wall_seconds": time.perf_counter()-started})
            print(f"{directory.name}: {len(report['scenes'])} scenes; {len(report['samples'])} loading events", flush=True)
    after = personal_files()
    if before != after:
        raise RuntimeError("Personal files changed during the diagnostic; inspect before continuing")
    result = {
        "schema": 1, "executable": str(executable),
        "executable_sha256": hashlib.sha256(executable.read_bytes()).hexdigest(),
        "data_root": str(args.data_root.resolve()), "platform": platform.platform(),
        "logical_cpu_count": os.cpu_count(), "build_description": args.build_description,
        "personal_file_hashes_unchanged": True, "personal_files_checked": len(before),
        "traced": summarize(reports), "untraced_control": summarize(controls), "wall_times": wall_times,
    }
    for name, row in result["traced"]["scenes"].items():
        control = result["untraced_control"]["scenes"].get(name)
        if control and not all(
            sample["engine_fingerprint"] == reference["engine_fingerprint"]
            and sample["terminal_fingerprint"] == reference["terminal_fingerprint"]
            for report in reports for sample in report["scenes"] if sample["scene"] == name
            for report in controls for reference in report["scenes"] if reference["scene"] == name
        ):
            raise RuntimeError(f"{name}: traced and untraced state differs")
    result["traced_and_control_states_identical"] = True
    (args.output / "summary.json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps(result["traced"]["scenes"], indent=2), flush=True)


if __name__ == "__main__":
    main()
