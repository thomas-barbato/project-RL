"""Preview-only torso item art. Production assets and game rules are read-only."""
import colorsys
import hashlib
import html
import json
import math
import shutil
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/torso-icons-review-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/torso-icons-review-01'
OUT = ROOT / 'artifacts/torso-icons-review-01'
ENTRIES = [
    (101, 212, 2, 1, 'Armure Cerbère', 'armure_cerbere', 'Dirty ivory light ceramic chest and shoulder shells on a charcoal flexible undersuit; two dark shoulder straps and small square buckles; segmented abdomen. Distinct lighter angular plates, not the very bulky Bastion design.'),
    (102, 4, 3, 1, 'Brigandine Cognefer', 'brigandine_cognefer', 'Worn burnt-orange brown industrial brigandine vest, compact segmented chest plates, dark collar and dark reinforced short sleeves, black straps and restrained steel buckles.'),
    (103, 213, 18, 2, 'Carapace Chrysalide', 'carapace_chrysalide', 'Olive and muted lime-green segmented carapace chest shell, pointed angular shoulder caps and charcoal flexible inner vest. Industrial protection, not actual insect anatomy.'),
    (104, 6, 4, 1, 'Carapace composite', 'composite_carapace', 'Compact charcoal and steel-blue composite chest protection with dark blue fabric underlayer, moderate squared shoulder pieces and restrained panel divisions. Dark functional industrial materials.'),
    (105, 214, 19, 2, 'Combinaison fantôme', 'combinaison_fantome', 'A slim charcoal technical torso vest with muted turquoise shoulder strips and small turquoise upper-chest plates, raised dark collar, dark flexible fabric. Turquoise paint, no emitted light.'),
    (106, 215, 20, 2, 'Manteau de mue', 'manteau_de_mue', 'Rust-orange outer torso garment with wide draped lapels and a broad dirty ivory inner front panel, black charcoal short sleeves and dark belt. Show the coat upper portion as a standalone short garment ending at the hips, no long tails or hood.'),
    (107, 216, 21, 2, 'Plastron Bastion', 'plastron_bastion', 'Very bulky dirty-ivory chest armor, large rounded rectangular shoulder shells, thick pale breastplate and segmented abdomen over a black-charcoal undersuit. Broad heavy industrial ceramic or metal, distinct from lighter Cerbere.'),
    (108, 208, 22, 3, 'Tablier de cuir', 'patched_plating', 'A dirty cream ivory leather protective bib apron with dark neck straps and brass-grey square buckles, over dark blue short work sleeves. A short thick front apron ending at hips. The ivory apron is the main recolorable material; retain blue fabric as secondary.'),
    (109, 217, 26, 4, 'Robe de la Seconde Ombre', 'veste_de_la_seconde_ombre', 'Worn deep red outer robe torso with wide lapels, dark charcoal central cloth and broad dark belt, short loose red sleeves. Draw its upper-body tunic ending at the belt or hips with a natural finished hem. No hood, skirt, legs or mask.'),
    (110, 209, 23, 3, 'Tenue de membranes', 'veste_de_membranes', 'Muted slate blue-grey membrane coat upper torso with a high open collar, broad pale grey central chest panel, angular layered shoulders, dark short sleeves and a dark waist seam. Not living anatomy.'),
    (111, 210, 24, 3, 'Combinaison Peau-Seconde', 'veste_de_peau_seconde', 'Slim navy torso suit with broad dirty ivory shoulder caps, ivory breastplate panels and ivory waist armor, navy fabric at the ribs and short upper sleeves. Preserve slim proportions rather than heavy Bastion plates.'),
    (112, 211, 25, 3, 'Tenue du technicien', 'veste_de_veille', 'Charcoal and muted dark teal industrial work vest, small dirty ivory shoulder guards and two small ivory chest patches, utilitarian short sleeves. Narrow functional coveralls upper portion, no extra gadgets.'),
    (113, 15, 5, 1, 'Tenue d’aviateur', 'veste_matelassee', 'A dirty beige grey padded aviator jacket torso, quilted large padded masses, dark collar and seams, compact short sleeves, small dark fastenings. No goggles, pilot helmet or orange accessories.'),
    (114, 218, 27, 4, 'Tenue du nomade', 'veste_a_fibres_croisees', 'Worn red-orange hoodless nomad torso coat, dark charcoal inner front, wide crossed dark straps and dark belt, short worn red sleeves. Compact practical layered cloth, natural hip-length hem, no hood or long skirt.'),
]

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')

