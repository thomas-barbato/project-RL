"""Coverage, provenance and attachment audit for the review-only comic catalogue."""
from pathlib import Path
import argparse,collections,colorsys,hashlib,json
from PIL import Image
ROOT=Path(__file__).resolve().parents[2];OUT=Path(__file__).resolve().parent/'comic-catalogue-review-01'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--complete',action='store_true');args=parser.parse_args()
    catalogue=json.loads((OUT/'catalogue.json').read_text(encoding='utf-8'));gallery=json.loads((OUT/'gallery-data.json').read_text(encoding='utf-8'))
    rig=json.loads((ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json').read_text(encoding='utf-8'))
    assert digest(ROOT/catalogue['source_game_catalog']['path'])==catalogue['source_game_catalog']['sha256']
    assert digest(ROOT/catalogue['active_rig']['path'])==catalogue['active_rig']['sha256']
    for f in rig['files']:assert digest(ROOT/f['path'])==f['binary_sha256'],f['path']
    pending=[];prompts=[];hashes=collections.defaultdict(list);native_hashes=collections.defaultdict(list);records=[];pink=[]
    frames_by_id={e['review_id']:e for e in gallery['entries']}
    for e in catalogue['entries']:
        cid=e['review_id'];path=ROOT/e['source_path'];provenance=path.with_name(path.stem+'-generation.json')
        if not path.exists() or not provenance.exists():pending.append(cid);continue
        p=json.loads(provenance.read_text(encoding='utf-8'));assert p['review_id']==cid and p['mode'] in ('builtin_image_gen','builtin_imagegen','builtin')
        prompts.append(p);h=digest(path);hashes[h].append(cid)
        with Image.open(path) as raw:
            im=raw.convert('RGBA');assert im.getchannel('A').getbbox(),cid
            assert min(im.getpixel(v)[3] for v in ((0,0),(im.width-1,0),(0,im.height-1),(im.width-1,im.height-1)))==0,cid
        rendered=frames_by_id.get(cid)
        if not rendered or not rendered['frames']:
            pending.append(cid);continue
        with Image.open(OUT/rendered['icon']) as im:
            assert im.size==(32,32);native_hashes[hashlib.sha256(im.convert('RGBA').tobytes()).hexdigest()].append(cid)
            count=0
            for r,g,b,a in im.convert('RGBA').get_flattened_data():
                hue,sat,val=colorsys.rgb_to_hsv(r/255,g/255,b/255)
                count+=int(a>127 and .855<hue<.965 and sat>.4 and val>.35)
            if count>2:pink.append({'review_id':cid,'pink_pixels':count})
        for f in rendered['frames']:
            meta=f['metadata'];assert meta['body_source_ready'],cid
            assert meta['body_native_height']==32,cid
            with Image.open(OUT/f['native']) as im:assert im.size==(128,128),cid
            with Image.open(OUT/f['normal']) as im:assert im.size==(512,512),cid
            if meta.get('separate_arms_and_fingers'):assert min(meta['finger_weapon_contacts'])>=1,cid
            if e['kind']=='shield':assert meta['shield_center_on_hand'] and not meta['hands_in_front_of_shield'],cid
        records.append({'review_id':cid,'content_id':e['content_id'],'source_sha256':h,'placements':len(rendered['frames'])})
    duplicates=[v for v in hashes.values() if len(v)>1];assert not duplicates,duplicates
    expected={e['review_id'] for e in catalogue['entries']};assert len(expected)==178
    if args.complete:assert not pending,pending
    report={'review_id':'BD-CATALOGUE01','selected_models':len(records),'target':178,'pending':pending,
        'unique_source_drawings':len(hashes),'duplicate_source_drawings':duplicates,
        'duplicate_native_icons':[v for v in native_hashes.values() if len(v)>1],
        'pink_native_flags':pink,'rig_files_unchanged':len(rig['files']),'catalogue_unchanged':True,
        'placements':sum(e['placements'] for e in records),'logical_tile':[32,32],'review_canvas':[128,128],
        'approval':'pending;not_integrated','method':'builtin_image_gen_per_model;conditioning_and_separate_layer_composition',
        'visual_acceptance':'not_inferred_from_technical_checks','results':records}
    (OUT/'coverage-and-provenance.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (OUT/'prompts-used.json').write_text(json.dumps(prompts,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({k:report[k] for k in ('selected_models','target','pending','unique_source_drawings','placements','rig_files_unchanged','pink_native_flags','duplicate_native_icons')},ensure_ascii=True))
if __name__=='__main__':main()
