"""Condition imagegen subjects and assemble non-destructive native art reviews."""
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
import json,hashlib,importlib.util,shutil,math

OUT=Path(__file__).resolve().parent;ROOT=OUT.parents[1]
OLD=ROOT/'artifacts/bestiaire-proposition-2026-10-05'
spec=importlib.util.spec_from_file_location('accepted_bestiary_packaging',OLD/'package.py')
helpers=importlib.util.module_from_spec(spec);spec.loader.exec_module(helpers)
catalog=json.loads((OUT/'catalog.json').read_text(encoding='utf-8'))
current=json.loads((OUT/'revision-current.json').read_text(encoding='utf-8')) if (OUT/'revision-current.json').exists() else {}
assert all((OUT/e['source']).exists() for e in catalog if e['number'] in current.get('numbers',[])),'Current revisions are still being generated.'
assert not any(e['number'] in current.get('retired',[]) for e in catalog),'Retired model remains in current catalogue.'
previous={e['number']:e for e in json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))} if (OUT/'manifest.json').exists() else {}
descriptions={9:'Casque, respirateur et tenue de travail industrielle ; la capuche médiévale disparaît.',56:'Coque volante compacte, deux nacelles latérales et un large capteur.',57:'Châssis chenillé bas, bloc de chargement et bras à pince.',58:'Quadrupède mécanique : tête en coin et quatre pattes articulées.',59:'Machine d’atelier : trois appuis à roues et deux bras-outils.',60:'Robot humanoïde massif avec plaque de protection intégrée au bras.',61:'Corps biomécanique massif : plaques ivoire et muscles rouges.',62:'Forme élancée : tête angulaire, jambes pliées et grandes griffes.',63:'Corps cuirassé : carapace verte, ventre segmenté et deux pinces.',64:'Forme démoniaque : deux cornes, épaules angulaires et mains griffues.',65:'Forme abyssale : œil unique, cavité du torse et deux vrilles arrière.'}
F=ImageFont.truetype('C:/Windows/Fonts/consola.ttf',14);S=ImageFont.truetype('C:/Windows/Fonts/consola.ttf',11)
def stamp(canvas,im,xy):canvas.paste(im,xy,im)
player=Image.open(OUT/'references/joueur-natif.png').convert('RGBA')
for key in ('vivant','cadavre'):shutil.copyfile(OLD/f'tiles/009-{key}.png',OUT/f'references/pillard-refuse-{key}.png')
ready=[]
for entry in catalog:
 source=OUT/entry['source']
 revisions=sorted((OUT/'sources').glob(f'{entry["number"]:03}-r*.png'))
 if revisions:source=revisions[-1];entry['source']=source.relative_to(OUT).as_posix()
 if not source.exists():continue
 digest=hashlib.sha256(source.read_bytes()).hexdigest();cached=previous.get(entry['number'])
 if cached and cached.get('source_sha256')==digest and cached['source']==entry['source'] and all((OUT/cached[k]).exists() for k in ('living','corpse')):ready.append(cached|entry);continue
 if entry.get('source_kind')=='corpse-only':
  dead=Image.open(source)
  tile=Image.open(OUT/entry['living'])
  entry['living_encoding']=(cached or {}).get('living_encoding',dict(native_bounds=list(tile.getbbox()),resampling='unchanged original pixels',palette_limit=24))
  entry['living_preserved_sha256']=hashlib.sha256((OUT/entry['living']).read_bytes()).hexdigest()
 else:
  live,dead=helpers.pair_subjects(Image.open(source))
  tile,metadata=helpers.condition(live,32,entry['maximum'])
  if entry['flying']:
   positioned=Image.new('RGBA',(32,32));positioned.alpha_composite(tile,(0,-6));tile=positioned
   metadata['native_bounds']=list(tile.getbbox());metadata['contact_kind']='hover, six native pixels above ground baseline'
  else:metadata['contact_kind']='feet or supports on ground baseline'
  tile.save(OUT/entry['living']);entry['living_encoding']=metadata
 tile,metadata=helpers.condition(dead,32,(30,14));tile.save(OUT/entry['corpse']);entry['corpse_encoding']=metadata
 entry['source_sha256']=digest;entry['review_description']=entry.get('review_description',descriptions.get(entry['number'],'Épave mécanique sans sang.'))
 entry['blood_layer_status']='none; dry mechanical wreck, no blood or replacement fluid pool' if not entry['corpse_includes_blood'] else 'pool included in review composite; independent reusable layer reserved for later integration'
 ready.append(entry)
