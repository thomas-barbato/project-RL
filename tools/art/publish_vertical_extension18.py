"""Publish the native C06 catalogue, preserving every accepted source file."""
from pathlib import Path
import hashlib,json,re,shutil
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18/extension-C06'
OUT=ROOT/'artifacts/vertical-walk-review-18/current/extension-C06'

def read(p):return json.loads(Path(p).read_text('utf8'))
def write(p,v):Path(p).write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n','utf8')

def main():
    decision=ROOT/'docs/reviews/vertical-walk-18/acceptance-C06.json'
    approval=read(decision) if decision.exists() else {}
    accepted=approval.get('status','').startswith('visual_accepted')
    normal=bool(approval.get('normal_game_enabled',False))
    if accepted:
        for name,digest in approval['accepted_candidate_hashes'].items():
            assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest,name
    weapons=read(ART/'conditioned-weapons/recipes.json');poses=read(ART/'conditioned-weapons/manifest.json')
    monsters=read(ART/'conditioned-monsters/manifest.json');colors=read(ART/'conditioned-monsters/palettes/manifest.json')
    assert len(monsters['profiles'])==129 and len(colors['entries'])==471
    assert not weapons['checks']['contact_failures']
    # Fill display labels from current localized content, keeping stable model ids.
    text=(ROOT/'src/game_art_catalog.rs').read_text('utf8').split('pub(super) fn held_weapon')[1].split('pub(super) fn weapon_hold')[0]
    models={key:int(n) for key,n in re.findall(r'"([^"]+)"\s*=>\s*(\d+)',text)};locale={};names={}
    for p in (ROOT/'content/core/locales').glob('fr*.json5'):locale.update(dict(re.findall(r'"([^"]+)"\s*:\s*"([^"\n]+)"',p.read_text('utf8'))))
    for p in (ROOT/'content/core/weapons').glob('*.json5'):
        text=p.read_text('utf8');ident=re.search(r'\bid"?\s*:\s*"([^"]+)"',text);name=re.search(r'\bname_key"?\s*:\s*"([^"]+)"',text)
        if ident and name and ident[1] in models:names[models[ident[1]]]=locale.get(name[1],ident[1].removeprefix('core:'))
    for p in weapons['profiles']:
        for c in p['cases']:c['name']=names.get(c['model'],c['name'])
    # Durable preparation inputs for the isolated engine: never source from artifacts.
    durable=ART/'conditioned-weapons/composites-original';durable.mkdir(exist_ok=True)
    for c in weapons['engine_cases']:
        source=ROOT/c['file'];dest=durable/source.name
        if source.resolve()!=dest.resolve() and not accepted:shutil.copy2(source,dest)
        c['file']=dest.relative_to(ROOT).as_posix()
    if not accepted:write(ART/'conditioned-weapons/recipes.json',weapons)
    cases=[]
    for c in weapons['engine_cases']:
        row=dict(c);row['save']=row['id'].startswith('arcaniste-W') or row['model'] in [2,66,88,104];cases.append(row)
    for c in colors['entries']:
        p=next(p for p in monsters['profiles'] if p['number']==c['number'])
        cases.append({'id':f'MON-{c["number"]:03}-v{c["variant"]}','kind':'monster','file':str(c['file']).replace('\\','/'),'side':c['side'],'foot':p['foot'],'save':c['variant']==0})
    extra=ART/'extra-engine-cases.json'
    if extra.exists():cases+=read(extra)
    if not accepted:write(ART/'engine-cases.json',cases)
    # The working manifest's old references are not runtime resource references.
    poses['weapon_references']=weapons['bindings']
    if not accepted:write(ART/'conditioned-weapons/manifest.json',poses)
    published={}
    def resource(path):
        path=str(path).replace('\\','/')
        if path in published:return published[path]
        source=ROOT/path;rel=Path('resources')/source.relative_to(ROOT/'assets');dest=OUT/rel;dest.parent.mkdir(parents=True,exist_ok=True)
        shutil.copy2(source,dest);published[path]=rel.as_posix();return rel.as_posix()
    for p in poses['profiles']:
        for family in p['family_poses'].values():
            for key in ['body','hands']:family[key]=resource(family[key])
    for b in weapons['bindings']:
        for c in b['palettes']:c['file']=resource(c['file'])
    for key in ['front','back','opening','ownership']:weapons['headgear'][key]=resource(weapons['headgear'][key])
    for e in colors['entries']:
        e['file']=resource(e['file']);p=next(p for p in monsters['profiles'] if p['number']==e['number']);e['foot']=p['foot'];e['offset']=p['front_vertical_offset'];e['shadow']=p['shadow']
    preview={'review':'VERT18-C06','native_tile_px':64,'normal_game_enabled':normal,'weapons':weapons,'poses':poses,'monsters':colors['entries'],'geometry':read(OUT/'monster-geometry-audit.json')}
    # Do not ship hundreds of unused engine paths in the browser data.
    preview['weapons'].pop('engine_cases',None)
    write(OUT/'catalogue.json',preview)
    for name in ['html','js','css']:
        source=ROOT/f'tools/art/vertical_extension18.{name}';shutil.copy2(source,OUT/('apercu.html' if name=='html' else 'app.'+name))
    if accepted:
        page=OUT/'apercu.html'
        page.write_text(page.read_text('utf8').replace('PROPOSITION À VALIDER','VALIDÉ · INTÉGRÉ EN JEU' if normal else 'VALIDÉ'), 'utf8')
    print(json.dumps({'published_resources':len(published),'weapon_models':113,'weapon_colors':7,'binding_cases':1216,'monster_palette_pairs':471,'engine_cases':len(cases),'normal_game_enabled':normal}))

if __name__=='__main__':main()
