"""Duplicate original drawings for palette proposals; no recoloring or painting."""
from pathlib import Path
import hashlib
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
GUNS = ROOT / "artifacts/tir-brutal-propositions-2026-10-05"

weapons = Image.new("RGBA", (1536,1024))
for row, index in enumerate((1,4,3,8)):
    tile = Image.open(GUNS / f"arme-{index:02}-128.png").convert("RGBA")
    tile = tile.resize((256,256),Image.Resampling.NEAREST)
    for col in range(6):
        weapons.alpha_composite(tile, (col*256,row*256))
weapons.save(OUT / "armes-cibles.png")

armor = Image.new("RGBA", (1536,768))
atlas = Image.open(ROOT / "assets/sprites/cyberpunk/player-poses-unarmed-128.png").convert("RGBA")
for row, index in enumerate((3,11,14)):
    x,y = index%16*128,index//16*128
    tile=atlas.crop((x,y,x+128,y+128)).resize((256,256),Image.Resampling.NEAREST)
    for col in range(6):
        armor.alpha_composite(tile, (col*256,row*256))
armor.save(OUT / "armures-cibles.png")

active = list((ROOT / "assets/sprites/cyberpunk").iterdir())
active += [ROOT/name for name in ("src/entity/inventory.rs", "src/entity/ground_item.rs",
                                "src/game/game_state.rs", "src/game/expedition.rs",
                                "src/game/rules.rs", "src/ascii_app.rs", "src/suspension.rs")]
hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in active if p.is_file()}
(OUT / "runtime-avant.json").write_text(json.dumps(hashes,indent=2),encoding="utf-8")
print("Palette targets: 4 weapons x 6; 3 outfits x 6. Active files hashed.")