(OUT/'manifest.json').write_text(json.dumps(ready,ensure_ascii=False,indent=2),encoding='utf-8')

def board(entries,name):
 cellw=244;cellh=248;cols=min(3,len(entries));rows=math.ceil(len(entries)/cols);canvas=Image.new('RGB',(cellw*cols,68+cellh*rows),'#121b22');d=ImageDraw.Draw(canvas)
 title={'56-59-et-66-drones-r3':'DRONES DU JOUEUR / 56–59 ET 66','59-66-propositions-r3':'NOUVELLES PROPOSITIONS / 59 ET 66','61-65-transformations':'TRANSFORMATIONS / 61–65','21-24-et-42-43-epaves':'ROBOTS ENNEMIS / ÉPAVES SANS SANG'}[name]
 d.text((12,12),title,font=F,fill='#d6e9e5');d.text((12,36),'Petit = natif 32 px / grand = zoom x3 / joueur de référence à gauche',font=S,fill='#aabfc8')
 for i,e in enumerate(entries):
  x=i%cols*cellw;y=68+i//cols*cellh;d.rectangle((x+4,y+4,x+cellw-5,y+cellh-5),fill='#202d35')
  d.text((x+12,y+13),f'{e["number"]:02}  {e["name"]}',font=F,fill='#a3dfd2')
  d.text((x+12,y+37),'Actif / '+e['inactive_label'].lower(),font=S,fill='#adc1c7')
  live=Image.open(OUT/e['living']);dead=Image.open(OUT/e['corpse'])
  stamp(canvas,player,(x+14,y+67));stamp(canvas,live,(x+73,y+67));stamp(canvas,dead,(x+151,y+67))
  stamp(canvas,live.resize((96,96),Image.Resampling.NEAREST),(x+12,y+125));stamp(canvas,dead.resize((96,96),Image.Resampling.NEAREST),(x+133,y+125))
 canvas.save(OUT/f'review/{name}.png')
for usage,name in [('player-companion','56-59-et-66-drones-r3'),('player-transformation','61-65-transformations'),('enemy-machine','21-24-et-42-43-epaves')]:
 entries=[e for e in ready if e['usage']==usage]
 if entries:board(entries,name)
new_candidates=[e for e in ready if e['number'] in current.get('numbers',[])]
if new_candidates:board(new_candidates,'59-66-propositions-r3')
comparison=[e for e in new_candidates if e.get('previous_living')]
if comparison:
 canvas=Image.new('RGB',(244*len(comparison),316),'#121b22');d=ImageDraw.Draw(canvas)
 d.text((12,12),'59 / 66 : AVANT ET APRÈS',font=F,fill='#d6e9e5')
 d.text((12,36),'Petit = 32 px ; grand = zoom x3 ; ancien à gauche, nouveau à droite',font=S,fill='#aabfc8')
 for i,e in enumerate(comparison):
  x=i*244;d.rectangle((x+4,72,x+239,310),fill='#202d35')
  d.text((x+12,82),f'{e["number"]:02}  {e["name"]}',font=F,fill='#a3dfd2')
  old=Image.open(OUT/e['previous_living']);new=Image.open(OUT/e['living'])
  stamp(canvas,old,(x+44,124));stamp(canvas,new,(x+165,124))
  stamp(canvas,old.resize((96,96),Image.Resampling.NEAREST),(x+12,192));stamp(canvas,new.resize((96,96),Image.Resampling.NEAREST),(x+133,192))
 canvas.save(OUT/'review/59-66-avant-apres.png')
