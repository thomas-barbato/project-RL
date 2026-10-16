"""Summarize final test failure names and like-for-like CPU diagnostic scenes."""
import json
import re
from pathlib import Path

STAGE = Path(__file__).resolve().parent
PREVIOUS = STAGE.parent / "palettes-integration-2026-10-05"
log = (STAGE / "tests-client-complets.log").read_text(encoding="utf-8", errors="replace")
if "test result:" not in log:
    raise SystemExit("Client suite still running")
failures = sorted(set(re.findall(r"^test ([^\r\n ]+) \.\.\. FAILED", log, re.M)))
baseline = json.loads((PREVIOUS / "comparaison-tests-client.json").read_text(encoding="utf-8"))["current_failures"]
rechecked = []
for name in ["tests-firearm-resume-final.log", "tests-menu-resume-final.log"]:
    recheck_path = STAGE / name
    if recheck_path.exists():
        recheck_log = recheck_path.read_text(encoding="utf-8", errors="replace")
        if "test result: ok." in recheck_log:
            rechecked.extend(re.findall(r"^test ([^\r\n ]+) \.\.\. ok", recheck_log, re.M))
observed_new = sorted(set(failures) - set(baseline))
fixed = [name for name in rechecked if "firearm_models_all_fire" in name]
intermittent = [name for name in rechecked if "main_menu_can_resume" in name]
result = {
    "test_result": re.findall(r"^test result:.*$", log, re.M)[-1],
    "baseline_count": len(baseline), "current_count": len(failures),
    "observed_new_failures": observed_new,
    "fixed_and_rechecked": sorted(set(observed_new) & set(fixed)),
    "intermittent_passing_in_isolation": sorted(set(observed_new) & set(intermittent)),
    "new_failures": sorted(set(observed_new) - set(rechecked)),
    "baseline_only": sorted(set(baseline) - set(failures)),
    "current_failures": failures,
}
(STAGE / "comparaison-tests-client.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
print(json.dumps({k:v for k,v in result.items() if k != "current_failures"}, indent=2))
if (STAGE / "performance-final/timings.json").exists():
    before = json.loads((PREVIOUS / "performance-warmed-final/timings.json").read_text())
    after = json.loads((STAGE / "performance-final/timings.json").read_text())
    compared = []
    for a in after["scenes"]:
        b = next(b for b in before["scenes"] if b["scene"] == a["scene"])
        compared.append({"scene": a["scene"], "before_cpu_median_ms": b["cpu"]["median_ms"],
                         "after_cpu_median_ms": a["cpu"]["median_ms"],
                         "before_first_cpu_ms": b["first_cpu_ms"],
                         "after_first_cpu_ms": a["first_cpu_ms"]})
    (STAGE / "comparaison-performance.json").write_text(json.dumps(compared, indent=2) + "\n")
    print(json.dumps([a for a in compared if a["scene"] in ("world", "inventory")], indent=2))
if result["new_failures"]:
    raise SystemExit("New client failure names require investigation")
