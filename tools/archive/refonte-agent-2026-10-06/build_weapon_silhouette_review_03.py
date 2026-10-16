"""Preserve the accepted rifle steel/anthracite palette in the shared rig bank."""
from pathlib import Path
import runpy

H = runpy.run_path(str(Path(__file__).with_name("build_weapon_silhouette_review_01.py")), run_name="weapon_layer_helpers")
if __name__ == "__main__":
    H["main"](revision=3)
