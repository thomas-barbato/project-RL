"""ANIM-S15: Cuirasse/Possede native bodies and transitions, isolated from accepted reviews."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw
import prepare_agent_skill_animation_transformations as engine
import prepare_agent_skill_animation_form_attacks as previous

ROOT=engine.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-41'
base=engine.base
SOURCES={
    'cuirasse':'exec-88b5638f-c534-4e4d-90c9-893680b45645.png',
    'possede':'exec-0a19c918-0907-4409-956e-f3ecd1362f0d.png',
    'cuirasse_fx':'exec-4d013217-b010-4129-bbdc-2a6aa25ebccf.png',
    'possede_fx':'exec-2bfb1d0f-fb9c-4dc3-99d0-1bd297649a41.png',
}
GEN=Path('C:/Users/User/.codex/generated_images/01a11010-433f-7fc2-ac29-ffb696dec5ea')


def protected():
    hashes=previous.protected()
    for p in previous.OUT.rglob('*'):
        if p.is_file() and 'tint-trial' not in p.relative_to(previous.OUT).parts:
            hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def protect():
    assert not (OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json',protected())
    print(json.dumps({'protected_baseline':str(OUT)}))


def prepare():
    packs=[]
    for kind,name in SOURCES.items():
        source=GEN/name;(OUT/f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as im:
            assert im.mode=='RGBA' and im.getchannel('A').getextrema()[0]==0
            if kind.endswith('_fx'):
                assert im.width*2==im.height*3
                side=im.width//3;atlas=Image.new('RGBA',(1536,256))
                for i in range(6):
                    x,y=(i%3)*side,(i//3)*side
                    atlas.paste(im.crop((x,y,x+side,y+side)).resize((256,256),Image.Resampling.LANCZOS),(i*256,0))
                atlas.save(OUT/f'{kind}-atlas.png')
                packs.append({'kind':kind,'source_size':list(im.size),'layout':[3,2],'cell_size':[256,256]})
            else:
                bounds=im.getchannel('A').point(lambda a:255 if a>32 else 0).getbbox();assert bounds
                crop=im.crop(bounds);scale=min(62/crop.width,62/crop.height)
                w,h=round(crop.width*scale),round(crop.height*scale)
                tile=Image.new('RGBA',(64,64));tile.paste(crop.resize((w,h),Image.Resampling.LANCZOS),((64-w)//2,63-h));tile.save(OUT/f'{kind}-64.png')
                packs.append({'kind':kind,'source_size':list(im.size),'bounds':bounds,'native_size':[64,64],'direct_from_high_resolution':True,'intermediate_32px':False})
    base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen','packing':packs,'postprocessing':'Alpha-bounds crop and direct uniform64px resizing for bodies; equal-cell crop/uniform atlas packing for transitions. No Python repainting.'})
    # Freeze a separate renderer to preserve the already approved engine and its old assets.
    src=(ROOT/'src/agent_skill_lab_review/fire_painted/transformations.rs').read_text(encoding='utf-8')
    replacements=[('ANIM-S13','ANIM-S15'),('"M01", "TR61"','"TR63", "TR64"'),('Mue prédatrice','Cuirassé'),('Colosse biomécanique','Possédé'),('skill-animation-review-39','skill-animation-review-41'),('predator','cuirasse'),('colossus','possede'),('27 +','35 +'),('158.0, 168.0','170.0, 178.0'),('176.0, 184.0','158.0, 180.0')]
    for old,new in replacements:
        assert old in src,old
        src=src.replace(old,new)
    src=src.split('#[cfg(test)]')[0]+'''#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_catalogue_forms_activate_for_twenty_two_turns_and_renew_without_a_human_flash() {
        for skill in 0..2 {
            let mut s=State::new(skill,0,2,false);s.launch();
            assert!(s.active);assert_eq!((s.remaining,s.cooldown),(22,10));
            for _ in 0..10 {s.advance();}s.clock=2.0;s.launch();
            assert_eq!(s.remaining,22);assert_eq!(s.form_opacity(),1.0);assert!(s.renewal);
        }
    }
}
'''
    (ROOT/'src/agent_skill_lab_review/fire_painted/more_forms.rs').write_text(src,encoding='utf-8')
    launcher=(engine.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-transformations','--lot-transformations-suite')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launcher,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'native_bodies':2,'effect_atlases':2}))


def save_animation(path,images,fps):
    width,height=images[0].size
    centres=[(288,288)] if width==576 else [(300,354),(916,354)]
    samples=[images[0].resize((256,round(height*256/width)),Image.Resampling.NEAREST)]
    for frame in (14,19,21,23,25,35,119,174):
        for x,y in centres:samples.append(images[frame].crop((x-112,y-144,x+112,y+64)))
    sheet=Image.new('RGB',(256,sum(p.height for p in samples)));y=0
    for sample in samples:sheet.paste(sample,(0,y));y+=sample.height
    colours=list(sheet.quantize(colors=64,method=Image.Quantize.MEDIANCUT).getpalette()[:192])
    pixels=[p for im in samples[1:] for p in im.get_flattened_data()]
    for selected in ([p for p in pixels if p[1]>p[0]+30 and p[1]>p[2]+30],
                     [p for p in pixels if p[0]>p[1]+30 and p[0]>p[2]+20],
                     [p for p in pixels if p[2]>p[1]+25 and p[0]>p[1]+20]):
        if not selected:selected=pixels
        strip=Image.new('RGB',(256,(len(selected)+255)//256));strip.putdata(selected)
        colours+=strip.quantize(colors=64,method=Image.Quantize.MEDIANCUT).getpalette()[:192]
    palette=Image.new('P',(1,1));palette.putpalette(colours)
    quantized=[im.quantize(palette=palette,dither=Image.Dither.NONE) for im in images]
    bounds=[round(i*1000/fps/10)*10 for i in range(len(images)+1)]
    quantized[0].save(path,save_all=True,append_images=quantized[1:],duration=[b-a for a,b in zip(bounds,bounds[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path) as check:
        elapsed=0;unique=set();peak=None
        for i in range(check.n_frames):
            check.seek(i);im=check.convert('RGB');unique.add(hashlib.sha256(im.tobytes()).hexdigest())
            if elapsed<=bounds[21]<elapsed+check.info['duration']:peak=im.copy()
            elapsed+=check.info['duration']
        assert elapsed==bounds[-1] and len(unique)>12 and peak is not None
        errors=[]
        for x,y in centres:
            box=(x-112,y-144,x+112,y+64)
            for native,encoded in zip(images[21].crop(box).get_flattened_data(),peak.crop(box).get_flattened_data()):
                if native[1]>native[0]+40 and native[1]>native[2]+40 or native[2]>native[1]+40 and native[0]>native[1]+25:
                    errors.append(max(abs(a-b) for a,b in zip(native,encoded)))
        assert errors
        mean=sum(errors)/len(errors);p95=sorted(errors)[int((len(errors)-1)*.95)]
        assert mean<18 and p95<45,(path.name,mean,p95)
        return {'file':path.name,'native_size':[width,height],'duration_ms':elapsed,'distinct_frames':len(unique),'frames':check.n_frames,'loop':0,'palette':'64 background +64 green +64 red +64 violet, common across frames','peak_colour_mean_max_channel_error':round(mean,2),'peak_colour_p95_max_channel_error':p95}


def adapted_package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'));assert protected()==baseline
    source=Path(engine.__file__).read_text(encoding='utf-8')
    # Execute an isolated helper copy with its names/paths updated, never modify accepted helper files.
    replacements=[('skill-animation-review-39','skill-animation-review-41'),('ANIM-S13','ANIM-S15'),('Mue prédatrice','Cuirassé'),('Colosse biomécanique','Possédé'),('Prédateur 62','Cuirassé 63'),('Colosse 61','Possédé 64'),('reference62','reference63'),('référence62','référence63'),('référence61','référence64'),('Prédateur62','Cuirassé63'),('Colosse61','Possédé64'),('27 ·','35 ·'),('28 ·','36 ·'),('M01','TR63'),('TR61','TR64'),('predator','cuirasse'),('colossus','possede'),('transformations.rs','more_forms.rs')]
    # Keep module imports/base aliases unchanged; only replace the literal drawing/skill strings.
    for old,new in replacements: source=source.replace(old,new)
    source=source.split("if __name__ == '__main__':")[0]
    namespace={'__name__':'adapted_review41','__file__':str(Path(engine.__file__))}
    exec(compile(source,str(Path(engine.__file__)), 'exec'),namespace)
    namespace['OUT']=OUT;namespace['protected']=protected;namespace['save_animation']=save_animation
    namespace['package']()
    body_board=Image.new('RGB',(600,375),'#0b1218');d=ImageDraw.Draw(body_board)
    for j,(kind,title,sid) in enumerate([('cuirasse','35 · Cuirassé 63','TR63'),('possede','36 · Possédé 64','TR64')]):
        with Image.open(OUT/f'{kind}-64.png') as im:
            x=24+300*j;d.text((x,15),title,font=base.font(18,True),fill='#e0e7e7')
            zoom=im.resize((192,192),Image.Resampling.NEAREST);body_board.paste(zoom,(x,52),zoom);body_board.paste(im,(x+64,278),im)
            d.text((x,351),sid+'-BODY01 · natif64 px',font=base.font(14),fill='#a5b9c5')
    body_board.save(OUT/'bodies-native.png')
    base.dump(OUT/'review.json',{'batch':'ANIM-S15','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'id':sid,'name':title,'variant':sid+'-A01','body_variant':sid+'-BODY01','existing_body_reference':ref,'visual_status':'awaiting_visual_review','body_status':'awaiting_visual_review'} for n,sid,title,ref in [(35,'TR63','Cuirassé',63),(36,'TR64','Possédé',64)]],
        'visuals':'Emerald/ivory chitin plates assemble around Cuirasse; crimson ribbons with visible purple edges accompany Possede. New direct64px bodies preserve reference63/64 identities. Entry0.38s, return0.30s.',
        'rules_retained':'Both forms last22 turns in the previously approved rank11 example, cooldown10. Transformation active at launch; renewal after cooldown resets22 without human flash. Previously agreed17/19 energy costs retained; fixture does not resolve energy or HP.',
        'scope':'Only appearance and entry/return/renewal. No own attacks yet, real combat, HP/stats, reserve, healing, AI, loot, inventory or saves. No campaign integration or existing bank replacement.'})
    html=(OUT/'index.html').read_text(encoding='utf-8').replace('Transformations · Prédateur et Colosse','Transformations · Cuirassé et Possédé')
    html=html.replace('Les corps et leurs deux transitions attendent leur validation propre.','Les corps et leurs transitions attendent leur validation propre.')
    html=html.replace('<a href="imagegen-cuirasse_fx-prompt.txt">prompt</a>','<a href="imagegen-cuirasse-fx-prompt.txt">prompt</a>')
    html=html.replace('<a href="imagegen-possede_fx-prompt.txt">prompt</a>','<a href="imagegen-possede-fx-prompt.txt">prompt</a>')
    html=html.replace('Transition mutagénique','Plaques du Cuirassé').replace('Transition du Colosse','Possession')
    html=html.replace('</p><script>', ' · <a href="imagegen-possede-cleanup-prompt.txt">Retouche locale du crâne</a>.</p><script>')
    (OUT/'index.html').write_text(html,encoding='utf-8')
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    (OUT/'adapted-engine-source.py').write_text(source,encoding='utf-8')
    print(json.dumps({'review':'ANIM-S15','native_views':14,'animations':9,'protected_files':len(baseline)}))


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--protect-only',action='store_true');parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    if args.protect_only:protect()
    elif args.prepare:prepare()
    else:adapted_package()
