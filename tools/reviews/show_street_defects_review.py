"""Native review of street defects; actual GPU frames, explicit collision list."""
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/street-defects-review-01'
ENGINE = OUT/'engine'
DOC = ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-defects-review-01'
ART = ROOT/'assets/source-art/first-sector-review-01/u01-corrections-r01'
VIEW = (334,78,1552,782)


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def build():
    if (OUT/'engine08/checks.json').exists():
        from show_street_civilians_review import build as latest
        latest()
        return
    if (OUT/'engine06/checks.json').exists():
        from show_street_paving_review import build as latest
        latest()
        return
    if (OUT/'engine04/checks.json').exists():
        from show_street_map_style_review import build as latest
        latest()
        return
    if (OUT/'engine03/checks.json').exists():
        from show_street_style_review import build as latest
        latest()
        return
    if (OUT/'engine02/checks.json').exists():
        from show_world_fixes_review import build as latest
        latest()
        return
    accepted = (DOC/'vision-acceptance.json').exists()
    checks=read(ENGINE/'checks.json')
    assert checks['review']=='DEFECTS-U01-01' and checks['defects_review']
    assert checks['native_cell']==64 and checks['camp_round_trip']
    assert all(r['render_read_only'] and r['cell'][2:]==[64.,64.] for r in checks['scenes'])
    labels={'join-middle':'Rue centrale','gate':'Sortie de prison','court-clean':'Cour',
            'house-a-closed':'Logement fermé','door-hidden':'Porte fermée hors de la ligne de vue des acteurs',
            'house-a-open':'Logement ouvert','house-b-closed':'Boutique fermée',
            'house-b-open':'Boutique ouverte','west-corner':'Angle de l’enceinte'}
    assert {r['scene'] for r in checks['scenes']}==set(labels)
    modes=[('', '12 cases · actuel'),('-cercle8','8 cases · proposé'),
           ('-cercle6','6 cases · proposé'),('-sombre','6 cases + ambiance sombre')]
    if accepted:
        modes=[('', '12 cases · ancien rayon'),('-cercle8','8 cases · comparaison'),
               ('-cercle6','6 cases · comparaison initiale'),('-sombre','Ambiance sombre · proposition')]
    cards=[]
    for name,title in labels.items():
        buttons=[]
        for suffix,label in modes:
            image=Image.open(ENGINE/f'{name}{suffix}.png').convert('RGB').crop(VIEW)
            image.save(OUT/f'{name}{suffix}-vue.png')
            digest=hashlib.sha256(image.tobytes()).hexdigest()[:12]
            buttons.append(f'<button data-src="{name}{suffix}-vue.png?v={digest}">{label}</button>')
        cards.append(f'<section><h2>{title}</h2><div class="buttons">'+''.join(buttons)+f'</div><div class="scroll"><img class="frame" src="{name}-sombre-vue.png" width="1218" height="704"></div></section>')
    font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',18)
    frames=[]
    for suffix,title in [('', 'Actuel · 12 cases'),('-sombre','Proposition · 6 cases et ambiance plus sombre')]:
        frame=Image.new('RGB',(1242,750),'#111a1f');d=ImageDraw.Draw(frame)
        d.text((12,10),title,font=font,fill='#ddc99b')
        frame.paste(Image.open(OUT/f'join-middle{suffix}-vue.png'),(12,40));frames.append(frame)
    frames[0].save(OUT/'vision-comparaison.gif',save_all=True,append_images=frames[1:],duration=1600,loop=0)
    # Same world, same camera and lighting; only the historical door predicate changes.
    before=Image.open(ENGINE/'door-hidden-avant.png').convert('RGB')
    after=Image.open(ENGINE/'door-hidden.png').convert('RGB')
    red,green,blue=ImageChops.difference(before,after).split()
    maximum=ImageChops.lighter(ImageChops.lighter(red,green),blue)
    changed=sum(maximum.histogram()[3:])
    assert changed>500,'The door regression comparison must show the missing panel'
    door=Image.new('RGB',(664,226),'#111a1f');d=ImageDraw.Draw(door)
    for x,image,title in [(12,before,'Ancienne condition : panneau absent'),(340,after,'Corrigé : porte fermée visible')]:
        d.text((x,10),title,font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',15),fill='#ddc99b')
        door.paste(image.crop((448,562,768,742)),(x,40))
    door.save(OUT/'porte-corrigee.png')
    cells=read(ART/'collision-proposal.json')['cells'];assert len(cells)==61
    scene=read(ROOT/'assets/maps/prison-street-expanded-01/street.json')
    surface=Image.open(ROOT/scene['background']).convert('RGB')
    collision=Image.new('RGB',(1328,308),'#111a1f');d=ImageDraw.Draw(collision)
    for offset,color,title in [(12,'#ef695d','Actuel · rouge : bloqué'),(676,'#65d9ad','Proposé · vert : passage')]:
        crop=surface.crop((320,384,960,640));cd=ImageDraw.Draw(crop)
        for x,y in cells:
            if 320<=x*64<960 and 384<=y*64<640:
                cd.rectangle((x*64-320+1,y*64-384+1,x*64-320+62,y*64-384+62),outline=color,width=2)
                cd.text((x*64-315,y*64-379),f'{x},{y}',fill=color)
        d.text((offset,10),title,font=font,fill='#ddc99b');collision.paste(crop,(offset,40))
    collision.save(OUT/'collisions-comparaison.png')
    all_cells=surface.copy();ad=ImageDraw.Draw(all_cells)
    for x,y in cells:
        ad.rectangle((x*64+1,y*64+1,x*64+62,y*64+62),outline='#53dfad',width=2)
        ad.text((x*64+5,y*64+5),f'{x},{y}',fill='#53dfad')
    all_cells.save(OUT/'collisions-carte-complete.png')
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>DEFECTS-U01-01</title>
<style>body{margin:24px;background:#111a1f;color:#d4ddda;font:17px Segoe UI,sans-serif}h1,h2,h3{color:#ddc99b}p{max-width:1100px;line-height:1.5}.scroll{overflow:auto;max-width:100%;margin:12px 0;border:1px solid #41535b}img{display:block;max-width:none}button{background:#263740;color:#dfd5bd;border:1px solid #6c7c80;padding:9px 15px;cursor:pointer}a{color:#dfc38b}section{margin:30px 0}.buttons{display:flex;gap:8px;flex-wrap:wrap}</style>
<h1>Reprise de la rue · DEFECTS-U01-01</h1>
<p><strong>Portes :</strong> le panneau fermé ne dépend plus de la visibilité réservée aux acteurs. Le correctif est actif en jeu. Les autres éléments ci-dessous sont des propositions à examiner.</p>
<h2>1 · Cercle et ambiance</h2>
<p>Le rayon actuel de 12 cases dépasse le cadre à 1920 × 1080. Les boutons comparent 12, 8 et 6 cases à la même position, sans zoom. Dans les propositions à 8 et 6, le décor hors du cercle est masqué pour garder une limite nette ; la mémoire de la mini-carte est conservée. La version sombre conserve les vraies lumières ; seule l’ambiance passe à RGB [0,28 ; 0,30 ; 0,36].</p>
<p>Ces captures comparent le masque visuel. La perception des acteurs reste à 12 pendant l’aperçu ; choisir une nouvelle portée réelle changera aussi la distance à laquelle le joueur voit les ennemis. Ce choix reste à valider.</p>'''+''.join(cards)+'''
<h2>2 · Porte fermée visible depuis la rue</h2>
<p>Les deux captures utilisent exactement le même état de partie et le même éclairage. La porte est fermée, hors de la ligne de vue des acteurs. L’ancienne condition de rendu faisait disparaître son panneau.</p><div class="scroll"><img src="porte-corrigee.png" width="664" height="226"></div>
<h2>3 · Passages proposés · COL-U01-01</h2>
<p>61 cases de rue sont proposées à la réouverture. Les débords d’images n’imposent plus un mur sur une case entière. Les vraies caisses et les gros obstacles restent bloqués. La grille de partie normale reste inchangée jusqu’à validation de cette liste.</p>
<p>Diagnostic sur les dessins sources, sans éclairage : même zone continue avant/après ; seuls les marqueurs montrent la différence de collision.</p><div class="scroll"><img src="collisions-comparaison.png" width="1328" height="308"></div>
<details><summary>Voir les 61 cases sur la carte complète, à taille native</summary><div class="scroll" style="height:700px"><img src="collisions-carte-complete.png" width="1664" height="4224"></div></details>
<h2>4 · Un premier signe de violence · GRIM-U01-01</h2>
<p>Barricade de quarantaine détruite avec victimes et sang. Objet original dessiné entier sur fond transparent avec le skill imagegen, puis conditionné à 128 × 64 px. La planche compare son échelle à celle du joueur. Candidat non intégré.</p><img src="GRIM-U01-01-native.png" width="560" height="220">
<p>La direction proposée renforce les conséquences de l’assaut et la répression carcérale. Les signes de corruption organique peuvent devenir plus présents dans les secteurs profonds.</p>
<script>document.querySelectorAll('button[data-src]').forEach(b=>b.onclick=()=>b.closest('section').querySelector('.frame').src=b.dataset.src);</script></html>'''
    if accepted:
        current=read(OUT/'vision-six-integration/checks.json')
        assert current['generation']==158 and current['player_vision_radius']==6
        assert current['six_cell_vision'] and current['camp_round_trip']
        assert len(current['scenes'])==6
        assert all(r['render_read_only'] and r['cell'][2:]==[64.,64.] for r in current['scenes'])
        image=Image.open(OUT/'vision-six-integration/join-middle.png').convert('RGB').crop(VIEW)
        image.save(OUT/'vision-six-native.png')
        digest=hashlib.sha256(image.tobytes()).hexdigest()[:12]
        integrated=f'''<h2>1 · VISION-U01-05 · Six cases validées et intégrées</h2>
<p>Le joueur a validé « 6 cases c’est bien ». La portée réelle et le cercle utilisent maintenant six cases. La couronne douce garde visibles les bords des cases perçues ; elle ne révèle aucun ennemi au-delà de la portée. La mémoire de la mini-carte et la vision propre aux monstres sont conservées.</p>
<p>Capture du lancement normal en génération 158, sans réglage de revue ni nouvel éclairage. Les parties de l’agent déjà commencées adoptent aussi six cases après vérification de leur sauvegarde, sans consommer de tour.</p>
<div class="scroll"><img src="vision-six-native.png?v={digest}" width="1218" height="704"></div>
<details><summary>Revoir les comparaisons initiales et la proposition d’ambiance plus sombre</summary>'''
        page=page.replace('<h2>1 · Cercle et ambiance</h2>', integrated)
        page=page.replace('Le rayon actuel de 12 cases', 'L’ancien rayon de 12 cases')
        page=page.replace('Ce choix reste à valider.', 'Cette comparaison précède la validation des six cases ; le nouvel éclairage reste à valider.')
        page=page.replace('<h2>2 · Porte fermée visible depuis la rue</h2>', '</details><h2>2 · Porte fermée visible depuis la rue</h2>')
    (OUT/'index.html').write_text(page,encoding='utf-8')
    report=dict(review='DEFECTS-U01-01',scenes=len(checks['scenes']),native_cell=64,
                door_fix_in_normal_game=True,door_same_state_changed_pixels=changed,
                collision_proposed_cells=61,collision_normal_game_changed=False,
                radius_levels=[12,8,6],normal_radius=6 if accepted else 12,new_ambience_normal_game_enabled=False,
                new_scenery_integrated=False,world_render_read_only=True,camp_round_trip=True)
    if accepted:
        report.update(vision_accepted=True,vision_generation=158,vision_native_captures=6,
                      existing_agent_runs_adopt_six_after_verification=True)
    (DOC/'checks.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report))


if __name__=='__main__':
    build()
