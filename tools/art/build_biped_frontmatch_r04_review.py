"""Two revised armed walks, their retained fronts, and five frozen models."""
import argparse
import re
import build_biped_armed03_review as previous


def main(page_only=False):
    if (previous.previous.base.ROOT/'artifacts/animation-polish-01/biped-lateral-04/r01/current/checks.json').exists():
        from build_biped_winged04_review import main as current
        return current(page_only=page_only)
    base = previous.configure()
    base.OUT = base.ROOT/'artifacts/animation-polish-01/biped-lateral-03/r04/current'
    section = base.SECTION.replace('Bipèdes : ailes et armes à deux mains', '02 et 03 : la marche reprend le frontal retenu')
    start = section.index('<p><strong>01 Ailé'); end = section.index('</p>', start)+4
    section = section[:start]+'''<p><strong>02 Cultiste au fusil · 03 Cultiste à la hache :</strong> la marche est reprise depuis leur frontal retenu, conservé à l’identique. <strong>01/04/05/06/07 :</strong> modèles validés, dessins inchangés.</p>'''+section[end:]
    section = section.replace('Le joueur et le monstre sont synchronisés.', 'Le frontal retenu est comparé au même endroit avec la nouvelle marche.')
    section = section.replace('Les trois nouveaux modèles restent à examiner avant intégration.', 'Examiner la continuité de carrure, de capuche et de port d’arme. Les deux nouvelles marches restent en revue.')
    section = section.replace('Joueur / monstre même cycle', 'Frontal retenu / nouvelle marche').replace('Face actuelle / nouvelle marche', 'Marche r03 / marche r04')
    section = section.replace('Joueur de référence · monstre adapté. Taille native, cases de 64 px ; même décor et lumière du camp, traitement habituel de chaque acteur.', 'Frontal retenu · marche r04 proposée. Taille native, cases de 64 px ; même décor, même position et même lumière.')
    section = section.replace('Joueur et monstre sur la même phase de marche', 'Frontal retenu et nouvelle marche du cultiste à taille native')
    section = section.replace('biped-lateral-03/r01/', 'biped-lateral-03/r04/').replace('current/MON-145-0/native.webp', 'current/MON-164-0/native.webp')
    note_start = section.index('<p class="note">Référence de démarche :'); note_end = section.index('</p>', note_start)+4
    section = section[:note_start]+'''<p class="note">02 et 03 : les deux faces retenues restent exactement celles de r03. La nouvelle marche reprend leur silhouette et leur équipement. Le fusil et la hache gardent deux prises pendant les pas. « Marche r03 / marche r04 » compare les deux cycles ; les trois colonnes ajoutent le frontal retenu. Le repos utilise la même face depuis les deux directions. Les cinq autres modèles sont validés ; l’intégration normale reste distincte.</p>'''+section[note_end:]
    section = section.replace('</section>', '<details><summary>Frontal retenu et marches dans les deux sens</summary><div class="native"><img src="biped-lateral-03/r04/frontmatch-summary.png" width="640" height="384" style="display:block;width:640px;height:384px;max-width:none;image-rendering:pixelated" alt="Cultistes 02 et 03 : frontal retenu, marche précédente, nouvelle marche gauche et droite à taille native"></div></details></section>')
    base.SECTION = section
    script = base.SCRIPT.replace('biped-lateral-03/r01/', 'biped-lateral-03/r04/').replace('BIPED-LATERAL03-r01', 'BIPED-LATERAL03-r04')
    labels = """function bipedLabels(){const item=bipedItems.find(i=>i.id===$('biped-item').value),basis=!!item.retained_front_review,front=!!item.identity_front,pair=$('biped-pair').value;$('biped-pair').querySelector('option[value="idle"]').textContent=basis?'Frontal retenu / nouvelle marche':front?'Ancien frontal / marche':'Joueur / monstre même cycle';$('biped-pair').querySelector('option[value="walk"]').textContent=basis?'Marche r03 / marche r04':front?'Frontal validé / marche':'Frontal actuel / marche';$('biped-columns').textContent=basis?(pair==='all'?'Frontal retenu · marche r03 · marche r04 proposée. Taille native, cases de 64 px.':pair==='walk'?'Marche r03 · marche r04 proposée, mêmes phases. Taille native, cases de 64 px.':'Frontal retenu · marche r04 proposée. Taille native, cases de 64 px.'):(pair==='all'?(front?'Ancien frontal · frontal validé · marche validée.':'Joueur · frontal actuel · marche validée.'):pair==='walk'?(front?'Frontal validé · marche validée.':'Frontal actuel · marche validée.'):front?'Ancien frontal · marche validée.':'Joueur · monstre validé.');}
$('biped-pair').onchange=()=>{bipedLabels();bipedPainted='';paintBiped();};"""
    script = '\n'.join(labels if line.startswith("$('biped-pair').onchange=") else line for line in script.splitlines())
    script = script.replace("bipedReady=true;bipedPainted='';", "$('biped-pair').value=item.retained_front_review?'idle':item.identity_front?'walk':'idle';bipedReady=true;bipedLabels();bipedPainted='';")
    script = script.replace("$('biped-animation').href=", "$('biped-scope').textContent='Créature '+item.id+' · '+(item.retained_front_review?'frontal retenu · marche r04 proposée':'modèle validé')+' · 400 ms par pas.';$('biped-animation').href=")
    base.SCRIPT = script
    options = ['01 · Ailé d’obsidienne · validé', '02 · Cultiste au fusil · marche r04', '03 · Cultiste à la hache · marche r04', '04 · Possédé · validé', '05 · Cultiste sans arme · validé', '06 · Démon massif · validé', '07 · Démon ailé · validé']
    base.main(page_only=page_only, labels=options, step_ms=400)
    html = base.PAGE.read_text(encoding='utf-8').replace('<option value="MON-164-0">', '<option value="MON-164-0" selected>')
    html = re.sub(r'<title>.*?</title>', '<title>Project RL · Marches fidèles au frontal · BIPED-LATERAL03-r04</title>', html, count=1)
    html = re.sub(r'(<div class="tag">).*?(</div>)', r'\g<1>BIPED-LATERAL03-r04 · FRONTAL RETENU, MARCHE REPRISE · 400 MS\g<2>', html, count=1)
    html = html.replace('Les trois cultistes conservent le dessin de leur marche pour leur nouveau frontal et leur retour au repos, à 400 ms par pas.', 'Les marches de 02 et 03 reprennent leur frontal retenu. Les cinq autres modèles sont validés et conservés, à 400 ms par pas.')
    base.PAGE.write_text(html, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--page-only', action='store_true'); args = parser.parse_args()
    main(page_only=args.page_only)
