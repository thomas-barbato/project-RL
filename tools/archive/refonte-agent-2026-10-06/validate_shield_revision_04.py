"""Validate scoped shield corrections and prepare their numbered review sheet."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
OLD = BASE / "shield-review-02"
NEW = BASE / "shield-review-04"


def reference(path):
    return {"reference": path.relative_to(ROOT).as_posix(), "reference_sha256": hashlib.sha256(path.read_text(encoding="utf-8").encode()).hexdigest()}


def binary_ref(path):
    return {"path": path.relative_to(ROOT).as_posix(), "binary_sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def font(n):
    return ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", n)


def main():
    old_path = OLD / "review-shields-SHIELD-VIS01-02.json"
    new_path = NEW / "review-shields-SHIELD-VIS01-04.json"
    old = json.loads(old_path.read_text(encoding="utf-8"))
    new = json.loads(new_path.read_text(encoding="utf-8"))
    old_c = {r["candidate_id"]: r for r in old["candidates"]}
    revised = {"EQ-S02", "EQ-S04", "EQ-S06"}
    for r in new["candidates"]:
        cid = r["candidate_id"]
        for field in ("source", "held_layer", "inventory_tile"):
            if cid not in revised:
                assert r[field]["binary_sha256"] == old_c[cid][field]["binary_sha256"], (cid, field)
            else:
                assert r[field]["binary_sha256"] != old_c[cid][field]["binary_sha256"], (cid, field)
    key = lambda r: (r["candidate_id"], r["class_number"], r["shield_hand"], r["other_hand_weapon"])
    old_frames = {key(r):r for r in old["compositions"]}
    preserved = 0
    for r in new["compositions"]:
        baseline = old_frames[key(r)]
        assert r["native_hand_anchor"] == baseline["native_hand_anchor"]
        assert r["source_frame"] == baseline["source_frame"]
        assert r["qa"]["outside_shield_pixel_changes"] == 0
        if r["candidate_id"] not in revised:
            assert r["binary_sha256"] == baseline["binary_sha256"]
            preserved += 1
    assert preserved == 90
    checked = []
    def walk(v):
        if isinstance(v,dict):
            if "path" in v and "binary_sha256" in v:
                p=ROOT/v["path"]
                assert hashlib.sha256(p.read_bytes()).hexdigest()==v["binary_sha256"],p
                checked.append(v["path"])
            if "reference" in v and "reference_sha256" in v:
                p=ROOT/v["reference"]
                assert hashlib.sha256(p.read_text(encoding="utf-8").encode()).hexdigest()==v["reference_sha256"],p
                checked.append(v["reference"])
            for value in v.values():walk(value)
        elif isinstance(v,list):
            for value in v:walk(value)
    walk(new)
    floor_tile = Image.open(ROOT / new["floor_source"]["path"]).convert("RGBA")
    floor = Image.new("RGBA",(64,64))
    for y in (0,32):
        for x in (0,32):floor.alpha_composite(floor_tile,(x,y))
    board = Image.new("RGB",(1110,395),(12,19,27))
    d=ImageDraw.Draw(board)
    d.text((18,12),"04 et 06 · versions industrielles",font=font(23),fill=(229,236,240))
    d.text((18,48),"SHIELD-VIS01 · révision 2 · placement conservé · taille native et zoom ×2",font=font(13),fill=(160,182,196))
    for n,name in enumerate(("Bastion","Technomancien","Traqueur","Arcaniste","Métamorphe"),1):
        d.text((190+(n-1)*181,86),name,font=font(13),fill=(160,182,196))
    for i,cid in enumerate(("EQ-S04","EQ-S06")):
        y=117+i*134
        d.text((18,y),cid,font=font(14),fill=(229,236,240))
        obj=Image.open(ROOT/next(r for r in new["candidates"] if r["candidate_id"]==cid)["inventory_tile"]["path"]).convert("RGBA")
        board.paste(obj,(45,y+27),obj)
        objzoom=obj.resize((64,64),Image.Resampling.NEAREST)
        board.paste(objzoom,(90,y+12),objzoom)
        for n in range(1,6):
            frame=Image.open(ROOT/next(r for r in new["compositions"] if key(r)==(cid,n,"L","pistol"))["path"]).convert("RGBA")
            composed=floor.copy()
            composed.alpha_composite(frame)
            x=178+(n-1)*181
            board.paste(composed,(x,y+2))
            zoom=composed.resize((128,128),Image.Resampling.NEAREST).crop((16,16,112,112))
            board.paste(zoom,(x+64,y-10))
    board_path=NEW/"shields-04-06-revised-native.png"
    board.save(board_path)
    decision={"date":"2026-10-10","review_id":"SHIELD-VIS01","revision":4,
              "user_feedback":{"native_readability":"Les contours du cuivre sont peu visibles; observer tous les boucliers à 30 px",
                               "EQ-S06":"Reprendre le style du reste du bouclier à la place de la partie en or, pas simplement changer l'or en acier"},
              "accepted_scope":"concrete_placement_of_shields;hand_and_forearm_occlusion",
              "revised_scope":["EQ-S02","EQ-S04","EQ-S06"],
              "other_candidates":"unchanged_pixels;no_new_blanket_approval_inferred",
              "new_style_proposal":{"EQ-S02":"olive_panels_preserved;continuous_dark_perimeter;internal_steel_ridge",
                                    "EQ-S04":"copper_panels_and_horizontal_reinforcement_preserved;continuous_dark_perimeter;less_surface_noise",
                                    "EQ-S06":"former_amber_module_removed_and_structurally_rebuilt;charcoal_front_plate_and_steel_rim_continue_on_right_side;no_gold_or_clasp"},
              "previous_review":reference(old_path),"current_review":reference(new_path),
              "revised_candidates_sheet":binary_ref(board_path),"visual_approval":"pending_revised_candidate_review",
              "runtime_integration":"not_integrated"}
    (NEW/"feedback-and-revision-04.json").write_text(json.dumps(decision,ensure_ascii=False,indent=2)+"\n",encoding="utf-8",newline="\n")
    validation={"date":"2026-10-10","review_id":"SHIELD-VIS01","revision":4,
                "verified_references":len(checked),"composition_count":180,
                "unchanged_candidates":["EQ-S01","EQ-S03","EQ-S05"],
                "unchanged_compositions_verified":preserved,"all_hand_anchors_preserved":True,
                "pixels_outside_shield_changes":0,"revised_candidates":["EQ-S02","EQ-S04","EQ-S06"],
                "current_review":reference(new_path),"revised_preview":binary_ref(board_path),
                "active_assets_modified":False,"runtime_integration":"none"}
    (NEW/"validation-04.json").write_text(json.dumps(validation,ensure_ascii=False,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"references":len(checked),"unchanged_compositions":preserved,"hand_anchors_preserved":True,"runtime_integration":"none"}))


if __name__=="__main__":main()
