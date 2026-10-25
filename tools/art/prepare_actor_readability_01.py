"""Package native assets for a browser-only readability review; no game changes."""
import json
import re
import shutil
from collections import defaultdict
from PIL import Image
from prepare_class_readability_review_01 import ROOT,sha,write_json,core_box

ART=ROOT/'assets/source-art/actor-readability-01'
OUT=ROOT/'artifacts/actor-readability-01'
DOC=ROOT/'docs/reviews/actor-readability-01'


def bestiary_scope():
    folder=ROOT/'assets/sprites/campaign-monsters'
    catalog=json.loads((folder/'manifest.json').read_text('utf-8'))
    groups=defaultdict(list)
    for entry in catalog['entries']:
        groups[entry['number']].append(entry)
    original={(int(n),int(v)) for n,v in re.findall(r'MonsterArt \{ number: (\d+), variant: (\d+)',(folder/'catalog.rs').read_text('utf-8'))}
    assert original=={(e['number'],e['variant']) for e in catalog['entries']}
    pairs=set(original)
    for name in ['material-palettes.rs','blue-material-palettes.rs']:
        pairs.update((int(n),int(v)) for n,v in re.findall(r'MaterialPalette \{ number: (\d+), variant: (\d+)',(folder/name).read_text('utf-8')))
    assert {n for n,_ in pairs}==set(groups)
    entries=[]
    for number,versions in sorted(groups.items()):
        first=versions[0]
        entries.append({'number':number,'label':first['display_name'],'family':first['section'],
            'role':first['role'],'native_canvas_side':first['side'],'foot':first['foot_anchor'],
            'variants':sorted(v for n,v in pairs if n==number),
            'shape_status':'pilot_selected' if number in {7,8,10} else 'to_review',
            'source_entries':[e['tile'] for e in versions]})
    scope={'review_id':'ACTOR-READABILITY01-BESTIARY02','request':'Apply readability direction to all monsters, including variants',
        'catalog_source':'assets/sprites/campaign-monsters/manifest.json','catalog_sha256':sha(folder/'manifest.json'),
        'campaign_models':len(groups),'campaign_appearances_including_material_palettes':len(pairs),
        'selected_shape_pilots':3,'remaining_campaign_models_for_shape_review':len(groups)-3,
        'global_render_requirements':['brighter actor silhouettes','optional ground markers','optional selective grayscale with colored markers and animated effects'],
        'removed_numbers_excluded':catalog['removed_numbers'],'deferred_bosses_not_reintroduced':catalog['deferred_boss_numbers'],
        'entries':entries,'runtime_integration':False,
        'scope_note':'Inventory of current campaign catalog; no new species or revived removed models. Other actor render paths must share global treatments when integrating.'}
    write_json(DOC/'bestiary-scope-02.json',scope)
    return {key:scope[key] for key in ['campaign_models','campaign_appearances_including_material_palettes','selected_shape_pilots','remaining_campaign_models_for_shape_review']}