def prepare():
    assert not (DOC/'acceptance.json').exists(), 'Accepted proposals are frozen'
    for path in [ART, ART/'generation', ART/'native', ART/'references', DOC, OUT]:
        path.mkdir(parents=True, exist_ok=True)
    common = ('Use case: stylized-concept. Asset type: one isolated torso armor ITEM ICON for Project RL, a grimdark futuristic roguelike. '
              'The reference is an existing complete OUTFIT, provided only to identify the chest garment, its finish and materials. '
              'Redraw ONLY its upper-body garment or chest armor as a freestanding inventory object, from neck opening to waist/hips, with shoulder pieces and short upper sleeves if present. '
              'A clearly EMPTY wearable object, not a living torso: transparent empty collar and sleeve openings, no human inside, no mannequin, no skin, no head, no helmet, no mask, no hood, no hands, no forearms, no legs, no boots, no weapon. '
              'For robes and long coats make a compact upper-garment item silhouette, no long skirt or full-body silhouette. '
              'Keep the reference chest design recognizable, original material colors and identity; do not invent ornaments or text. '
              'Slightly elevated frontal game item view, near symmetric readable large masses, similar proportions to a breastplate icon; no dramatic perspective. '
              'Match the accepted matte worn painted game finish: controlled dark contours, broad shadow planes, locally scuffed metal or worn cloth, restrained highlights. Not shiny plastic, not noisy microdetail, not vector clipart. '
              'The object will fill about 52 of a 64 pixel inventory cell; prioritize a strong torso silhouette and few large readable divisions. '
              'Exactly ONE COMPLETE object with a finished lower hem, no crop, generous clear margins at every edge, true transparent background, no cast shadow, no floor, no frame, no label. ')
    jobs=[]
    for ident,outfit,n,batch,label,key,detail in ENTRIES:
        ref=ROOT/f'assets/sprites/art-finish-{batch:02}/FIN-{n:02}.png'
        # Native reference enlarged only for model readability, preserving exact pixels.
        im=Image.open(ref).convert('RGBA');im.resize((1024,1024),Image.Resampling.NEAREST).save(ART/'references'/f'TOR-{ident}-outfit.png')
        jobs.append(dict(id=f'TOR-{ident}',illustration=ident,outfit=outfit,finish_number=n,batch=batch,label=label,identity=f'core:{key}',prompt=common+detail,
                         referenced_image_paths=[(ART/'references'/f'TOR-{ident}-outfit.png').as_posix()],transparent_background=True,
                         output=(ART/f'TOR-{ident}-source.png').as_posix()))
    write(ART/'prompts.json',dict(review='TORSO-ICONS-01',mode='built-in imagegen',status='candidate_not_integrated',jobs=jobs))
    write(DOC/'request.json',dict(review='TORSO-ICONS-01',date='2026-10-20',status='candidate_not_integrated',scope='14 real torso items; chest-only ground and inventory icons; whole equipped outfits retained; color appearances reviewed side by side',human_visual_approval_required=True,prior_delegation_not_used=True,mechanics_unchanged=True))
    baseline=DOC/'production-before.json'
    if not baseline.exists():
        paths=list((ROOT/'assets/sprites').rglob('*.png'))+[ROOT/'src/entity/equipment_palette.rs',ROOT/'src/game_art_finish.rs']
        write(baseline,{p.relative_to(ROOT).as_posix():sha(p) for p in paths})
    print('Prepared 14 chest item requests; five class-only default outfits excluded')

PALETTES = [
    ('A', 'Origine', None),
    ('B', 'Bleu ardoise', [[14,30,49],[49,87,124],[145,180,205]]),
    ('C', 'Vert mousse', [[24,39,26],[72,112,62],[160,184,130]]),
    ('D', 'Rouille', [[66,30,19],[160,77,36],[223,158,94]]),
]
RED_PALETTE = ('E', 'Rouge sombre', [[62,10,16],[163,27,35],[228,104,100]])
# Only the Second Shadow robe already has an unambiguous red base. The nomad's
# red-orange base still receives a true crimson option, distinct from rust.
RED_ALREADY_PRESENT = {109}

def palette_options(ident):
    return PALETTES if ident in RED_ALREADY_PRESENT else PALETTES+[RED_PALETTE]

