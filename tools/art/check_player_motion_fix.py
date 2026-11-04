"""Verify colored outfit routing and the shared player exposure at native size."""
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/player-motion-fix-01'


def main():
    d=json.loads((OUT/'current/checks.json').read_text(encoding='utf-8'))
    assert len(d['cases'])==14 and d['player_texture_exposure_checks']==364
    for c in d['cases']:
        assert c['lighting']=={'gain':1.18,'lift':0.30,'edge':0.012}
        assert len(c['directions'])==4 and all(p['distinct_drawings']==6 for p in c['directions'])
    for tier in range(5):
        a=Image.open(OUT/'current'/f'traqueuse-{tier}-original.png').convert('RGB')
        b=Image.open(OUT/'current'/f'traqueuse-{tier}-blue.png').convert('RGB')
        assert ImageChops.difference(a.crop((160,32,800,192)),b.crop((160,32,800,192))).getbbox() is None
    saved=json.loads((OUT/'after-saved-audit.json').read_text(encoding='utf-8'))
    player=next(p for p in saved['actors'] if p['player']); assert player['normal_vertical_cycle']
    vertical=[p['active_render_facing'] for p in saved['movement_trials'] if p['player'] and p['motion_facing']==0]
    assert vertical==[-2,2]
    frozen=0
    for name in ['BIPED-LATERAL03-ACCEPTANCE-r03-partial.json','BIPED-LATERAL03-ACCEPTANCE-r04.json','BIPED-LATERAL04-ACCEPTANCE-r01.json']:
        record=json.loads((ROOT/'docs/reviews/animation-polish-01'/name).read_text(encoding='utf-8'))
        for a in record['assets']+record.get('front_references_retained',[]):
            assert hashlib.sha256((ROOT/a['file']).read_bytes()).hexdigest()==a['sha256']; frozen+=1
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    summary=Image.new('RGB',(800,960),'#1e2831'); proof=Image.new('RGB',(800,384),'#1e2831')
    for tier in range(5):
        im=Image.open(OUT/'current'/f'traqueuse-{tier}-blue.png').convert('RGB'); draw=ImageDraw.Draw(im)
        draw.rectangle((0,0,799,31),fill='#1e2831')
        for col,label in enumerate(['Face avant','Face corrigée','Gauche','Droite','Dos']):draw.text((col*160+5,3),label,font=font,fill='#d2e9da')
        label=['Base','Légère','Intermédiaire','Lourde','Très lourde'][tier]
        draw.text((5,17),'Traqueuse · '+label,font=font,fill='#d2dde1')
        summary.paste(im,(0,tier*192))
        if tier in [0,3]:proof.paste(im,(0,0 if tier==0 else 192))
    summary.save(OUT/'all-five-native.png'); proof.save(OUT/'native-proof.png')
    report={'review':'PLAYER-MOTION-FIX01','status':'native_validation_passed','player_cases':14,'directions_per_case':4,
            'distinct_drawings_per_direction':6,'same_player_exposure_checks':364,'traqueuse_outfit_palette_consistency_checks':5,
            'restored_saved_player_vertical_facings':vertical,'biped_frozen_source_hashes':frozen,'monster_vertical_redrawing':'requested; separate visual review pending','step_ms':400,'source_art_changed':False}
    (OUT/'native-validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8'); print(json.dumps(report))


if __name__=='__main__':main()
