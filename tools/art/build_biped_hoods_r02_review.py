"""Native before/after review of the two cultist hood/head directions."""
import argparse
import re
import build_biped_armed03_review as previous


def main(page_only=False):
    if (previous.previous.base.ROOT/'artifacts/animation-polish-01/biped-lateral-03/r03/current/checks.json').exists():
        from build_biped_fronts_r03_review import main as current
        return current(page_only=page_only)
    base = previous.configure()
    base.OUT = base.ROOT/'artifacts/animation-polish-01/biped-lateral-03/r02/current'
    section = base.SECTION.replace('Bipèdes : ailes et armes à deux mains', 'Cultistes : tête orientée avec le corps')
    start = section.index('<p><strong>01 Ailé'); end = section.index('</p>', start)+4
    section = section[:start]+'''<p><strong>02 Cultiste au fusil · 03 Cultiste à la hache :</strong> reprise des têtes et capuches vers la gauche et la droite. Les cinq autres dessins restent disponibles sans modification.</p>'''+section[end:]
    section = section.replace('Les trois nouveaux modèles restent à examiner avant intégration.', 'Les deux corrections restent à examiner avant intégration.')
    section = section.replace('<option value="idle" selected>', '<option value="idle">').replace('<option value="walk">', '<option value="walk" selected>')
    section = section.replace('Face actuelle / nouvelle marche', 'Avant r01 / tête orientée r02')
    section = section.replace('Joueur de référence · monstre adapté. Taille native, cases de 64 px ; même décor et lumière du camp, traitement habituel de chaque acteur.', 'Avant r01 · tête orientée r02, mêmes phases de marche. Taille native, cases de 64 px ; même décor et même lumière.')
    section = section.replace('Joueur et monstre sur la même phase de marche', 'Cultiste avant et après correction de la tête sur la même phase')
    section = section.replace('biped-lateral-03/r01/', 'biped-lateral-03/r02/').replace('current/MON-145-0/native.webp', 'current/MON-164-0/native.webp')
    section = section.replace('Référence de démarche : Bastion lourd, cycle conservé.', 'Sur 02 et 03, comparer l’ouverture de capuche et la direction du visage entre r01 et r02 dans les deux sens. Le repos reprend le frontal habituel. Référence de démarche : Bastion lourd, cycle conservé.')
    section = section.replace('</section>', '<details><summary>Gauche et droite sur une même planche</summary><div class="native"><img src="biped-lateral-03/r02/hood-direction-summary.png" width="512" height="288" style="display:block;width:512px;height:288px;max-width:none;image-rendering:pixelated" alt="Cultistes au fusil et à la hache, gauche et droite, avant r01 et reprise r02 à taille native"></div></details></section>')
    base.SECTION = section
    script = base.SCRIPT.replace('biped-lateral-03/r01/', 'biped-lateral-03/r02/').replace('BIPED-LATERAL03-r01', 'BIPED-LATERAL03-r02')
    labels = """function bipedLabels(){const item=bipedItems.find(i=>i.id===$('biped-item').value),before=!!item.previous_walk,pair=$('biped-pair').value;$('biped-pair').querySelector('option[value="walk"]').textContent=before?'Avant r01 / tête orientée r02':'Frontal actuel / marche';$('biped-columns').textContent=pair==='idle'?'Joueur de référence · monstre adapté. Taille native, cases de 64 px.':pair==='all'?(before?'Joueur · avant r01 · tête orientée r02.':'Joueur · frontal actuel · marche.'):(before?'Avant r01 · tête orientée r02, mêmes phases. Taille native, cases de 64 px.':'Frontal actuel · marche. Taille native, cases de 64 px.');}
$('biped-pair').onchange=()=>{bipedLabels();bipedPainted='';paintBiped();};"""
    script = '\n'.join(labels if line.startswith("$('biped-pair').onchange=") else line for line in script.splitlines())
    script = script.replace("bipedReady=true;bipedPainted='';", "bipedReady=true;bipedLabels();bipedPainted='';")
    base.SCRIPT = script
    options = ['01 · Ailé d’obsidienne · précédent', '02 · Cultiste au fusil · tête r02', '03 · Cultiste à la hache · tête r02', '04 · Possédé · validé', '05 · Cultiste · validé', '06 · Démon massif · validé', '07 · Démon ailé · validé']
    base.main(page_only=page_only, labels=options, step_ms=400)
    html = base.PAGE.read_text(encoding='utf-8')
    html = html.replace('<option value="MON-164-0">', '<option value="MON-164-0" selected>')
    html = re.sub(r'<title>.*?</title>', '<title>Project RL · Orientation des cultistes · BIPED-LATERAL03-r02</title>', html, count=1)
    html = re.sub(r'(<div class="tag">).*?(</div>)', r'\g<1>BIPED-LATERAL03-r02 · TÊTES ET CAPUCHES · 400 MS\g<2>', html, count=1)
    html = html.replace('Un nouvel ailé et deux cultistes armés reprennent le cycle joueur à 400 ms par pas ; les quatre modèles validés restent comparables.', 'Les deux cultistes sont repris pour que leur tête et leur capuche suivent le sens de la marche, à 400 ms par pas.')
    base.PAGE.write_text(html, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--page-only', action='store_true'); args = parser.parse_args()
    main(page_only=args.page_only)