def normalize(job):
    source=Path(job['output']);im=Image.open(source).convert('RGBA')
    box=im.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
    assert box and box[0]>0 and box[1]>0 and box[2]<im.width and box[3]<im.height,(job['id'],'clipped source')
    cut=im.crop(box);ratio=min(54/cut.width,54/cut.height)
    cut=cut.resize((round(cut.width*ratio),round(cut.height*ratio)),Image.Resampling.LANCZOS)
    icon=Image.new('RGBA',(64,64));icon.alpha_composite(cut,((64-cut.width)//2,(64-cut.height)//2))
    return icon,box

def icon_material(icon,ident):
    """Author-time swatch memberships, frozen as explicit per-pixel material maps."""
    labels=[];members={}
    for pixel in icon.get_flattened_data():
        r,g,b,a=pixel;h,l,s=colorsys.rgb_to_hls(r/255,g/255,b/255);h*=360
        main=False
        if ident in (101,107,108,111,113):main=l>.22 and (h<95 or s<.06) and s<.5
        elif ident==106:main=(h<30 or h>340) and s>.50 and l>.09
        elif ident==114:main=(h<26 or h>340) and s>.55 and l>.09
        elif ident in (102,109):main=(h<32 or h>340) and s>.38 and l>.09
        elif ident==103:main=40<h<105 and s>.24 and l>.12
        elif ident in (104,110):main=173<h<260 and s>.08 and l>.08
        elif ident==105:main=150<h<210 and s>.27 and l>.13
        elif ident==112:main=130<h<235 and s>.05 and .06<l<.46
        # Deep outline, empty openings, neutral straps and secondary fabric stay unchanged.
        value=1 if a and main else 0
        labels.append((value,0,0,255) if a else (0,0,0,0))
        members[pixel[:3]]=max(members.get(pixel[:3],0),value)
    mask=Image.new('RGBA',icon.size);mask.putdata(labels)
    return mask,members

def recolor(im,mask,ramp):
    if ramp is None:return im.copy()
    result=im.copy();pixels=[]
    for before,label in zip(im.get_flattened_data(),mask.get_flattened_data()):
        if label[0]!=1:
            pixels.append(before);continue
        v=max(before[:3])/255
        low,high,t=(ramp[0],ramp[1],max(0,min(1,(v-.12)/.41))) if v<.53 else (ramp[1],ramp[2],max(0,min(1,(v-.53)/.47)))
        rgb=[int(math.floor(low[c]*(1-t)+high[c]*t+.5)) for c in range(3)]
        pixels.append(tuple((rgb[c]*9+before[c]+5)//10 for c in range(3))+(before[3],))
    result.putdata(pixels);return result

def build():
    assert not (DOC/'acceptance.json').exists(),'Accepted review is frozen'
    jobs=json.loads((ART/'prompts.json').read_text('utf-8'))['jobs'];entries=[]
    for job in jobs:
        icon,source_box=normalize(job);ident=job['illustration'];mask,members=icon_material(icon,ident)
        mask.save(ART/'native'/f'TOR-{ident}-material.png')
        write(ART/'native'/f'TOR-{ident}-members.json',{'rgb_labels':[[*rgb,label] for rgb,label in sorted(members.items())]})
        n=job['finish_number'];worn=ROOT/'assets/sprites/art-finish-completion-06/worn'
        body=Image.open(worn/f'FIN-{n:02}-base.png').convert('RGBA');body_mask=Image.open(worn/f'FIN-{n:02}-base-mask.png').convert('RGBA')
        variants=[]
        for letter,label,ramp in palette_options(ident):
            item=recolor(icon,mask,ramp);actor=recolor(body,body_mask,ramp)
            ip=ART/'native'/f'TOR-{ident}-{letter}.png';bp=ART/'native'/f'TOR-{ident}-{letter}-worn.png'
            item.save(ip);actor.save(bp)
            assert item.getchannel('A').tobytes()==icon.getchannel('A').tobytes()
            assert actor.getchannel('A').tobytes()==body.getchannel('A').tobytes()
            for original,modified,material in zip(body.get_flattened_data(),actor.get_flattened_data(),body_mask.get_flattened_data()):
                assert material[0]==1 or original==modified
            variants.append(dict(code=f'TOR-{ident}-{letter}',label=label,ramp=ramp,icon=ip.relative_to(ROOT).as_posix(),worn=bp.relative_to(ROOT).as_posix(),icon_sha256=sha(ip),worn_sha256=sha(bp)))
        entries.append({**{k:v for k,v in job.items() if k not in ('prompt','referenced_image_paths')},'source_sha256':sha(Path(job['output'])),'source_bounds':source_box,'material_mask':(ART/'native'/f'TOR-{ident}-material.png').relative_to(ROOT).as_posix(),'variants':variants})
    old=json.loads((DOC/'production-before.json').read_text('utf-8'));assert all(sha(ROOT/p)==s for p,s in old.items())
    count=sum(len(e['variants']) for e in entries)
    write(DOC/'candidates.json',dict(review='TORSO-ICONS-01',revision='r02-red',status='awaiting_human_visual_approval',native_cell=64,models=len(entries),appearances=count,new_palette_proposals=['Bleu ardoise','Vert mousse','Rouille','Rouge sombre'],primary_ramp_mix=[9,1],existing_palettes_unchanged=True,entries=entries))
    write(DOC/'checks.json',dict(source_masters=14,source_clipping=False,transparent_icons=count,identical_variant_alpha=True,protected_worn_materials_unchanged=True,production_files_unchanged=len(old),runtime_integration=False))
    print('Packaged 14 candidates,',count,'icon/outfit pairs; production files unchanged:',len(old))

def add_red():
    """Extend the current review from its frozen material maps; never rewrite A-D."""
    assert not (DOC/'acceptance.json').exists(),'Accepted review is frozen'
    data=json.loads((DOC/'candidates.json').read_text('utf-8'));before={}
    for entry in data['entries']:
        for variant in entry['variants']:
            if variant['code'][-1] in 'ABCD':
                for field in ('icon','worn'):
                    path=variant[field];assert sha(ROOT/path)==variant[field+'_sha256'];before[path]=sha(ROOT/path)
    new=[]
    for entry in data['entries']:
        ident=entry['illustration']
        if ident in RED_ALREADY_PRESENT:continue
        base=next(v for v in entry['variants'] if v['code'].endswith('-A'))
        icon=Image.open(ROOT/base['icon']).convert('RGBA');mask=Image.open(ROOT/entry['material_mask']).convert('RGBA')
        body=Image.open(ROOT/base['worn']).convert('RGBA')
        body_mask=Image.open(ROOT/f'assets/sprites/art-finish-completion-06/worn/FIN-{entry["finish_number"]:02}-base-mask.png').convert('RGBA')
        letter,label,ramp=RED_PALETTE;result=recolor(icon,mask,ramp);actor=recolor(body,body_mask,ramp)
        for original,changed,labels in [(icon,result,mask),(body,actor,body_mask)]:
            assert original.getchannel('A').tobytes()==changed.getchannel('A').tobytes()
            assert all(m[0]==1 or a==b for a,b,m in zip(original.get_flattened_data(),changed.get_flattened_data(),labels.get_flattened_data()))
        ip=ART/'native'/f'TOR-{ident}-E.png';bp=ART/'native'/f'TOR-{ident}-E-worn.png';result.save(ip);actor.save(bp)
        variant=dict(code=f'TOR-{ident}-E',label=label,ramp=ramp,icon=ip.relative_to(ROOT).as_posix(),worn=bp.relative_to(ROOT).as_posix(),icon_sha256=sha(ip),worn_sha256=sha(bp))
        entry['variants']=[v for v in entry['variants'] if not v['code'].endswith('-E')]+[variant];new.append(variant)
    assert len(new)==13
    assert all(sha(ROOT/path)==value for path,value in before.items())
    old=json.loads((DOC/'production-before.json').read_text('utf-8'));assert all(sha(ROOT/p)==s for p,s in old.items())
    data.update(revision='r02-red',appearances=sum(len(e['variants']) for e in data['entries']),new_palette_proposals=['Bleu ardoise','Vert mousse','Rouille','Rouge sombre'],red_additions=13,red_already_present=['TOR-109-A'])
    assert data['appearances']==69;write(DOC/'candidates.json',data)
    write(DOC/'red-amendment-02.json',dict(review='TORSO-ICONS-01',revision='r02-red',date='2026-10-20',status='candidate_not_integrated',request='Ajouter une variante rouge à chaque armure qui en manque ; le Manteau de mue présente deux oranges.',method='Existing frozen item and worn material maps; same authoring palette interpolation; no new master, geometry, or global hue filter',palette=RED_PALETTE,red_already_present=['TOR-109-A'],new_variants=new,previous_candidate_hashes=before,previous_candidate_files_unchanged=len(before),production_files_unchanged=len(old)))
    checks=json.loads((DOC/'checks.json').read_text('utf-8'));checks.update(transparent_icons=69,red_additions=13,models_with_red=14,red_already_present=['TOR-109-A'],previous_candidate_files_unchanged=len(before),proposal_palette_names=data['new_palette_proposals'],visual_self_check='Red amendment requires native review; awaiting human approval');write(DOC/'checks.json',checks)
    print('Added 13 red icon/outfit pairs; 112 A-D files and',len(old),'production files unchanged')

def font(size=17):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)

def paste(page,im,x,y):
    page.paste(im,(x,y),im)

def publish():
    data=json.loads((DOC/'candidates.json').read_text('utf-8'));entries=data['entries']
    sheets=[]
    for start in range(0,14,7):
        page=Image.new('RGB',(1232,106+7*154),'#11191d');d=ImageDraw.Draw(page)
        d.text((18,12),'TORSO-ICONS-01 · candidats · taille native',font=font(22),fill='#decca7')
        d.text((18,45),'Icône 64 px · porteur à l’échelle du jeu · aucune intégration active',font=font(16),fill='#abbab5')
        d.text((20,78),'Actuel',font=font(16),fill='#a8b7b1')
        for i,(_,label,_) in enumerate(PALETTES):
            x=156+i*264;d.text((x,78),label+' · objet / porté',font=font(16),fill='#decca7')
        for row,e in enumerate(entries[start:start+7]):
            y=106+row*154;d.rectangle((12,y,1220,y+146),fill='#1b272d');d.text((24,y+8),e['id']+' · '+e['label'],font=font(17),fill='#decca7')
            old=Image.open(ROOT/f'assets/sprites/art-finish-completion-06/worn/icon-{e["illustration"]}.png').convert('RGBA');paste(page,old,28,y+61)
            for col,v in enumerate(e['variants'][:4]):
                x=156+col*264;d.rectangle((x,y+61,x+63,y+124),fill='#0d1216',outline='#5b625d')
                paste(page,Image.open(ROOT/v['icon']).convert('RGBA'),x,y+61)
                d.rectangle((x+108,y+61,x+171,y+124),fill='#454841')
                paste(page,Image.open(ROOT/v['worn']).convert('RGBA'),x+76,y+25)
                d.text((x+8,y+127),v['code'],font=font(12),fill='#a8b7b1')
        path=OUT/f'torses-native-{start//7+1:02}.png';page.save(path);sheets.append(path)
    # Compact, explicitly labelled compositing review. Terrain pixels come from a real lit capture;
    # the candidate objects and wearers are intentionally not claimed as an active engine capture.
    terrain=Image.open(ROOT/'artifacts/art-quality-audit-01/completion-wings-viewport.png').convert('RGB')
    tile=terrain.crop((514,320,578,384))
    for start in range(0,14,7):
        page=Image.new('RGB',(1232,110+7*148),'#11191d');d=ImageDraw.Draw(page)
        d.text((18,12),'Au sol et équipé · montage de revue à échelle native',font=font(22),fill='#decca7')
        d.text((18,46),'Sol extrait de la carte éclairée ; candidats composés pour comparaison, pas encore actifs.',font=font(15),fill='#abbab5')
        for row,e in enumerate(entries[start:start+7]):
            y=88+row*148;d.text((18,y),e['id']+' · '+e['label'],font=font(17),fill='#decca7')
            for col,v in enumerate(e['variants'][:4]):
                x=24+col*294
                for j in range(4):page.paste(tile,(x+j*64,y+32))
                paste(page,Image.open(ROOT/v['icon']).convert('RGBA'),x+28,y+32)
                paste(page,Image.open(ROOT/v['worn']).convert('RGBA'),x+126,y-4)
                d.text((x+8,y+102),v['label'],font=font(15),fill='#c8cfc8')
        page.save(OUT/f'torses-sol-{start//7+1:02}.png')
    # One concise set emphasizing blue cloth and the preserved secondary materials.
    page=Image.new('RGB',(1000,330),'#11191d');d=ImageDraw.Draw(page)
    d.text((18,12),'Même couleur sur l’objet et la tenue portée · 64 px natifs',font=font(20),fill='#decca7')
    for col,ident in enumerate([109,106,108,107]):
        e=next(e for e in entries if e['illustration']==ident);v=e['variants'][1];x=20+col*245
        d.text((x,53),e['id'],font=font(18),fill='#decca7');d.text((x,81),e['label'],font=font(14),fill='#b6c4be')
        d.rectangle((x,119,x+63,182),fill='#0d1216',outline='#5b625d');paste(page,Image.open(ROOT/v['icon']).convert('RGBA'),x,119)
        paste(page,Image.open(ROOT/v['worn']).convert('RGBA'),x+76,83)
        d.text((x,231),'Bleu ardoise · matières ciblées',font=font(14),fill='#abbab5')
    page.save(OUT/'couleurs-portees-native.png')
    page=Image.new('RGB',(1232,108+7*102),'#11191d');d=ImageDraw.Draw(page)
    d.text((18,12),'Inventaire · taille de la liste actuelle : 40 px',font=font(22),fill='#decca7')
    d.text((18,48),'Même emplacement de 40 px : ancien personnage miniature / nouvelle pièce de torse.',font=font(16),fill='#abbab5')
    for index,e in enumerate(entries):
        x=18+(index%2)*608;y=94+(index//2)*102;d.rectangle((x,y,x+588,y+94),fill='#1b272d')
        d.text((x+10,y+6),e['id']+' · '+e['label'],font=font(15),fill='#decca7')
        old=Image.open(ROOT/f'assets/sprites/art-finish-completion-06/worn/icon-{e["illustration"]}.png').convert('RGBA').resize((40,40),Image.Resampling.NEAREST)
        paste(page,old,x+12,y+37);d.text((x+62,y+48),'→',font=font(18),fill='#aab8b0')
        for i,v in enumerate(e['variants'][:4]):
            xx=x+110+i*112;d.rectangle((xx,y+36,xx+39,y+75),fill='#0d1216',outline='#5b625d')
            paste(page,Image.open(ROOT/v['icon']).convert('RGBA').resize((40,40),Image.Resampling.NEAREST),xx,y+36)
            d.text((xx+46,y+48),v['code'][-1],font=font(16),fill='#c4cdc5')
    page.save(OUT/'inventaire-40px.png')
    red_entries=[e for e in entries if any(v['code'].endswith('-E') for v in e['variants'])];red_sheets=[]
    for start in range(0,len(red_entries),7):
        subset=red_entries[start:start+7];page=Image.new('RGB',(1100,100+len(subset)*150),'#11191d');d=ImageDraw.Draw(page)
        d.text((18,12),'Rouge ajouté · origine / rouille / rouge sombre',font=font(22),fill='#decca7')
        d.text((18,45),'E : nouvelle couleur sur l’objet et la tenue · dimensions natives · aperçu d’inventaire à 40 px',font=font(15),fill='#abbab5')
        for row,e in enumerate(subset):
            y=88+row*150;d.rectangle((12,y,1088,y+142),fill='#1b272d');d.text((24,y+6),e['id']+' · '+e['label'],font=font(17),fill='#decca7')
            for col,letter in enumerate('ADE'):
                v=next(v for v in e['variants'] if v['code'].endswith('-'+letter));x=28+col*294
                d.text((x,y+32),letter+' · '+v['label'],font=font(14),fill='#bbc8c0')
                d.rectangle((x,y+57,x+63,y+120),fill='#0d1216',outline='#5b625d');paste(page,Image.open(ROOT/v['icon']).convert('RGBA'),x,y+57)
                d.rectangle((x+108,y+57,x+171,y+120),fill='#454841');paste(page,Image.open(ROOT/v['worn']).convert('RGBA'),x+76,y+21)
                if letter=='E':
                    d.rectangle((x+203,y+69,x+242,y+108),fill='#0d1216',outline='#5b625d');paste(page,Image.open(ROOT/v['icon']).convert('RGBA').resize((40,40),Image.Resampling.NEAREST),x+203,y+69)
        path=OUT/f'rouges-native-{start//7+1:02}.png';page.save(path);red_sheets.append(path)
    parts=['''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>TORSO-ICONS-01 · armures de torse</title>
<style>body{background:#11191d;color:#d1dad6;margin:24px;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1100px}a{color:#d8bd85}.scroll{overflow:auto;max-width:100%;margin:16px 0;border:1px solid #43514b}img{display:block;max-width:none}.card{background:#1b272d;margin:20px 0;padding:16px;overflow:auto}.strip{display:flex;gap:12px}.pair{min-width:230px;background:#293332;padding:10px}.pair img{display:inline-block;vertical-align:middle}.pair strong{display:block;color:#decca7}nav{display:flex;gap:24px;flex-wrap:wrap}summary{cursor:pointer;color:#decca7}</style>
<h1>TORSO-ICONS-01 · icônes d’armures de torse</h1><p>14 pièces isolées, sans personnage miniature. A : origine ; B : bleu ardoise ; C : vert mousse ; D : rouille ; E : rouge sombre ajouté aux 13 armures qui n’avaient pas de vrai rouge. La Robe de la Seconde Ombre possède déjà son rouge d’origine (TOR-109-A). Les images sont à leur taille prévue, sans agrandissement automatique.</p><p><strong>Candidats à valider : aucun remplacement actif dans le jeu.</strong> Équiper le torse continuera à choisir la tenue complète. Les cinq apparences de classe par défaut ne sont pas des objets de loot. Les versions A–D précédentes sont conservées à l’identique.</p>
<nav><a href="#rouges">Rouges ajoutés</a><a href="#planches">Avant / après</a><a href="#sol">Au sol et équipé</a><a href="#details">Codes individuels</a></nav><h2 id="rouges">13 nouveaux rouges · comparaison avec les oranges</h2><p>La teinte E est rouge carmin sombre, distincte de l’orange et de la rouille. Les panneaux ivoire et autres matériaux secondaires restent protégés. La robe TOR-109 garde son rouge A et ne reçoit pas de doublon.</p>''']
    for p in red_sheets:
        im=Image.open(p);parts.append(f'<div class="scroll"><img src="{p.name}" width="{im.width}" height="{im.height}" alt="Comparaison origine, rouille et nouveau rouge, avec vêtement porté"></div>')
    parts.append('<h2 id="planches">Inventaire et tenue · premières palettes conservées</h2>')
    for p in sheets:
        im=Image.open(p);parts.append(f'<div class="scroll"><img src="{p.name}" width="{im.width}" height="{im.height}" alt="Comparaison actuelle et 4 palettes, taille native"></div>')
    im=Image.open(OUT/'inventaire-40px.png');parts.append(f'<h2>Dans la liste d’inventaire · 40 px réels</h2><div class="scroll"><img src="inventaire-40px.png" width="{im.width}" height="{im.height}" alt="Avant et après dans les emplacements actuels de 40 pixels"></div>')
    parts.append('<h2 id="sol">Au sol et équipé</h2><p>Ces montages comparent les objets et porteurs sur des pixels de sol issus de la carte éclairée. Ils servent à examiner les formes et couleurs ; ce ne sont pas des captures d’une intégration active.</p>')
    for name in ['torses-sol-01.png','torses-sol-02.png','couleurs-portees-native.png']:
        im=Image.open(OUT/name);parts.append(f'<div class="scroll"><img src="{name}" width="{im.width}" height="{im.height}" loading="lazy" alt="Candidats au sol et portés à taille native"></div>')
    parts.append('<h2 id="details">Candidats individuels</h2>')
    for e in entries:
        parts.append(f'<article class="card" id="{e["id"]}"><h3>{e["id"]} · {html.escape(e["label"])}</h3><div class="strip">')
        for v in e['variants']:
            parts.append(f'<div class="pair"><strong>{v["code"]} · {v["label"]}</strong><img src="../../{v["icon"]}" width="64" height="64" loading="lazy" alt="Objet seul"><img src="../../{v["worn"]}" width="128" height="128" loading="lazy" alt="Tenue complète correspondante"></div>')
        parts.append(f'</div><details><summary>Source du nouveau dessin</summary><a href="../../assets/source-art/torso-icons-review-01/{e["id"]}-source.png">Ouvrir le dessin source</a></details></article>')
    parts.append('<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/torso-icons-review-01/README.md">Périmètre et contrôles</a> · <a href="../../assets/source-art/torso-icons-review-01/prompts.json">Prompts imagegen intégrés</a></p></html>')
    (OUT/'index.html').write_text('\n'.join(parts),encoding='utf-8')
    print('Published native before/after sheets,',sum(len(e['variants']) for e in entries),'pairs, red amendment and ground-comparison montages')

if __name__=='__main__':
    import sys
    if '--prepare' in sys.argv:prepare()
    if '--build' in sys.argv:build()
    if '--add-red' in sys.argv:add_red()
    if '--publish' in sys.argv:publish()