if any(e['number']==9 for e in ready):
 canvas=Image.new('RGB',(960,300),'#121b22');d=ImageDraw.Draw(canvas)
 d.text((14,12),'09 PILLARD / version refusée et tenue industrielle proposée',font=F,fill='#c7e9df')
 entry=next(e for e in ready if e['number']==9)
 pairs=[('Ancien vivant refusé',OUT/'references/pillard-refuse-vivant.png'),('Ancien cadavre refusé',OUT/'references/pillard-refuse-cadavre.png'),('Nouveau vivant proposé',OUT/entry['living']),('Nouveau cadavre proposé',OUT/entry['corpse'])]
 for i,(label,path) in enumerate(pairs):
  x=i*240;d.text((x+10,55),label,font=S,fill='#b7c9cf');im=Image.open(path)
  stamp(canvas,im,(x+104,87));stamp(canvas,im.resize((128,128),Image.Resampling.NEAREST),(x+55,145))
 canvas.save(OUT/'review/09-pillard.png')

terrain=Image.open(ROOT/'assets/sprites/cyberpunk/terrain-materials-32.png').convert('RGBA');floor=terrain.crop((0,0,32,32));floor.save(OUT/'review/sol-jeu.png')
scenes=[]
def scene(entries,name):
 width=960;height=130+len(entries)*86;canvas=Image.new('RGB',(width,height),'#121b22');d=ImageDraw.Draw(canvas)
 title={'scene-09':'PILLARD','scene-56-59-et-66':'DRONES DU JOUEUR','scene-61-65':'TRANSFORMATIONS','scene-epaves-ennemies':'ÉPAVES DES ROBOTS ENNEMIS'}[name]
 d.text((14,12),title+' / COMPOSITION DE REVUE NATIVE 1:1',font=F,fill='#b8e2d7')
 d.text((14,37),'Joueur actuel, modèle actif et version morte ; sols sombre / clair / jeu',font=S,fill='#a8bdc7')
 for i,e in enumerate(entries):
  y=80+i*86;d.text((12,y),f'{e["number"]:02}  {e["name"]}',font=F,fill='#b8dace')
  live=Image.open(OUT/e['living']);dead=Image.open(OUT/e['corpse'])
  for j,color in enumerate(('#12191f','#8d938e',None)):
   x=265+j*220
   if color:d.rectangle((x,y-10,x+211,y+61),fill=color)
   else:
    for dx in range(0,224,32):
     for dy in range(0,96,32):stamp(canvas,floor,(x+dx,y-10+dy))
   stamp(canvas,player,(x+8,y+26));stamp(canvas,live,(x+65,y+26));stamp(canvas,dead,(x+137,y+26))
 file=f'review/{name}.png';canvas.save(OUT/file);scenes.append(file)
for usage,name in [('enemy','scene-09'),('player-companion','scene-56-59-et-66'),('player-transformation','scene-61-65'),('enemy-machine','scene-epaves-ennemies')]:
 entries=[e for e in ready if e['usage']==usage]
 if entries:scene(entries,name)

