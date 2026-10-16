"""ANIM-REACTION01: reuse the drawings and capture pipeline, retain every old review."""
from __future__ import annotations
import argparse
import hashlib
import json
import types
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_armoured_possessed_actions as prior

ROOT = prior.ROOT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-43'


def protected():
    hashes = prior.protected()
    for p in prior.OUT.rglob('*'):
        if p.is_file():
            hashes[p.relative_to(ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def engine():
    source = Path(prior.__file__).read_text(encoding='utf-8')
    source = source.replace("OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-42'",
                            "OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-43'")
    source = source.replace('ANIM-S16', 'ANIM-REACTION01').replace('-A01', '-A02')
    source = source.replace('src/agent_skill_lab_review/fire_painted/armoured_possessed_actions.rs',
                            'src/agent_skill_lab_review/fire_painted/armoured_possessed_reactions.rs')
    module = types.ModuleType('reaction_package')
    module.__file__ = str(OUT / 'adapted-package.py')
    exec(compile(source, module.__file__, 'exec'), module.__dict__)
    module.protected = protected
    return module, source


def save_common_animation(path, images):
    # A 180 ms out-and-back has only 4-5 motion poses at 24 fps. Repetition
    # must not be counted as new poses or lengthened to satisfy VFX checks.
    samples=[images[0].resize((256,round(images[0].height*256/images[0].width)),Image.Resampling.NEAREST)]
    for i in [0,18,19,20,21,49,50,51,52]:
        for row in range(images[0].height//370):
            for x in [140,428]:samples.append(images[i].crop((x-48,150+row*370,x+48,278+row*370)))
    sheet=Image.new('RGB',(256,sum(p.height for p in samples)));y=0
    for sample in samples:sheet.paste(sample,(0,y));y+=sample.height
    colours=list(sheet.quantize(colors=126,method=Image.Quantize.MEDIANCUT).getpalette()[:378])+[224,231,231,165,185,197]
    pixels=[p for sample in samples[1:] for p in sample.get_flattened_data()]
    for selected in ([p for p in pixels if p[1]>p[0]+25 and p[1]>p[2]+25],
                     [p for p in pixels if p[2]>p[1]+20 and p[0]>p[1]+15]):
        assert selected
        strip=Image.new('RGB',(256,(len(selected)+255)//256));strip.putdata(selected)
        colours+=strip.quantize(colors=64,method=Image.Quantize.MEDIANCUT).getpalette()[:192]
    palette=Image.new('P',(1,1));palette.putpalette(colours)
    encoded=[im.quantize(palette=palette,dither=Image.Dither.NONE) for im in images]
    bounds=[round(i*1000/24/10)*10 for i in range(len(images)+1)]
    encoded[0].save(path,save_all=True,append_images=encoded[1:],duration=[b-a for a,b in zip(bounds,bounds[1:])],loop=0,optimize=False,disposal=2)
    with Image.open(path) as gif:
        elapsed=0;unique=set()
        for i in range(gif.n_frames):
            gif.seek(i);elapsed+=gif.info.get('duration',0);unique.add(hashlib.sha256(gif.convert('RGB').tobytes()).hexdigest())
        assert elapsed==4000 and len(unique)>=4
        return {'file':path.name,'native_size':list(gif.size),'duration_ms':elapsed,'distinct_frames':len(unique),'palette':'128 neutral/background/font,64 green body,64 purple body; native actor samples retained'}


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists()
    prior.base.dump(OUT / 'review-protection.json', protected())
    for name in ['closed-64.png', 'closed-source.png', 'plates-atlas.png', 'plates-source.png',
                 'bolt-atlas.png', 'bolt-source.png', 'pulse-atlas.png', 'pulse-source.png',
                 'drawing-pack.json', 'imagegen-closed-prompt.txt', 'imagegen-plates-prompt.txt',
                 'imagegen-bolt-prompt.txt', 'imagegen-pulse-prompt.txt']:
        (OUT / name).write_bytes((prior.OUT / name).read_bytes())
    launcher = (prior.OUT / 'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launcher.replace('--lot-cuirasse-possede-actions',
        '--lot-cuirasse-possede-reactions'), encoding='utf-8-sig')
    (OUT / 'verify-gallery.cjs').write_bytes((prior.OUT / 'verify-gallery.cjs').read_bytes())
    (OUT / 'verify-links.py').write_bytes((prior.OUT / 'verify-links.py').read_bytes())
    (OUT / 'verify-gif-palette.py').write_text((prior.OUT / 'verify-gif-palette.py')
        .read_text(encoding='utf-8').replace('-A01', '-A02'), encoding='utf-8')
    module, source = engine()
    (OUT / 'adapted-package.py').write_text(source, encoding='utf-8')
    print(json.dumps({'prepared': str(OUT), 'drawings_reused_without_edits': 8}))


def package():
    module, source = engine()
    (OUT / 'adapted-package.py').write_text(source, encoding='utf-8')
    module.package()
    finish_common()


def finish_common():
    module,_=engine()
    main_entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    candidates=sorted([v for e in main_entries if e['case']=='HAUT' for v in e['views']],key=lambda v:v['number'])
    module.write_html(main_entries,candidates)
    (OUT/'verify-gallery.cjs').write_bytes((prior.OUT/'verify-gallery.cjs').read_bytes())
    entries = json.loads((OUT / 'native/common-capture.json').read_text(encoding='utf-8'))
    common_views, report, pairs = [], [], []
    for entry in entries:
        folder = OUT / 'native' / entry['folder']
        timeline = json.loads((folder / 'timeline.json').read_text(encoding='utf-8'))
        assert timeline['logical_positions'] == [[4, 4], [4, 4]]
        pair = []
        for side, who in enumerate(['Monstre', 'Joueur']):
            identifier = f"RX01-{'DMG' if entry['folder']=='COMMON-DAMAGE' else 'STATUS'}-{'M' if side==0 else 'J'}"
            variant = identifier + '-R01'
            frames = []
            dest = OUT / 'frames' / variant
            dest.mkdir(parents=True, exist_ok=True)
            x = 32 + side * 616
            for i in range(96):
                with Image.open(folder / f'{i:03}.png') as im:
                    frame = im.crop((x + 160, 248, x + 416, 504)).convert('RGB')
                frame.save(dest / f'{i:03}.png', compress_level=1)
                frames.append(frame)
            for i in entry['controls']:
                with Image.open(folder / f'{i:03}.png') as im, Image.open(folder / f'control-{i:03}.png') as control:
                    delta = ImageChops.difference(im.convert('RGB'), control.convert('RGB')).crop((x, 88, x + 576, 664))
                for cy, row in enumerate(entry['map'].splitlines()):
                    for cx, symbol in enumerate(row):
                        if symbol == '#':
                            assert delta.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None
                if i in [14,17,23,27,30,35,47,48,65,66,70,72,83,84,89,95]:
                    assert delta.getbbox() is None, (variant, i)
                if i == 19:
                    assert delta.getbbox() is not None, (variant, i)
            # The existing NPC renderer rounds very small subpixel offsets.
            # Check both visible contacts rather than requiring a 0.1 px tail.
            assert any(ImageChops.difference(frames[i], frames[17]).getbbox() is not None for i in [18,19,20,21])
            assert any(ImageChops.difference(frames[i], frames[48]).getbbox() is not None for i in [49,50,51,52])
            common_views.append({'id': identifier, 'variant': variant, 'who': who, 'label': entry['label']})
            pair.append(frames)
            report.append({'variant': variant, 'logical_position_unchanged': True,
                           'two_contacts_visible': True, 'returns_to_rest': True, 'walls_clear': True})
        boards = []
        for i in range(96):
            board = Image.new('RGB',(576,370),'#0b1218'); draw=ImageDraw.Draw(board)
            for side, who in enumerate(['Monstre', 'Joueur']):
                x = 12 + side * 288
                draw.text((x,10),who,font=prior.base.font(20,True),fill='#e0e7e7')
                draw.text((x,43),entry['label'],font=prior.base.font(14),fill='#a5b9c5')
                board.paste(pair[side][i],(x,80))
            boards.append(board)
        save_common_animation(OUT / f"{entry['folder'].lower()}-64.gif", boards)
        pairs.append(boards)
    combined=[]
    for i in range(96):
        image=Image.new('RGB',(576,740),'#0b1218')
        image.paste(pairs[0][i],(0,0));image.paste(pairs[1][i],(0,370));combined.append(image)
    common_gif=save_common_animation(OUT / 'reaction-commune-64.gif',combined)
    script=(OUT/'gallery-script.js').read_text(encoding='utf-8')
    script=script.replace('const manifest=', 'const commonViews='+json.dumps(common_views)+';const manifest=',1)
    script=script.replace('entries().flatMap(e=>e.views).map(', 'entries().flatMap(e=>e.views).concat(commonViews).map(')
    (OUT/'gallery-script.js').write_text(script,encoding='utf-8')
    html=(OUT/'index.html').read_text(encoding='utf-8')
    old_script=html[html.index('<script>')+8:html.index('</script>')]
    html=html.replace(old_script,script)
    cards=''.join(f'<article><h2>{v["who"]} · {v["label"]}</h2><canvas data-skill="{v["id"]}" width="256" height="256"></canvas></article>' for v in common_views)
    addition='<h1>Réaction commune aux compétences</h1><p>Le dessin du personnage touché fait un bref aller-retour, puis retrouve sa position. Ces quatre vues isolent la réaction pour les dégâts et un effet négatif sans dégâts, sur le monstre et le joueur. Deux contacts sont préparés ; les touches, PV et statuts ne sont pas calculés dans ces vues.</p><div class="common">'+cards+'</div><p><a href="reaction-commune-64.gif">Aperçu animé des quatre réactions</a></p>'
    html=html.replace('<main class="pair">',addition+'<main class="pair">')
    html=html.replace('</style>','.common{display:grid;grid-template-columns:repeat(4,286px);gap:16px;margin-bottom:24px}.common canvas{width:256px;height:256px}.common h2{font-size:17px}.common article{padding:12px}@media(max-width:1296px){.common{grid-template-columns:repeat(2,286px)}}</style>')
    html=html.replace('<h1>Actions du Cuirassé et du Possédé</h1>', '<h1>Aller-retour des cibles · actions du Cuirassé et du Possédé</h1>')
    (OUT/'index.html').write_text(html,encoding='utf-8')
    verifier=(OUT/'verify-gallery.cjs').read_text(encoding='utf-8')
    marker='const canvases=ids.map'
    verifier=verifier.replace(marker,'ids.push('+','.join(json.dumps(v['id']) for v in common_views)+');\n'+marker)
    verifier=verifier.replace('six_native_views_loaded:true,','six_native_views_loaded:true,common_reaction_views_loaded:true,native_canvas_count:ids.length,')
    (OUT/'verify-gallery.cjs').write_text(verifier,encoding='utf-8')
    review=json.loads((OUT/'review.json').read_text(encoding='utf-8'))
    review['excluded_trials']=['native-before-wall-mask/**','native-before-control-mask/**','control-correction/**','controls-before-wall-mask/**','reaction-phase-check.png','correction-difference.png']
    review['scope']='The lab captures do not resolve actual damage, HP, energy, AI, loot or saves. Shared existing in-game status/damage feedback is connected, without implementing new refonte combat abilities.'
    review['common_reaction']={'id':'ANIM-REACTION01','reuses_existing_game_recoil':True,
        'duration_seconds':.18,'peak_cell_fraction':.17,'cases':common_views,
        'scope':'Shared damage/status feedback in the game; six revised lab compositions and four prepared isolated cases. No newly implemented refonte combat abilities.'}
    review['approval_retained']={'user_reply':"d'accord, il faut donc mettre en place cette regle :) en tout cas visuellement j'approuve ces deux compétences",'forms':['TR63-BODY01 / TR63-A01','TR64-BODY01 / TR64-A01'],'own_actions_not_implicitly_approved':True}
    prior.base.dump(OUT/'review.json',review)
    prior.base.dump(OUT/'common-reaction-verification.json',{'cases':report,'combined_animation':common_gif})
    for name in ['combat_visuals.rs','combat_visuals_app.rs']:
        (OUT/name).write_bytes((ROOT/'src'/name).read_bytes())
    (OUT/'package-entry-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'common_reaction_views':len(report),'all_native_views':64,'animations':22}))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--prepare',action='store_true');parser.add_argument('--convert-stream',action='store_true');parser.add_argument('--finish-common',action='store_true')
    args=parser.parse_args()
    if args.prepare: prepare()
    elif args.convert_stream: engine()[0].convert_stream()
    elif args.finish_common: finish_common()
    else: package()
