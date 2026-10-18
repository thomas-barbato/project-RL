"""Original, procedural typing sounds for DIALOGUE-AUDIO-01; review only."""
from pathlib import Path
import hashlib
import json
import wave

import numpy as np

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/handcrafted-places-review-01/current"
DOC = ROOT / "docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01"
RATE = 48000


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    checks = json.loads((DOC / "dialogue-bubbles-02.json").read_text(encoding="utf-8"))["checks"]
    duration = checks["characters"] / checks["characters_per_second"]
    text = checks["text"]
    results = []
    for variant, label in enumerate([
        "Terminal feutré", "Relais mécanique", "Impulsions numériques"
    ], 1):
        rng = np.random.default_rng(1810 + variant)
        track = np.zeros(round(RATE * duration))
        events = 0
        for i, char in enumerate(text):
            # Fewer ticks than characters, and no tick on spaces/punctuation.
            if i % 2 or not char.isalnum():
                continue
            start = round((i + 1) / len(text) * duration * RATE)
            t = np.arange(round(RATE * 0.018)) / RATE
            noise = rng.normal(size=t.size)
            soft_noise = np.convolve(noise, np.ones(9) / 9, mode="same")
            pitch = rng.uniform(0.91, 1.09)
            attack = np.minimum(t / 0.001, 1)
            if variant == 1:
                pulse = (0.65 * soft_noise + 0.25 * np.sin(2*np.pi*630*pitch*t))
                pulse *= attack * np.exp(-t / 0.003)
            elif variant == 2:
                pulse = 0.52 * soft_noise + 0.26 * np.sin(2*np.pi*280*pitch*t)
                pulse += 0.08 * np.sin(2*np.pi*1180*pitch*t)
                pulse *= attack * np.exp(-t / 0.004)
            else:
                phase = 2*np.pi*pitch*(790*t - 11000*t*t)
                pulse = (0.33*np.sin(phase) + 0.13*soft_noise)
                pulse *= attack * np.exp(-t / 0.0038)
            n = min(len(pulse), len(track) - start)
            if n > 0:
                track[start:start+n] += pulse[:n] * rng.uniform(0.78, 1)
                events += 1
        track *= min(1, 0.14 / max(np.max(np.abs(track)), 1e-12))
        track[-480:] *= np.linspace(1, 0, 480)
        pcm = np.round(track * 32767).astype("<i2")
        path = OUT / f"dialogue-audio-{variant:02}.wav"
        with wave.open(str(path), "wb") as audio:
            audio.setparams((1, 2, RATE, 0, "NONE", "not compressed"))
            audio.writeframes(pcm.tobytes())
        assert np.isfinite(track).all() and np.max(np.abs(track)) <= 0.141
        results.append(dict(code=f"DIALOGUE-AUDIO-01-{variant:02}", name=label,
                            file=path.name, duration_seconds=duration, events=events,
                            peak=float(np.max(np.abs(track))),
                            sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
    report = dict(review_id="DIALOGUE-AUDIO-01", date="2026-10-18",
                  status="proposals_awaiting_listening", production_enabled=False,
                  source="Original procedural synthesis; no external recording",
                  sample_rate=RATE, channels=1, bits=16, candidates=results)
    (DOC / "dialogue-audio-01.json").write_text(
        json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, ensure_ascii=False))


if __name__ == "__main__":
    prepare()
