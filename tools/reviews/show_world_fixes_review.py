"""FIXES-U01-02: native GPU proof and a separately labelled grimdark proposal."""
from pathlib import Path
import json
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/street-defects-review-01'
ENGINE=OUT/'engine02'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-defects-review-02'
VIEW=(334,78,1552,782)

def build():
    if (OUT/'engine03/checks.json').exists():
        from show_street_style_review import build as latest
        latest()
        return
    data=json.loads((ENGINE/'checks.json').read_text())
    assert data['review']=='FIXES-U01-02'
    assert data['real_radius']==6 and data['circular_player_vision']
    assert data['native_cell']==64 and data['npc_rect'][2:]==[64.,64.]
    assert not data['npc_visible_before'] and data['npc_visible_after']
    assert data['movement_start']==[16,31] and data['movement_end']==[7,31] and data['movement_turns']==9
    assert data['render_read_only'] and data['camp_round_trip']
    font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',16)
    x,y,_,_=data['npc_rect']
    left=max(334,min(int(x)-160,1552-384));top=max(78,min(int(y)-128,782-288))
    box=(left,top,left+384,top+288)
    before=Image.open(ENGINE/'npc-before.png').convert('RGB').crop(box)
    after=Image.open(ENGINE/'npc-fixed.png').convert('RGB').crop(box)
    red,green,blue=ImageChops.difference(before,after).split()
    changed=sum(ImageChops.lighter(ImageChops.lighter(red,green),blue).histogram()[3:])
    assert changed>100,'The visible NPC needs a real image difference'
    board=Image.new('RGB',(796,334),'#111a1f');draw=ImageDraw.Draw(board)
    for offset,im,title in [(8,before,'Avant · PNJ masqué par le mur'),(404,after,'Corrigé · PNJ visible dans le cercle')]:
        draw.text((offset,8),title,font=font,fill='#ddc99b');board.paste(im,(offset,38))
    board.save(OUT/'r02-npc-comparaison.png')
    for name in ['street-before','street-fixed','street-path-walked','street-current','street-grim-candidate']:
        Image.open(ENGINE/f'{name}.png').crop(VIEW).save(OUT/f'r02-{name}.png')
    scene=json.loads((ROOT/'assets/maps/prison-street-expanded-01/street.json').read_text())
    source=Image.open(ROOT/scene['background']).convert('RGB')
    old=(ROOT/'assets/maps/prison-street-expanded-01/layout.txt').read_text().splitlines()
    cells=set(map(tuple,json.loads((ROOT/'assets/maps/prison-street-expanded-01/collision-repairs.json').read_text())['cells']))
    area=(6*64,30*64,17*64,33*64)
    collisions=Image.new('RGB',(1440,246),'#111a1f');d=ImageDraw.Draw(collisions)
    for offset,repaired,title in [(8,False,'Avant · mur logique invisible dans le dessin'),(728,True,'Corrigé · 9 pas vers l’ouest, 9 tours')]:
        im=source.crop(area);marks=ImageDraw.Draw(im)
        for x in range(7,17):
            walkable=old[31][x]=='.' or (repaired and (x,31) in cells)
            color='#57d8ab' if walkable else '#ef695d'
            marks.rectangle((x*64-area[0]+1,65,x*64-area[0]+62,126),outline=color,width=2)
            marks.text((x*64-area[0]+4,72),f'{x},31',fill=color)
        d.text((offset,8),title,font=font,fill='#ddc99b');collisions.paste(im,(offset,40))
    collisions.save(OUT/'r02-collisions-ligne31.png')
    board=Image.new('RGB',(650,230),'#111a1f');d=ImageDraw.Draw(board)
    ground=source.crop((22*64,31*64,23*64,31*64+130))
    for x in range(12,638,64):board.paste(ground,(x,72))
    player=Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png').convert('RGBA')
    player=player.crop(player.getchannel('A').getbbox())
    prop1=Image.open(ROOT/'assets/source-art/first-sector-review-01/u01-corrections-r01/GRIM-U01-01-native.png').convert('RGBA')
    prop2=Image.open(ROOT/'assets/source-art/first-sector-review-01/u01-corrections-r02/GRIM-U01-02-native.png').convert('RGBA')
    for im,pos in [(player,(48,196-player.height)),(prop1,(224,132)),(prop2,(492,116))]:board.paste(im,pos,im)
    for text,pos in [('Joueur',(46,208)),('GRIM-U01-01 · 128 × 64',(190,208)),('GRIM-U01-02 · 96 × 80',(430,208))]:d.text(pos,text,font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14),fill='#c3ceca')
    d.text((12,12),'Décors candidats · même échelle, sans zoom',font=font,fill='#ddc99b')
    d.text((12,38),'Sources entières et isolées · aucun nouveau décor activé',font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14),fill='#bdc9c7')
    board.save(OUT/'r02-decors-natifs.png')
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Rue · corrections et ambiance</title>
<style>body{margin:24px;background:#111a1f;color:#d4ddda;font:17px Segoe UI,sans-serif}h1,h2{color:#ddc99b}p{max-width:1100px;line-height:1.5}.scroll{overflow:auto;max-width:100%;margin:14px 0;border:1px solid #41535b}img{display:block;max-width:none}button{background:#263740;color:#dfd5bd;border:1px solid #6c7c80;padding:10px 18px;cursor:pointer}a{color:#dfc38b}section{margin:30px 0}.buttons{display:flex;gap:10px}</style>
<h1>La rue · corrections et ambiance</h1><p>FIXES-U01-02 · 19 octobre 2026 · captures du moteur à 64 px par case.</p>
<section id="vision"><h2>1 · PNJ derrière un mur</h2>
<p>Le cercle de six cases s’applique désormais aussi aux personnages. Le PNJ ci-dessous était dans le rayon mais masqué par une ancienne règle de ligne de vue. Les murs gardent leurs collisions et leur rôle dans les trajectoires des attaques ; la vision propre aux monstres ne change pas.</p>
<div class="scroll"><img src="r02-npc-comparaison.png" width="796" height="334"></div></section>
<section id="collisions"><h2>2 · Passage depuis X 16 · Y 31</h2>
<p>Le blocage est reproduit puis corrigé dans une ancienne partie : neuf déplacements successifs vers l’ouest atteignent X 7 · Y 31, avec neuf tours consommés. Les 61 corrections de rue sont maintenant actives. Les caisses, voitures, façades et limites physiques restent solides.</p>
<p>Diagnostic sur le dessin source : la grille montre les collisions avant/après, à la même échelle.</p><div class="scroll"><img src="r02-collisions-ligne31.png" width="1440" height="246"></div>
<details><summary>Capture réelle à la fin du trajet</summary><div class="scroll"><img src="r02-street-path-walked.png" width="1218" height="704"></div></details></section>
<section id="ambiance"><h2>3 · Proposition grimdark / glauque</h2>
<p>Une ville frappée par un assaut brutal, avec des traces de quarantaine, de répression et de pratiques médicales inquiétantes. La descente pourra montrer davantage de corruption organique et les horreurs du culte. L’éclairage reste lisible et les vraies sources lumineuses sont conservées.</p>
<p>GRIM-L02 : pénombre froide plus forte. GRIM-U01-01 : barricade et victimes. GRIM-U01-02 : station médicale de contrainte abandonnée. Ces nouveaux pixels et cet éclairage attendent ta validation ; ils ne sont pas actifs dans le lancement normal.</p>
<div class="buttons"><button data-src="r02-street-current.png">Rendu actuel</button><button data-src="r02-street-grim-candidate.png">Proposition grimdark</button></div>
<div class="scroll"><img id="ambience" src="r02-street-grim-candidate.png" width="1218" height="704"></div>
<div class="scroll"><img src="r02-decors-natifs.png" width="650" height="230"></div>
<p>Éclairage et accessoires calculés et dessinés dans le moteur. Même position, même caméra, mêmes tailles. Les objets sont créés entiers sur fond transparent, puis placés séparément.</p></section>
<script>document.querySelectorAll('button').forEach(b=>b.onclick=()=>document.getElementById('ambience').src=b.dataset.src);</script></html>'''
    (OUT/'index.html').write_text(page,encoding='utf-8')
    DOC.mkdir(parents=True,exist_ok=True)
    data.update(npc_comparison_changed_pixels=changed,review_page='artifacts/street-defects-review-01/index.html')
    (DOC/'capture-checks.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'review':data['review'],'npc_changed_pixels':changed,'movement_turns':9,'repairs':61,'grim_integrated':False}))

if __name__=='__main__':build()
