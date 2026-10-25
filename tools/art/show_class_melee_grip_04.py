"""Review actual melee compositions and narrow knife-orientation regression."""
import hashlib
import json
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,write_json,font,sha

OUT=ROOT/'artifacts/class-armor-integration-01'
DEST=OUT/'melee-grip-04'
DOC=ROOT/'docs/reviews/class-armor-integration-01/melee-grip-04'
CLASSES=['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe']


def normalize(folder):
    path=folder/'capture.json';data=json.loads(path.read_text('utf-8'))
    if data['exported_pngs_need_vertical_flip']:
        for p in folder.glob('outfit-*.png'):
            Image.open(p).transpose(Image.Transpose.FLIP_TOP_BOTTOM).save(p,optimize=True)
        data['exported_pngs_need_vertical_flip']=False;write_json(path,data)
    return data


def tile(index,label,folder=DEST/'compositor'):
    data=json.loads((folder/'capture.json').read_text('utf-8'))
    c=data['configurations'].index(label);cols=data['columns']
    im=Image.open(folder/f'outfit-{index:02}.png').convert('RGBA')
    return im.crop((c%cols*128,c//cols*128,c%cols*128+128,c//cols*128+128))


def main():
    manifest=json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text('utf-8'))
    base=Image.open(ROOT/manifest['atlases']['base']['path']).convert('RGBA')
    for approval in manifest['source_acceptances']:
        assert sha(ROOT/approval['path'])==approval['sha256']
        accepted=json.loads((ROOT/approval['path']).read_text('utf-8'))
        for e in accepted['entries']:
            for ref in [e['source'],e['native'],*e['components'].values()]:
                assert sha(ROOT/ref['path'])==ref['sha256']
    for e in manifest['outfits']:
        ref=e.get('runtime_native',e['native']);n=e['index']
        assert sha(ROOT/ref['path'])==ref['sha256']
        assert base.crop((n%16*128+32,n//16*128+32,n%16*128+96,n//16*128+96)).tobytes()==Image.open(ROOT/ref['path']).convert('RGBA').tobytes()
    data=normalize(OUT/'compositor');melee=normalize(DEST/'compositor')
    prior=json.loads((DOC/'before-cell-hashes.json').read_text('utf-8'));preserved=0;changes=[]
    for n in range(25):
        im=Image.open(OUT/f'compositor/outfit-{n:02}.png').convert('RGBA')
        for c,label in enumerate(data['configurations']):
            image=im.crop((c%16*128,c//16*128,c%16*128+128,c//16*128+128));key=f'{n:02}/{label}'
            if hashlib.sha256(image.tobytes()).hexdigest()==prior[key]:preserved+=1
            else:
                assert label in ('weapon-068','weapon-079','dual-melee','dual-melee-ranged'),key
                changes.append(key)
    assert len(changes)==100
    board=Image.new('RGB',(620,590),'#172126');d=ImageDraw.Draw(board)
    for col,label in enumerate(['Avant · droite','04 · droite','04 · gauche','04 · deux mains']):
        d.text((110+col*125,8),label,font=font(12),fill='white')
    for row,name in enumerate(CLASSES):
        index=row*5+4;y=32+row*110;d.text((4,y+42),name,font=font(12),fill='white')
        images=[Image.open(DOC/f'before-{index:02}-weapon-068.png').convert('RGBA'),
                tile(index,'weapon-068-right'),tile(index,'weapon-068-left'),tile(index,'dual-knives')]
        for col,image in enumerate(images):
            image=image.crop((12,16,116,120));board.paste(image,(110+col*125,y),image)
    board.save(DEST/'knife-native.png')
    # Enlarge the exact held pixels to make the cutting-bevel direction visible.
    manifest=json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text('utf-8'))
    board=Image.new('RGB',(620,220),'#29343a');d=ImageDraw.Draw(board)
    index=4;x,y=manifest['outfits'][index]['palms'][1]
    for col,(model,title) in enumerate([(68,'Couteau céramique'),(79,'Stylet courbe')]):
        d.text((10+col*310,8),title,font=font(14),fill='white')
        images=[Image.open(DOC/f'before-{index:02}-weapon-{model:03}.png').convert('RGBA'),tile(index,f'weapon-{model:03}-right')]
        for j,im in enumerate(images):
            crop=im.crop((round(x)-6,round(y)-18,round(x)+7,round(y)+6)).resize((78,144),Image.Resampling.NEAREST)
            px=10+col*310+j*135;d.text((px,34),'Avant' if j==0 else '04 · corrigé',font=font(12),fill='white');board.paste(crop,(px,57),crop)
    board.save(DEST/'blade-direction-zoom.png')
    board=Image.new('RGB',(620,590),'#172126');d=ImageDraw.Draw(board)
    for row in range(5):
        d.text((2,38+row*108),f'{row+1:02}',font=font(12),fill='white')
        for col,(name,label) in enumerate([('Couteau G','weapon-068-left'),('Couteau D','weapon-068-right'),('Stylet G','weapon-079-left'),('Stylet D','weapon-079-right'),('Marteau','weapon-097-right')]):
            if row==0:d.text((35+col*116,5),name,font=font(12),fill='white')
            im=tile(row,label).crop((12,16,116,120));board.paste(im,(32+col*116,30+row*108),im)
    board.save(DEST/'all-tiers-native.png')
    # Keep all 27 existing one-hand melee models and all five classes inspectable.
    for cls in range(5):
        board=Image.new('RGB',(1024,896),'#243037');d=ImageDraw.Draw(board)
        for c,model in enumerate(list(range(68,88))+list(range(94,101))):
            col=c%8;row=c//8
            for side,name in enumerate(['right','left']):
                im=tile(cls*5+4,f'weapon-{model:03}-{name}');yy=row*224+side*108
                board.paste(im,(col*128,yy),im);d.text((col*128+4,yy+3),f'{model:03} · {name}',font=font(11),fill='white')
        board.save(DEST/f'all-melee-class-{cls}.png')
    section='''<section id="melee-grip-04"><h1>04 · Armes de mêlée à une main</h1>
<p>Couteau céramique et stylet courbe : tranchant tourné vers l’extérieur, dans chaque main. Les dessins des autres modèles gardent leur orientation actuelle.</p>
<div class="media"><img src="melee-grip-04/knife-native.png" alt="Avant et correction 04, couteau dans chaque main et deux couteaux sur les cinq classes à taille native"></div>
<details open><summary>Voir le tranchant au zoom ×6</summary><div class="media"><img src="melee-grip-04/blade-direction-zoom.png" alt="Pixels réellement portés du couteau et du stylet, avant et après au zoom six"></div></details>
<details><summary>Comparer les cinq niveaux d’armure du Bastion</summary><div class="media"><img src="melee-grip-04/all-tiers-native.png" alt="Couteau et stylet dans chaque main sur les cinq Bastions, avec marteau de contrôle"></div></details>
<details><summary>Examiner les 27 modèles dans chaque main, sur les cinq classes</summary>GALLERIES</details>
<p>Contrôles : 25 tenues × 27 modèles × 2 mains ; deux couteaux, deux stylets et paires couteau + pistolet. Les prises utilisent les poignées et une couche de doigts séparée.</p></section>'''
    galleries=''.join(f'<h3>{name}</h3><div class="media"><img src="melee-grip-04/all-melee-class-{i}.png" alt="Les 27 armes de mêlée à une main sur {name}"></div>' for i,name in enumerate(CLASSES))
    section=section.replace('GALLERIES',galleries)
    html=(OUT/'index.html').read_text('utf-8')
    import re
    html=re.sub(r'<section id="melee-grip-04">.*?</section>','',html,flags=re.S)
    html=html.replace('<h1 id="top">',section+'<h1 id="top">',1).replace('ws.value=102;','ws.value=69;')
    (OUT/'index.html').write_text(html,encoding='utf-8')
    checks=json.loads((DOC/'checks.json').read_text('utf-8')) if (DOC/'checks.json').exists() else {}
    checks.update({'review_id':'CLASS-MELEE-GRIP04','native_tile':64,'one_hand_melee_models':27,
        'outfits':25,'hand_cases':1350,'focused_compositions':25*len(melee['configurations']),
        'ordinary_compositions':3100,'exactly_preserved_compositions':preserved,'changed_compositions':len(changes),
        'unexpected_changes':[],'changed_models':[68,79],'source_pixels':'no raster assets modified; mirror whole colored weapon around its grip',
        'mechanics_changes':False,'save_format_changes':False,'source_acceptance_hashes_verified':5,'frozen_native_source_groups_verified':25,
        'visual_approval':'corrective orientation requested; awaiting feedback'})
    write_json(DOC/'checks.json',checks)
    print(json.dumps({'preserved':preserved,'changed':len(changes),'focused_compositions':25*len(melee['configurations'])}))


if __name__=='__main__':main()
