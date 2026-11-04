"""Installed pixels, review approvals, package inventory and native evidence."""
import hashlib
import json
import sys
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/biped-integration-01'


def main():
    manifest=json.loads((ROOT/'assets/animations/biped-lateral-01/manifest.json').read_text(encoding='utf-8'))
    provenance=json.loads((ROOT/'docs/reviews/animation-polish-01/BIPED-INTEGRATION01-ASSETS.json').read_text(encoding='utf-8'))
    captures=json.loads((OUT/'current/checks.json').read_text(encoding='utf-8'))
    assert captures['captures']==660 and captures['vertical_frames_identical']==120 and captures['warm_prepare_ms']==0
    frozen=0
    for name in ['BIPED-LATERAL03-ACCEPTANCE-r03-partial.json','BIPED-LATERAL03-ACCEPTANCE-r04.json','BIPED-LATERAL04-ACCEPTANCE-r01.json']:
        record=json.loads((ROOT/'docs/reviews/animation-polish-01'/name).read_text(encoding='utf-8'))
        for asset in record['assets']+record.get('front_references_retained',[]):
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']; frozen+=1
    moving=0; fronts=0
    for e,p in zip(manifest['entries'],provenance['models']):
        assert e['number']==p['number'] and e['variant']==0
        assert hashlib.sha256((ROOT/e['file']).read_bytes()).hexdigest()==p['sha256']
        assert hashlib.sha256((ROOT/p['walk_source']).read_bytes()).hexdigest()==p['walk_sha256']
        installed=Image.open(ROOT/e['file']).convert('RGBA'); old=Image.open(ROOT/p['walk_source']).convert('RGBA'); w,h=e['cell']
        for side in range(2):
            for frame in range(8):
                box=(frame*w,side*h,(frame+1)*w,(side+1)*h)
                assert installed.crop(box).tobytes()==old.crop(box).tobytes(); moving+=1
            assert installed.crop((8*w,side*h,9*w,(side+1)*h)).tobytes()==installed.crop((8*w,0,9*w,h)).tobytes(); fronts+=1
        actual=next(r for r in captures['models'] if r['number']==e['number'])
        assert actual['accepted_pixels_match'] and not actual['draw_resource_work'] and len(actual['captures'])==66
    sys.path.insert(0,str(ROOT/'tools/distribution')); from package_release import inventory
    files=inventory(); assert all(e['file'] in files for e in manifest['entries'])
    assert manifest['step_ms']==400 and manifest['cycle_ms']==800
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    def labelled(n,file):
        im=Image.open(OUT/'current'/f'MON-{n:03}-0'/file).convert('RGB'); draw=ImageDraw.Draw(im)
        assert im.size==(384,192)
        draw.rectangle((0,0,383,31),fill='#1e2831')
        for x,label in [(5,'Validé'),(197,'Rendu normal intégré')]: draw.text((x,3),label,font=font,fill='#c9e9d2')
        draw.text((5,17),f'MON-{n:03} · '+('gauche' if file.startswith('left') else 'droite')+' · 400 ms par pas',font=font,fill='#d2dde1')
        return im
    summary=Image.new('RGB',(768,960),'#1e2831')
    for i,e in enumerate(manifest['entries']): summary.paste(labelled(e['number'],'left-09.png'),((i%2)*384,(i//2)*192))
    summary.save(OUT/'all-ten-native.png')
    proof=Image.new('RGB',(768,384),'#1e2831')
    for i,(n,file) in enumerate([(164,'left-09.png'),(165,'right-25.png'),(146,'left-09.png'),(151,'right-25.png')]):
        proof.paste(labelled(n,file),((i%2)*384,(i//2)*192))
    proof.save(OUT/'native-proof.png')
    report={'review':'BIPED-INTEGRATION01','normal_game_enabled':True,'models':[e['number'] for e in manifest['entries']],
            'frozen_source_hashes_verified':frozen,'moving_poses_pixel_identical':moving,'identical_left_right_fronts':fronts,
            'normal_render_captures_identical':660,'vertical_frames_unchanged':120,'draw_resource_work':False,'cold_prepare_ms':captures['cold_prepare_ms'],
            'warm_prepare_ms':0,'step_ms':400,'cycle_ms':800,'other_variants_enabled':False,'runtime_inventory_verified':True}
    (OUT/'native-validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8'); print(json.dumps(report))


if __name__=='__main__':main()
