"""Compare the preferred walk identity with its own frontal idle."""
import argparse
import re
from pathlib import Path
import build_biped_armed03_review as previous


def main(page_only=False):
    if (Path(__file__).resolve().parents[2]/'artifacts/animation-polish-01/biped-lateral-03/r04/current/checks.json').exists():
        from build_biped_frontmatch_r04_review import main as current
        return current(page_only=page_only)
    base = previous.configure()
    base.OUT = base.ROOT/'artifacts/animation-polish-01/biped-lateral-03/r03/current'
    section = base.SECTION.replace('Bipèdes : ailes et armes à deux mains', 'Cultistes : le même dessin de face et en marche')
    start = section.index('<p><strong>01 Ailé'); end = section.index('</p>', start)+4
    section = section[:start]+'''<p><strong>02 Cultiste au fusil · 03 Cultiste à la hache · 05 Cultiste sans arme :</strong> le frontal reprend la pose de repos dessinée avec leur marche. Les phases de marche sont conservées exactement. Les quatre autres modèles gardent leurs dessins.</p>'''+section[end:]
    section = section.replace('Les trois nouveaux modèles restent à examiner avant intégration.', 'Examiner surtout le retour au repos : même tenue, même capuche et même arme, les deux pieds au sol.')
    section = section.replace('Le joueur et le monstre sont synchronisés.', 'Le frontal et la marche sont comparés à la même position.')
    section = section.replace('<option value="idle" selected>', '<option value="idle">').replace('<option value="walk">', '<option value="walk" selected>')
    section = section.replace('Face actuelle / nouvelle marche', 'Frontal harmonisé / marche')
    section = section.replace('Joueur de référence · monstre adapté. Taille native, cases de 64 px ; même décor et lumière du camp, traitement habituel de chaque acteur.', 'Frontal harmonisé · marche conservée. Taille native, cases de 64 px ; même position, même décor et même lumière.')
    section = section.replace('Joueur et monstre sur la même phase de marche', 'Frontal harmonisé et marche du cultiste dans le même décor')
    section = section.replace('biped-lateral-03/r01/', 'biped-lateral-03/r03/').replace('current/MON-145-0/native.webp', 'current/MON-014-0/native.webp')
    note_start = section.index('<p class="note">Référence de démarche:') if '<p class="note">Référence de démarche:' in section else section.index('<p class="note">Référence de démarche :')
    note_end = section.index('</p>', note_start)+4
    section = section[:note_start]+'''<p class="note">Sur 02, 03 et 05, la face affichée à gauche appartient désormais au même dessin que la marche à droite. À l’arrêt, la marche rejoint ce frontal avec les deux pieds posés. « Les trois colonnes » conserve aussi l’ancien frontal pour comparer. La cadence reste de 400 ms par pas ; les autres palettes et l’intégration au jeu gardent leur revue distincte.</p>'''+section[note_end:]
    section = section.replace('</section>', '<details><summary>Ancienne face, nouvelle face et deux sens de marche</summary><div class="native"><img src="biped-lateral-03/r03/front-identity-summary.png" width="640" height="576" style="display:block;width:640px;height:576px;max-width:none;image-rendering:pixelated" alt="Trois cultistes : ancien frontal, frontal issu de la marche, marche gauche et marche droite à taille native"></div></details></section>')
    base.SECTION = section
    script = base.SCRIPT.replace('biped-lateral-03/r01/', 'biped-lateral-03/r03/').replace('BIPED-LATERAL03-r01', 'BIPED-LATERAL03-r03')
    labels = """function bipedLabels(){const item=bipedItems.find(i=>i.id===$('biped-item').value),front=!!item.identity_front,pair=$('biped-pair').value;$('biped-pair').querySelector('option[value="idle"]').textContent=front?'Ancien frontal / marche':'Joueur / monstre même cycle';$('biped-pair').querySelector('option[value="walk"]').textContent=front?'Frontal harmonisé / marche':'Frontal actuel / marche';$('biped-columns').textContent=pair==='all'?(front?'Ancien frontal · frontal harmonisé · marche conservée. Taille native, cases de 64 px.':'Joueur · frontal actuel · marche. Taille native, cases de 64 px.'):pair==='idle'?(front?'Ancien frontal · marche et nouveau repos. Taille native, cases de 64 px.':'Joueur de référence · monstre adapté. Taille native, cases de 64 px.'):(front?'Frontal harmonisé · marche conservée. Taille native, cases de 64 px.':'Frontal actuel · marche. Taille native, cases de 64 px.');}
$('biped-pair').onchange=()=>{bipedLabels();bipedPainted='';paintBiped();};"""
    script = '\n'.join(labels if line.startswith("$('biped-pair').onchange=") else line for line in script.splitlines())
    base.SCRIPT = script.replace("bipedReady=true;bipedPainted='';", "bipedReady=true;bipedLabels();bipedPainted='';")
    base.SCRIPT = base.SCRIPT.replace("$('biped-animation').href=", "if(item.identity_front)$('biped-scope').textContent='Créature '+item.id+' · frontal proposé · marche conservée · 400 ms par pas.';$('biped-animation').href=")
    options = ['01 · Ailé d’obsidienne · précédent', '02 · Cultiste au fusil · frontal r03', '03 · Cultiste à la hache · frontal r03', '04 · Possédé · validé', '05 · Cultiste sans arme · frontal r03', '06 · Démon massif · validé', '07 · Démon ailé · validé']
    base.main(page_only=page_only, labels=options, step_ms=400)
    html = base.PAGE.read_text(encoding='utf-8').replace('<option value="MON-014-0">', '<option value="MON-014-0" selected>')
    html = re.sub(r'<title>.*?</title>', '<title>Project RL · Frontal cohérent des cultistes · BIPED-LATERAL03-r03</title>', html, count=1)
    html = re.sub(r'(<div class="tag">).*?(</div>)', r'\g<1>BIPED-LATERAL03-r03 · MÊME DESSIN DE FACE ET EN MARCHE · 400 MS\g<2>', html, count=1)
    html = html.replace('Les deux cultistes sont repris pour que leur tête et leur capuche suivent le sens de la marche, à 400 ms par pas.', 'Les trois cultistes conservent le dessin de leur marche pour leur nouveau frontal et leur retour au repos, à 400 ms par pas.')
    html = html.replace('Les deux cultistes conservent le dessin de leur marche pour leur nouveau frontal et leur retour au repos, à 400 ms par pas.', 'Les trois cultistes conservent le dessin de leur marche pour leur nouveau frontal et leur retour au repos, à 400 ms par pas.')
    base.PAGE.write_text(html, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--page-only', action='store_true'); args = parser.parse_args()
    main(page_only=args.page_only)
