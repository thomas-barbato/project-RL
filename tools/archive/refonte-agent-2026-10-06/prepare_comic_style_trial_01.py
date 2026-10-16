"""Record approval and prepare existing-art references for a separate BD trial."""
from pathlib import Path
import hashlib,json
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[2];BASE=Path(__file__).resolve().parent
OUT=BASE/'comic-style-review-01';OUT.mkdir(exist_ok=True)
def ref(p):return {'path':p.relative_to(ROOT).as_posix(),'binary_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
def jref(p):return {'reference':p.relative_to(ROOT).as_posix(),'reference_sha256':hashlib.sha256(p.read_text(encoding='utf-8').encode()).hexdigest()}
def write(p,d):
    assert not p.exists(),p
    p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
bank=BASE/'shaft-review-01/review-shafts-SHAFT-VIS01-01.json'
data=json.loads(bank.read_text(encoding='utf-8'))
approval=BASE/'shaft-visual-acceptance-01.json'
write(approval,{'date':'2026-10-10','review_id':'SHAFT-VIS01-ACCEPT01',
 'user_acceptance':'okay bon Je valide ces ajouts',
 'accepted_presentation_url':'http://127.0.0.1:59134/shaft-gallery-01.html','accepted_bank':jref(bank),
 'approved_candidates':[{k:r[k] for k in ('candidate_id','held_layer','inventory_tile','source')} for r in data['candidates']],
 'approved_placements':len(data['placements']),'approval_scope':'Six current shaft drawings and concrete two-handed placements on the five fixed class bodies.',
 'item_names_RP_stats':'not_defined_or_approved_by_this_visual_acceptance','runtime_integration':'not_integrated;art_bank_accepted'})
previous=BASE/'review-equipment-series-EQUIP-SERIES01-05.json';plan=json.loads(previous.read_text(encoding='utf-8'))
plan.update(revision=6,previous_proposal=jref(previous),current_shaft_acceptance=jref(approval),
 pixel_approval='shield_revision7_focalizer_revision1_and_shaft_revision1_accepted_separately;heavy_art_pending')
for s in plan['series']:
 if s['family_id']=='caster:channeling_shaft':s['model_art']='six_drawings_and_concrete_two_handed_placements_accepted;SHAFT-VIS01-ACCEPT01'
write(BASE/'review-equipment-series-EQUIP-SERIES01-06.json',plan)
names=['Bastion','Technomancien','Traqueur','Arcaniste','Métamorphe']
BANK=BASE/'weapon-silhouette-review-03'
paths=[BANK/'C1-Lnone-R01.png',BANK/'C2-Lnone-R06.png',BANK/'C3-Lnone-R17.png',
       BASE/'shaft-review-01/frames/EQ-H01-C4.png',BASE/'focalizer-review-01/frames/EQ-F06-C5-R-knife.png']
rows=[]
board=Image.new('RGB',(1280,370),(18,25,33));d=ImageDraw.Draw(board)
font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',19)
for i,p in enumerate(paths,1):
 im=Image.open(p).convert('RGBA')
 refs=[ref(p)]
 if i==1:
  shield=BASE/'shield-review-07/layers/EQ-S01-C1-L.png';im.alpha_composite(Image.open(shield).convert('RGBA'));refs.append(ref(shield))
 dest=OUT/f'reference-C{i}-native64.png';im.save(dest)
 large=im.resize((256,256),Image.Resampling.NEAREST);board.paste(large,((i-1)*256,32),large)
 d.text(((i-1)*256+18,308),f'{i:02} · {names[i-1]}',font=font,fill=(235,240,244))
 rows.append({'review_id':f'BD-C0{i}','class_number':i,'class_name':names[i-1],'native_reference':ref(dest),'source_layers':refs})
board.save(OUT/'existing-style-reference.png')
write(OUT/'references.json',{'review_id':'BD-STYLE01','scope':'style_trial_only;no_global_style_migration_or_gameplay_integration_approved','characters':rows,'reference_board':ref(OUT/'existing-style-reference.png')})
with (ROOT/'docs/REVUE_TEXTES_EFFETS_AGENT.md').open('a',encoding='utf-8',newline='\n') as f:
 f.write('''
### SHAFT-VIS01-ACCEPT01 — Six hampes validées ; essai BD demandé

Le joueur valide les six ajouts présentés dans `shaft-gallery-01.html` :
« okay bon Je valide ces ajouts ». Un accord distinct conserve les dessins
EQ-H01 à EQ-H06 et leurs prises concrètes à deux mains. La banque émise reste
inchangée. Les noms d'objets, textes RP et statistiques restent à définir.

Le joueur demande ensuite une planche des personnages et armes dans un style
bande dessinée. Il souhaite tester cette direction et ne passera à ce style
partout que s'il lui plaît. BD-STYLE01 est donc une étude visuelle distincte,
à présenter en grand et à l'échelle des tiles ; elle ne migre pas les assets
actifs et ne remplace pas les validations des séries précédentes.

Accord : [SHAFT-VIS01-ACCEPT01](../artifacts/refonte-agent-2026-10-06/shaft-visual-acceptance-01.json).
Inventaire : [EQUIP-SERIES01, révision 6](../artifacts/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-06.json).
''')
print(json.dumps({'approved_shafts':6,'trial_references':5,'runtime_asset_changes':0}))