html=(OUT/'gallery.html').read_text(encoding='utf-8').replace('__DATA__',json.dumps(ready,ensure_ascii=False)).replace('__SCENES__',json.dumps(scenes)).replace('__FLOOR__','review/sol-jeu.png').replace('__PLAYER__','references/joueur-natif.png').replace('__OLDLIVE__','references/pillard-refuse-vivant.png').replace('__OLDDEAD__','references/pillard-refuse-cadavre.png')
html=html.replace('href="galerie-bestiaire.html"','href="../bestiaire-proposition-2026-10-05/galerie-bestiaire.html"')
(OUT/'galerie-allies-transformations.html').write_text(html,encoding='utf-8')
server=ROOT/'artifacts/armures-bicolores-completes-2026-10-05';destination=server/'allies-transformations-proposes';destination.mkdir(exist_ok=True)
for folder in ('tiles','review','references'):shutil.copytree(OUT/folder,destination/folder,dirs_exist_ok=True)
shutil.copyfile(OUT/'manifest.json',destination/'manifest.json')
served=html.replace('"tiles/','"allies-transformations-proposes/tiles/').replace('"review/','"allies-transformations-proposes/review/').replace('"references/','"allies-transformations-proposes/references/').replace("'references/","'allies-transformations-proposes/references/").replace("url('review/","url('allies-transformations-proposes/review/").replace('../bestiaire-proposition-2026-10-05/galerie-bestiaire.html','galerie-bestiaire.html')
(server/'galerie-allies-transformations.html').write_text(served,encoding='utf-8')

baseline=json.loads((OUT/'baseline.json').read_text(encoding='utf-8'));changed=[p for p,digest in baseline.items() if hashlib.sha256((ROOT/p).read_bytes()).hexdigest()!=digest]
document_edits=json.loads((OUT/'document-revisions.json').read_text(encoding='utf-8')) if (OUT/'document-revisions.json').exists() else {}
for p,revision in document_edits.items():
 assert baseline[p]==revision['previous_sha256'] and hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==revision['current_sha256'],p
 changed.remove(p)
assert not changed,changed
invariants=json.loads((OUT/'revisions/r2-invariants.json').read_text(encoding='utf-8'))
preserved=[]
for p,digest in invariants.items():
 number=int(Path(p).name[:3]);kind=Path(p).stem.split('-')[-1]
 if number in (57,59,60) or (number in (56,58) and kind=='cadavre'):continue
 assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==digest,p
 preserved.append(p)
round_preserved=[]
if (OUT/'revisions/r3-invariants.json').exists():
 for p,digest in json.loads((OUT/'revisions/r3-invariants.json').read_text(encoding='utf-8')).items():
  if int(Path(p).name[:3]) in current.get('numbers',[]):continue
  assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==digest,p
  round_preserved.append(p)
tile_checks=[]
for e in ready:
 for key in ('living','corpse'):
  im=Image.open(OUT/e[key]).convert('RGBA');pixels=list(im.get_flattened_data());opaque=[p for p in pixels if p[3]];colors=len({p[:3] for p in opaque});bounds=im.getbbox()
  assert im.size==(32,32) and opaque and {p[3] for p in pixels}<={0,255} and colors<=24,(e['number'],key)
  if e['usage']=='enemy-machine' and key=='living':
   assert hashlib.sha256((OUT/e[key]).read_bytes()).hexdigest()==hashlib.sha256((OLD/e[key]).read_bytes()).hexdigest(),e['number']
   assert 0<=bounds[0]<bounds[2]<=32 and 0<=bounds[1]<bounds[3]<=32
   margin_status='accepted original unchanged; existing boundary contact preserved'
  else:
   assert 0<bounds[0] and bounds[2]<32 and 0<bounds[1] and bounds[3]<32,(e['number'],key,bounds)
   margin_status='one-pixel transparent margin'
  tile_checks.append(dict(number=e['number'],kind=key,colors=colors,bounds=bounds,margin_status=margin_status))
assert len([e for e in ready if e['usage']=='player-companion'])==5 and not any(e['number']==60 for e in ready)
report=dict(candidates=len(ready),numbers=[e['number'] for e in ready],native_tiles=len(tile_checks),active_files_checked=len(baseline),active_files_changed=changed,document_edits=document_edits,preserved_previous_tiles=preserved,preserved_round_tiles=round_preserved,current_revision=current,checks=tile_checks,scenes=scenes,preview_only=True,engine_tests='not run; no production code or assets changed',review_status='awaiting opinion')
(OUT/'validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({k:v for k,v in report.items() if k!='checks'},ensure_ascii=False))
