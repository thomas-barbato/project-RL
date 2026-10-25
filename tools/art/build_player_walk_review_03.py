"""Build the complete, review-only movement catalogue from real runtime sources.

No generated output is an input to the game. Run the ignored Rust layer exporter
first; this script checks the complete manifests and packages the review Site.
"""
from __future__ import annotations
import argparse
import base64
import hashlib
import json
import re
from collections import Counter
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'artifacts/player-walk-review-03'
DOC = ROOT / 'docs/reviews/player-walk-03'
SPECIAL = {(164,0):'FIN-06',(165,0):'FIN-07',(5,0):'FIN-08',
    (144,0):'FIN-09',(144,1):'FIN-09-blue',(144,2):'FIN-09-orange',(144,3):'FIN-09-green',
    (145,0):'FIN-10',(145,2):'FIN-10-orange',(145,3):'FIN-10-green',
    (14,0):'FIN-28',(16,0):'FIN-29',(6,0):'FIN-30',(120,0):'FIN-31'}
CLASS_NAMES = {'bastion':'Bastion','technomancien':'Technomancien','traqueuse':'Traqueuse','arcaniste':'Arcaniste','metamorphe':'Métamorphe'}
RIG_LABELS = {'biped':'Marche bipède','heavy':'Marche lourde','robe':'Pas sous une robe',
    'quad':'Appuis de quadrupède','crawl':'Pattes multiples','serpent':'Ondulation',
    'float':'Flottement','winged':'Appuis et ailes','machine':'Châssis mécanique','roll':'Roulement / chenilles'}

def load(path):
    return json.loads((ROOT / path).read_text('utf-8'))

def campaign_rig(n):
    if n in [29,33,65,140,141,142]: return 'serpent'
    if n in [124,125,126,127,139]: return 'float'
    if n in [39,66,88,144,145,146,147,148,149,150,151,153,157,162]: return 'winged'
    if n in [21,30,31,37,38,67,69,70,71,83,89,101,107,113,143,155,158]: return 'crawl'
    if n in [9,20,35,63,64,75,76,81,87,90,92,103,104,109,118,122,131,154]: return 'quad'
    if n in [14,15,16,17,22,24,116,117,132,133,134,135,164,165]: return 'robe'
    if n in [7,9,28,68,72,77,78,80,84,93,98,99,102,123,138,152,156,159]: return 'heavy'
    return 'biped'

def legacy_rig(e):
    number=e['number']
    groups={
        'serpent':[39,45,48], 'float':[5,8,32,46,56], 'winged':[28,38],
        'quad':[3,20,33,34,35,36,58], 'crawl':[24,37,40,41,43,44,47],
        'machine':[21,22,27,42], 'roll':[31,57,59], 'heavy':[1,25,29,30,61,63,66],
        'robe':[18,19,26],
    }
    for key,values in groups.items():
        if number in values:return key
    if 1<=number<=66:return 'biped'
    text=(e['name']+' '+e['family']).lower()
    if any(s in text for s in ['spectr','fantôm','fantom','esprit','flott','sonde','drone']):return 'float'
    if any(s in text for s in ['serpent','ver ','larve','limace']):return 'serpent'
    if any(s in text for s in ['araign','scorpion','scarab','crabe','insect','mille']):return 'crawl'
    if any(s in text for s in ['chien','loup','rat','félin','bête','meute','quadrup']):return 'quad'
    if any(s in text for s in ['ailé','chauve','volant']):return 'winged'
    if any(s in text for s in ['automate','machine','robot','tourelle','sentinelle']):return 'machine'
    if any(s in text for s in ['cultiste','mage']):return 'robe'
    if e['tile_size']==64:return 'heavy'
    return 'biped'

