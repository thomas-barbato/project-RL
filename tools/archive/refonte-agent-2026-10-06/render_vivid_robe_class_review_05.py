"""Saturated red material proposal; preserve the downward pistol from review4."""
from pathlib import Path
import runpy

HELP = runpy.run_path(str(Path(__file__).with_name("render_pistol_robe_class_review_04.py")), run_name="palette_composition_helpers")
if __name__ == "__main__":
    HELP["main"](revision=5, red_ramp=[[76, 8, 15], [224, 22, 32], [255, 58, 50]])