def main():
    for p in [ART,OUT,DOC,OUT/'players',OUT/'monsters',OUT/'fx']:
        p.mkdir(parents=True,exist_ok=True)
    plan=json.loads((DOC/'prompts.json').read_text('utf-8'))
    backgrounds=[
        {'id':'street','name':'Rue sombre','source':'assets/source-art/first-sector-review-01/u01-style-r07/references/street-native-lighting.png','crop':[0,128,320,384]},
        {'id':'camp','name':'Sol clair du camp','source':'artifacts/class-armor-integration-01/world-bastion/cold-start.png','crop':[673,335,737,399]},
    ]
    for bg in backgrounds:
        source=ROOT/bg['source'];image=Image.open(source).convert('RGBA').crop(bg['crop'])
        if bg['id']=='camp':
            tiled=Image.new('RGBA',(320,256))
            for y in range(0,256,64):
                for x in range(0,320,64):tiled.paste(image,(x,y))
            image=tiled
        image.save(OUT/f"{bg['id']}.png")
        bg['path']=f"{bg['id']}.png";bg['source_sha256']=sha(source)
    players=[]
    capture=json.loads((ROOT/'artifacts/class-armor-integration-01/compositor/capture.json').read_text('utf-8'))
    assert not capture['exported_pngs_need_vertical_flip']
    cell=capture['configurations'].index('weapon-038')
    for index in range(25):
        source=ROOT/f'artifacts/class-armor-integration-01/compositor/outfit-{index:02}.png'
        im=Image.open(source).convert('RGBA').crop((cell%16*128,cell//16*128,cell%16*128+128,cell//16*128+128))
        im.save(OUT/f'players/{index:02}.png')
        players.append({'index':index,'path':f'players/{index:02}.png','foot':[64,96],
            'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source)})
    monsters=[]
    for job in plan['jobs']:
        n=job['number'];ident=job['id'];source=ART/f'{ident}-source.png'
        if not source.exists():shutil.copyfile(job['generated'],source)
        original=ROOT/f'assets/sprites/art-finish-completion-06/monsters/{n:03}-0.png'
        old=Image.open(original).convert('RGBA');bounds=core_box(old)
        target_height=bounds[3]-bounds[1]
        new=Image.open(source).convert('RGBA');b=core_box(new)
        pad=max(2,round((b[3]-b[1])*.015))
        cropped=new.crop((max(0,b[0]-pad),max(0,b[1]-pad),min(new.width,b[2]+pad),min(new.height,b[3]+pad)))
        scale=target_height/(b[3]-b[1]);reduced=cropped.resize((round(cropped.width*scale),round(cropped.height*scale)),Image.Resampling.LANCZOS)
        rb=core_box(reduced);native=Image.new('RGBA',(128,128))
        native.alpha_composite(reduced,(round(64-(rb[0]+rb[2])/2),bounds[3]-rb[3]))
        nb=core_box(native);assert nb[3]==bounds[3] and abs((nb[3]-nb[1])-target_height)<=1
        assert 4<=nb[0]<nb[2]<=124 and nb[1]>=4
        native.save(ART/f'{ident}-native128.png')
        shutil.copyfile(original,OUT/f'monsters/{n:03}-original.png')
        native.save(OUT/f'monsters/{n:03}-candidate.png')
        monsters.append({'id':ident,'number':n,'label':{7:'Possédé blindé',8:'Fendeur',10:'Possédé tireur'}[n],
            'original':f'monsters/{n:03}-original.png','candidate':f'monsters/{n:03}-candidate.png',
            'foot':[64,96],'old_core_box':list(bounds),'new_core_box':list(nb),
            'original_source':original.relative_to(ROOT).as_posix(),'original_sha256':sha(original),
            'new_source':source.relative_to(ROOT).as_posix(),'new_source_sha256':sha(source),
            'candidate_native':(ART/f'{ident}-native128.png').relative_to(ROOT).as_posix(),
            'candidate_native_sha256':sha(ART/f'{ident}-native128.png')})
    effects=[]
    for ident,folder,name in [
        ('flame','06','flame-atlas'),('gel-rear','27','gel-atlas'),
        ('gel-front','28','gel-atlas'),('projectile','16','projectile-atlas'),
        ('burst','20','burst-atlas'),
    ]:
        source=ROOT/f'assets/animations/agent/skill-animation-review-{folder}/{name}.png'
        target=OUT/f'fx/{ident}.png';shutil.copyfile(source,target)
        effects.append({'id':ident,'path':f'fx/{ident}.png','source':source.relative_to(ROOT).as_posix(),
            'source_sha256':sha(source),'dimensions':list(Image.open(source).size),
            'reused_existing_runtime_art':True})
    selection_path=DOC/'selection-02.json'
    selection=json.loads(selection_path.read_text('utf-8')) if selection_path.exists() else None
    manifest={'review_id':'ACTOR-READABILITY01','status':'selected_02_04_05_selective_grayscale_in_review' if selection else 'review_only_awaiting_visual_choice',
        'native_tile':64,'scene_size':[320,256],'backgrounds':backgrounds,'players':players,'monsters':monsters,
        'classes':['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe'],'effects':effects,
        'selection':selection,'bestiary_scope':bestiary_scope(),
        'game_integration':False,'engine_scene_execution':False,
        'presentation':'interactive canvas composition of actual native game assets, not a played game capture',
        'scope':'selected 02/04/05 plus optional selective grayscale study; existing spell animation art reused; no gameplay changes'}
    write_json(OUT/'manifest.json',manifest);write_json(DOC/'candidates.json',manifest)
    for name in ['actor_readability_review_01.html','actor_readability_review_01.js']:
        shutil.copyfile(ROOT/'tools/art'/name,OUT/('index.html' if name.endswith('.html') else 'review.js'))
    print(json.dumps({'players':25,'monster_studies':3,'scenes':2,'native_tile':64,'game_integration':False}))


if __name__=='__main__':main()