class Builder:
    def __init__(self, out):
        self.out=out
        self.assets=out/'catalogue/assets'
        self.assets.mkdir(parents=True,exist_ok=True)
        self.records=[]
        self.hashes={}
        self.sources=[]
        self.previews={}
        self.paths={}

    def image(self, im, source):
        im=im.convert('RGBA')
        assert im.getbbox(), f'empty source {source}'
        digest=hashlib.sha256(im.tobytes()+str(im.size).encode()).hexdigest()
        filename=f'{digest[:20]}.png'
        path='catalogue/assets/'+filename
        if digest not in self.hashes:
            im.save(self.assets/filename,compress_level=6)
            self.hashes[digest]=path
        self.paths[path]=True
        self.sources.append({'source':source,'image_sha256':digest,'size':list(im.size),'asset':path})
        return path

    def file(self, path, flip=False):
        im=Image.open(ROOT/path).convert('RGBA')
        if flip: im=im.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        return self.image(im,str(path)),im

    def add(self, record, im):
        box=im.getbbox()
        assert box and len(record['foot'])==2
        record['bounds']=list(box)
        record['rigLabel']=RIG_LABELS[record['rig']]
        record['reviewId']='WALK03-'+record['id']
        self.records.append(record)
        self.previews[record['id']]=im

    def classes(self):
        manifest=load('assets/sprites/class-armors-01/manifest.json')
        capture=load('artifacts/player-walk-review-03/compositor/capture.json')
        weapons=load('assets/sprites/cyberpunk/manifest.json')['held_weapons']
        names={}
        for path in (ROOT/'content/core/locales').glob('fr*.json5'):
            for key,value in re.findall(r'"(equipment\.[^"]+\.name)"\s*:\s*("(?:\\.|[^"\\])*")',path.read_text('utf-8')):
                names[key]=json.loads(value)
        configs=capture['configurations']
        for config in configs:
            if config['weapon'] is not None:
                model=config['weapon'];key=weapons[model]['id'].removeprefix('core:')
                name=names.get(f'equipment.{key}.name',key.removeprefix('modele_').replace('_',' ').title())
                config['label']=f'{model:03} · {name}'
        for e in manifest['outfits']:
            n=e['index']; paths=[];ims=[]
            for layer in range(3):
                p,im=self.file(f'artifacts/player-walk-review-03/compositor/class-{n:02}-layer-{layer}.png',True)
                paths.append(p);ims.append(im)
            core,coreim=self.file(f'artifacts/player-walk-review-03/compositor/class-{n:02}-core.png',True)
            headmask,_=self.file(f'artifacts/player-walk-review-03/compositor/class-{n:02}-head-mask.png',True)
            cname=e['class_id'].split(':')[-1]
            name=CLASS_NAMES.get(cname,cname.title())
            body=ims[0].crop((0,0,128,128))
            preview=self.image(body,f'class {n} neutral production layers')
            self.add({'id':f'CLASS-{n:02}','label':f'{name} · tenue {e["rank"]}',
                'category':'Classes','family':name,'rig':'robe' if name=='Arcaniste' else 'heavy' if name=='Bastion' else 'biped',
                'image':preview,'pack':paths,'core':core,'headmask':headmask,
                'foot':[64,96],'size':[128,128],'classIndex':n,'candidate':e['candidate_id'],
                'status':'Tenue actuelle · mouvement proposé','configs':'class',
                'note':'Sources réelles du jeu ; raccords des manches à examiner.'},body)
        return configs

    def campaign(self):
        manifest=load('assets/sprites/campaign-monsters/manifest.json')
        mappings={}
        filenames=['game_art_readability_catalog.rs','game_art_finish_catalog.rs']+[f'game_art_finish_catalog_{n:02}.rs' for n in range(2,7)]
        for filename in filenames:
            text=(ROOT/'src'/filename).read_text('utf-8')
            for key,path in re.findall(r'"([^"]+)"\s*=>\s*(?:Some\()?include_bytes!\(\s*"\.\./([^"]+)"',text):
                mappings.setdefault(key,path)
        self.finish=mappings
        entries={(e['number'],e['variant']):dict(e) for e in manifest['entries']}
        for key in ['runtime_material_palettes','runtime_blue_material_palettes']:
            for e in manifest[key]:
                base=entries[(e['number'],e['source_variant'])]
                entries[(e['number'],e['variant'])]=dict(base,variant=e['variant'],preview=e['preview'])
        for (n,v),e in sorted(entries.items()):
            path=mappings.get(f'VIS-{n:03}-{v}') or mappings.get(SPECIAL.get((n,v),f'MON-{n:03}-{v}'))
            if not path:path=e.get('preview') or 'assets/sprites/campaign-monsters/'+e['tile']
            asset,im=self.file(path)
            assert im.size==(e['side'],e['side']), (n,v,im.size,e['side'])
            reserve=e['role']=='ReservedCaster'
            self.add({'id':f'MON-{n:03}-{v}','label':f'{n:03} · {e["display_name"]} · variante {v}',
                'category':'Campagne','family':e['section'],'rig':campaign_rig(n),'image':asset,
                'foot':e['foot_anchor'],'size':list(im.size),'number':n,'variant':v,
                'status':'Visuel réservé' if reserve else 'Ressource de campagne',
                'note':'Une ressource visuelle ne crée pas de rencontre ni de nouvelle capacité.' if reserve else 'Silhouette et palette actuelles ; animation proposée selon la morphologie.'},im)
        assert len(entries)==471

    def others(self):
        camp=load('assets/maps/cryogenic-camp-01/camp.json')
        for e in camp['actors'][1:]:
            asset,im=self.file(e['file'])
            self.add({'id':e['id'],'label':e['id']+' · camp','category':'PNJ et formes','family':'Camp',
                'rig':'biped','image':asset,'foot':e['foot_anchor'],
                'size':list(im.size),'status':'Source du camp','note':'Le mouvement de revue ne change pas les déplacements ou les rôles des PNJ.'},im)
        drones=[('SCOUT','Éclaireur','97/D56-BODY01-SIZE02-native128.png','float'),
            ('TRACKER','Traqueur','96/D58-BODY01-native128.png','quad'),
            ('GUNNER','Artilleur','92/artilleur-body03/D59-BODY03-native64.png','roll'),
            ('GUARDIAN','Gardien','94/gardien-size01/D66-BODY02-SIZE01-native128.png','heavy')]
        for key,name,source,rig in drones:
            asset,im=self.file('assets/animations/agent/skill-animation-review-'+source)
            self.add({'id':'DRONE-'+key,'label':'Drone · '+name,'category':'PNJ et formes','family':'Drones','rig':rig,
                'image':asset,'foot':[im.width//2,im.height//2+32],'size':list(im.size),
                'status':'Drone actuel','note':'Dimensions et corps issus du rendu actuel.'},im)
        forms=[('PREDATOR','Prédateur','39/predator-64.png','biped'),('COLOSSUS','Colosse','39/colossus-64.png','heavy'),
            ('ARMOR','Cuirasse','41/cuirasse-64.png','heavy'),('POSSESSED','Possédé','41/possede-64.png','biped'),
            ('ABYSS','Abysse','44/abysse-64.png','biped'),('CLOSED','Forme fermée','42/closed-64.png','heavy')]
        for key,name,source,rig in forms:
            asset,im=self.file('assets/animations/agent/skill-animation-review-'+source)
            self.add({'id':'FORM-'+key,'label':'Forme · '+name,'category':'PNJ et formes','family':'Transformations','rig':rig,
                'image':asset,'foot':[32,64],'size':list(im.size),'status':'Transformation actuelle',
                'note':'Proposition de déplacement uniquement ; pouvoirs et effets conservés.'},im)

    def legacy(self):
        manifest=load('assets/sprites/bestiary/manifest.json')
        for e in manifest['entries']:
            if e['number']==57:
                asset,im=self.file(self.finish['FIN-11']);foot=[64,96]
            else:
                im=Image.open(ROOT/'assets/sprites/bestiary'/e['living']).convert('RGBA')
                # Reproduce the historical bank's *current* 2x world scaling.
                # It is not a new 32px candidate and must not reduce large bosses.
                im=im.resize((im.width*2,im.height*2),Image.Resampling.NEAREST)
                asset=self.image(im,'assets/sprites/bestiary/'+e['living']+' · current world scale x2')
                foot=[e['living_shadow_contact'][0]*2,e['living_shadow_contact'][1]*2]
            self.add({'id':f'BANK-{e["number"]:03}','label':f'Banque {e["number"]:03} · {e["name"]}',
                'category':'Héritage','family':e['family'],'rig':legacy_rig(e),'image':asset,'foot':foot,
                'size':list(im.size),'status':'Banque historique','note':'Ancienne source affichée à l’échelle actuelle sur une grille de 64 px ; aucun nouveau dessin en 32 px.'},im)
        actors=load('assets/sprites/cyberpunk/manifest.json')
        atlas=Image.open(ROOT/'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
        for n in range(actors['actor_count']):
            x=n%16*32;y=n//16*32
            im=atlas.crop((x,y,x+32,y+32)).resize((64,64),Image.Resampling.NEAREST)
            asset=self.image(im,f'assets/sprites/cyberpunk/actors-32.png actor {n}, current display scale x2')
            foot=[actors['actor_contacts'][n][0]*2,actors['actor_contacts'][n][1]*2]
            self.add({'id':f'ACTOR-{n:02}','label':f'Acteur historique {n:02}','category':'Héritage','family':'Acteurs historiques',
                'rig':legacy_rig({'number':n+30,'name':'','family':'','tile_size':32}) if 3<=n<=25 else 'biped',
                'image':asset,'foot':foot,'size':[64,64],'status':'Ancien acteur',
                'note':'Les classes actuelles et les remplacements de campagne figurent dans leurs propres catégories.'},im)
        flat=load('assets/sprites/flat-agents/manifest.json')
        extended=load('assets/sprites/equipment-native64/manifest.json')
        legacy_configs=[e['label'] for e in flat['configurations']]
        for bank,outfits,filename in [('flat-agents',flat['outfits'],'reviewed-128.png'),
            ('equipment-native64',[{'review_id':s} for s in extended['extended_outfits']],'outfits-128.png')]:
            atlas=Image.open(ROOT/'assets/sprites'/bank/filename).convert('RGBA')
            for n,e in enumerate(outfits):
                for variant in range(3):
                    cells=[]
                    for conf in range(16):
                        idx=(n*3+variant)*16+conf;x=idx%16*128;y=idx//16*128
                        cells.append(atlas.crop((x,y,x+128,y+128)))
                    sheet=Image.new('RGBA',(16*128,128))
                    for j,im in enumerate(cells):sheet.paste(im,(j*128,0))
                    pack=self.image(sheet,f'{bank}/{filename} outfit {n}, variant {variant}')
                    im=cells[0];asset=self.image(im,f'{bank} {e["review_id"]} {variant} unarmed')
                    self.add({'id':f'ARMOR-{bank}-{n:02}-{variant}',
                        'label':f'{e["review_id"]} · palette {variant}', 'category':'Héritage','family':'Anciennes armures',
                        'rig':'robe' if 'AR-09'==e['review_id'] else 'biped','image':asset,'foot':[64,96],
                        'size':[128,128],'status':'Tenue historique figée','pack':[pack], 'configs':'legacy',
                        'note':'Anciennes configurations de revue conservées ; elles ne remplacent pas les 25 tenues de classe actuelles.'},im)
        return legacy_configs

    def boards(self):
        font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',12)
        for category,cols in [('Classes',5),('PNJ et formes',5),('Campagne',8)]:
            entries=[e for e in self.records if e['category']==category and e.get('variant',0)==0]
            w=192;h=174
            board=Image.new('RGB',(cols*w,((len(entries)+cols-1)//cols)*h),'#263136')
            dr=ImageDraw.Draw(board)
            for i,e in enumerate(entries):
                im=self.previews[e['id']];x=i%cols*w;y=i//cols*h
                # Native pixels, never fit-to-bounds. Large actors receive a
                # separate full-size gallery stage rather than silent shrinking.
                if max(im.size)>150:
                    box=im.getbbox();im=im.crop(box)
                if im.width<=w and im.height<=140:
                    board.paste(im,(x+(w-im.width)//2,y+136-im.height),im)
                else:
                    dr.text((x+8,y+70),'Grand visuel : voir le catalogue',font=font,fill='#bdc6c5')
                dr.text((x+5,y+139),e['id'],font=font,fill='white')
                dr.text((x+5,y+155),e['label'][:29],font=font,fill='#b6c3c5')
            board.save(ART/(category.split()[0].lower()+'-native.png'))

    def page(self):
        template=(ROOT/'tools/art/player_walk_review_03.html').read_text('utf-8')
        template=template.replace('/*__RIG__*/',(ROOT/'tools/art/player_walk_review_03_rig.js').read_text('utf-8'))
        template=template.replace('/*__APP__*/',(ROOT/'tools/art/player_walk_review_03.js').read_text('utf-8'))
        (self.out/'catalogue/app.html').write_text(template,'utf-8')
        wrapper='''<!doctype html><html lang="fr"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; img-src data:; frame-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'none'">
<title>Project RL · tous les déplacements</title>
<style>body{margin:0;background:#172226;color:#e8e6df;font:14px system-ui,sans-serif}nav{max-width:1120px;margin:auto;padding:12px 24px;border-bottom:1px solid #3b4949;display:flex;gap:20px;flex-wrap:wrap}a{color:#efb776}iframe{display:block;border:0;width:100%;height:1800px}#error{margin:24px;color:#efb776}</style></head>
<body><nav><span>Catalogue de revue</span><a href="armes.html">Prototype peint du Bastion avec armes</a><a href="index.html">Premier comparatif</a></nav>
<p id="error" hidden></p><iframe id="review" title="Catalogue complet des propositions de déplacement" sandbox="allow-scripts"></iframe>
<script>
'use strict';
const frame=document.getElementById('review'),cache=new Map(),waiting=new Map();
const data=fetch('catalogue/data.json').then(r=>{if(!r.ok)throw Error('Catalogue indisponible');return r.json();});
const send=m=>frame.contentWindow.postMessage(m,'*');
function fail(e){const p=document.getElementById('error');p.hidden=false;p.textContent=String(e.message||e);send({kind:'walk03-error',error:p.textContent});}
data.catch(fail);
window.addEventListener('message',async e=>{
 if(e.source!==frame.contentWindow)return;const m=e.data;if(!m||typeof m!=='object')return;
 try{
  if(m.kind==='walk03-ready'){send({kind:'walk03-data',data:await data});return;}
  if(m.kind==='walk03-height'){if(Number.isFinite(m.height))frame.style.height=Math.min(30000,Math.max(400,m.height+4))+'px';return;}
  if(m.kind==='walk03-asset'){
   const d=await data;
   if(typeof m.path!=='string'||!/^catalogue\\/assets\\/[a-f0-9]{20}\\.png$/.test(m.path)||!d.assets.includes(m.path))throw Error('Source non autorisée');
   if(!waiting.has(m.path))waiting.set(m.path,(async()=>{
    const response=await fetch(m.path);if(!response.ok)throw Error('Source indisponible');
    const blob=await response.blob();if(blob.size>8000000)throw Error('Source trop volumineuse');
    return await new Promise((resolve,reject)=>{const reader=new FileReader();reader.onload=()=>resolve(reader.result);reader.onerror=reject;reader.readAsDataURL(blob);});
   })());
   send({kind:'walk03-asset-result',path:m.path,url:await waiting.get(m.path)});
  }
 }catch(err){if(m.kind==='walk03-asset')send({kind:'walk03-asset-result',path:m.path,error:String(err.message||err)});else fail(err);}
});
frame.srcdoc=/*__SRCDOC__*/;
</script></body></html>'''
        # The sandbox never makes an authenticated document request. Its own
        # origin remains opaque; only the parent fetches whitelisted assets.
        wrapper=wrapper.replace('/*__SRCDOC__*/',json.dumps(template,ensure_ascii=False).replace('</','<\\/'))
        (self.out/'catalogue.html').write_text(wrapper,'utf-8')

    def scene(self):
        floor=Image.open(ROOT/'assets/maps/cryogenic-camp-01/floor.png').convert('RGBA').crop((768,512,832,576))
        tile=self.image(floor,'assets/maps/cryogenic-camp-01/floor.png tile (12,8), native64')
        items=Image.open(ROOT/'assets/sprites/cyberpunk/items-128.png').convert('RGBA')
        item=items.crop((256,0,384,128)).resize((64,64),Image.Resampling.NEAREST)
        loot=self.image(item,'assets/sprites/cyberpunk/items-128.png illustration 2, current world64 scale')
        return {'floor':tile,'item':loot,'actors':['NPC-01','CLASS-14','MON-010-0']}

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--standalone',type=Path,help='Export an already-built catalogue as one offline HTML file')
    args=parser.parse_args()
    if args.standalone:
        data=json.loads((args.output/'catalogue/data.json').read_text('utf-8'))
        embedded={p:'data:image/png;base64,'+base64.b64encode((args.output/p).read_bytes()).decode('ascii') for p in data['assets']}
        wrapper=(args.output/'catalogue.html').read_text('utf-8')
        old="const data=fetch('catalogue/data.json').then(r=>{if(!r.ok)throw Error('Catalogue indisponible');return r.json();});"
        assert old in wrapper
        wrapper=wrapper.replace(old,'const data=Promise.resolve('+json.dumps(data,ensure_ascii=False).replace('</','<\\/')+');const embedded='+json.dumps(embedded)+';')
        wrapper,count=re.subn(r'const response=await fetch\(m\.path\);.*?reader\.readAsDataURL\(blob\);\}\);','return embedded[m.path];',wrapper,flags=re.S)
        assert count==1
        for filename in ['armes.html','index.html']:
            wrapper=wrapper.replace(f'href="{filename}"',f'href="{(args.output/filename).as_uri()}"')
        args.standalone.parent.mkdir(parents=True,exist_ok=True)
        args.standalone.write_text(wrapper,'utf-8')
        print(json.dumps({'standalone':str(args.standalone),'bytes':args.standalone.stat().st_size}))
        return
    DOC.mkdir(parents=True,exist_ok=True)
    b=Builder(args.output)
    configs=b.classes();b.campaign();b.others();legacy=b.legacy();b.boards();b.page();scene=b.scene()
    ids=[e['id'] for e in b.records];assert len(ids)==len(set(ids))
    data={'review':'WALK03-r01','tile':64,'defaultDuration':400,'durations':[120,200,400],
        'records':b.records,'classConfigs':configs,'legacyConfigs':legacy,'scene':scene,'assets':sorted(b.paths),
        'counts':dict(Counter(e['category'] for e in b.records)),
        'limits':['Profils latéraux obtenus par déformation 2D, pas par de nouveaux dessins de profil.',
            'La banque historique conserve son agrandissement actuel ; les nouveaux dessins restent en 64 px.',
            'Cet aperçu ne valide ni les rencontres, ni les pouvoirs, ni une intégration dans le jeu.']}
    (args.output/'catalogue/data.json').write_text(json.dumps(data,ensure_ascii=False,separators=(',',':')),'utf-8')
    coverage={'review':data['review'],'status':'proposal-awaiting-user-review','runtime_integration':False,
        'source_manifests':['assets/sprites/class-armors-01/manifest.json','assets/sprites/campaign-monsters/manifest.json',
            'assets/sprites/bestiary/manifest.json','assets/sprites/cyberpunk/manifest.json','assets/sprites/flat-agents/manifest.json',
            'assets/sprites/equipment-native64/manifest.json','assets/maps/cryogenic-camp-01/camp.json'],
        'counts':data['counts'],'total_entries':len(ids),'campaign_base_models':129,'campaign_variants':471,
        'class_outfits':25,'class_weapon_models':113,'class_configs':len(configs),
        'movement_profiles':dict(Counter(e['rig'] for e in b.records)),
        'review_ids':ids,'assets':b.sources,'limitations':data['limits']}
    (DOC/'coverage.json').write_text(json.dumps(coverage,ensure_ascii=False,indent=2)+'\n','utf-8')
    print(json.dumps({'entries':len(ids),'counts':data['counts'],'assets':len(b.paths),'bytes':sum(p.stat().st_size for p in b.assets.glob('*.png'))},ensure_ascii=False))

if __name__=='__main__':main()
