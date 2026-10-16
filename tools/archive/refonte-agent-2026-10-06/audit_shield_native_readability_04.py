"""Native readability review on three backgrounds and diagnostic 30px tiles.

30px is a review scale only; native game assets stay 32px. No sprite is drawn
here; reviewed shield and approved body frames are only placed and resized.
"""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
OUT = BASE / "shield-review-04"
BEFORE = BASE / "shield-review-02"


def edges(im):
    result=[]
    for y in range(im.height):
        for x in range(im.width):
            p=im.getpixel((x,y))
            if p[3] and any(not(0<=x+dx<im.width and 0<=y+dy<im.height) or not im.getpixel((x+dx,y+dy))[3] for dx,dy in ((-1,0),(1,0),(0,-1),(0,1))):
                result.append(p)
    return result


def font(n):
    return ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf",n)


def main():
    manifest=json.loads((OUT/"review-shields-SHIELD-VIS01-04.json").read_text(encoding="utf-8"))
    floor_tile=Image.open(ROOT/manifest["floor_source"]["path"]).convert("RGBA")
    floor=Image.new("RGBA",(64,64))
    for y in (0,32):
        for x in (0,32):floor.alpha_composite(floor_tile,(x,y))
    backgrounds=[Image.new("RGBA",(64,64),(16,23,31,255)),Image.new("RGBA",(64,64),(173,182,184,255)),floor]
    board=Image.new("RGB",(790,625),(12,19,27))
    d=ImageDraw.Draw(board)
    d.text((18,12),"Lisibilité · les six boucliers",font=font(23),fill=(229,236,240))
    d.text((18,48),"Technomancien · case 30 px (test) / 32 px (natif) · trois fonds",font=font(13),fill=(160,182,196))
    audit=[]
    for k,label in enumerate(("Sombre","Clair","Sol du jeu")):
        x=130+k*210
        d.text((x+10,84),label,font=font(13),fill=(160,182,196))
        d.text((x+15,106),"30 px",font=font(11),fill=(160,182,196))
        d.text((x+85,106),"32 px",font=font(11),fill=(160,182,196))
    for i,c in enumerate(manifest["candidates"]):
        cid=c["candidate_id"]
        y=130+i*78
        d.text((18,y+20),cid,font=font(14),fill=(229,236,240))
        frame=Image.open(ROOT/next(r for r in manifest["compositions"] if r["candidate_id"]==cid and r["class_number"]==2 and r["shield_hand"]=="L" and r["other_hand_weapon"]=="pistol")["path"]).convert("RGBA")
        for k,bg in enumerate(backgrounds):
            composed=bg.copy()
            composed.alpha_composite(frame)
            board.paste(composed.resize((60,60),Image.Resampling.NEAREST),(130+k*210,y+2))
            board.paste(composed,(198+k*210,y))
        held=Image.open(ROOT/c["held_layer"]["path"]).convert("RGBA")
        prior=Image.open(BEFORE/(cid+"-held.png")).convert("RGBA")
        current_edges=edges(held)
        prior_edges=edges(prior)
        dark=lambda ps:sum(sum(p[:3])/3<45 for p in ps)
        amber=sum(p[3]>0 and p[0]>p[1]+15 and p[1]>p[2]+30 for p in held.get_flattened_data())
        if cid in ("EQ-S02","EQ-S04"):
            assert dark(current_edges)==len(current_edges)
        if cid=="EQ-S06":assert amber==0
        findings={"EQ-S01":"Ivory_center_and_dark_perimeter_remain_distinct;unchanged",
                  "EQ-S02":"Olive_green_can_blend_with_operator_armor;full_dark_perimeter_now_survives_native_reduction",
                  "EQ-S03":"Blue_panels_separated_by_dark_contour;unchanged",
                  "EQ-S04":"Source_gray_corners_and_noise_weakened_edge;full_dark_perimeter_and_broad_copper_planes_now_survive_native_reduction",
                  "EQ-S05":"Red_plate_and_dark_outline_remain_identifiable;unchanged",
                  "EQ-S06":"Gold_module_and_oversized_clasp_removed;continuous_charcoal_face_and_matching_steel_side_frame"}[cid]
        audit.append({"candidate_id":cid,"native_edge_pixels":len(current_edges),"native_dark_edge_pixels":dark(current_edges),
                      "previous_edge_pixels":len(prior_edges),"previous_dark_edge_pixels":dark(prior_edges),
                      "amber_pixels":amber,"visual_findings":findings,"visual_approval":"pending_user_review"})
    file=OUT/"shields-readability-30-32.png"
    board.save(file)
    record={"date":"2026-10-10","review_id":"SHIELD-VIS01","revision":4,"review_scale_30":"diagnostic_only_no_asset_or_runtime_change",
            "native_scale":32,"readability_backgrounds":["dark","light","actual_terrain"],
            "native_sheet_covers_profiles":[1,2,3,4,5],"both_hand_placements_checked":60,
            "diagnostic_sheet":{"path":file.relative_to(ROOT).as_posix(),"binary_sha256":hashlib.sha256(file.read_bytes()).hexdigest()},
            "candidates":audit,"runtime_integration":"none"}
    (OUT/"native-readability-audit-04.json").write_text(json.dumps(record,ensure_ascii=False,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"reviewed_candidates":6,"dark_contours_survive":["EQ-S02","EQ-S04"],"EQ-S06_amber_pixels":0,"diagnostic_scales":[30,32]}))


if __name__=="__main__":main()
