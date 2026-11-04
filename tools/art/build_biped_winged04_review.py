"""Three original winged identities and the accepted red demon at native scale."""
import argparse
import re
import build_biped_armed03_review as previous


def main(page_only=False):
    base = previous.configure()
    base.OUT = base.ROOT/'artifacts/animation-polish-01/biped-lateral-04/r01/current'
    section = base.SECTION.replace('Bipèdes : ailes et armes à deux mains', 'Trois nouveaux ailés · frontal conservé')
    start = section.index('<p><strong>01 Ailé'); end = section.index('</p>', start)+4
    section = section[:start]+'''<p><strong>01 Petit démon ailé · 02 Ailé doré · 03 Ailé pâle au brassard :</strong> trois marches proposées depuis leurs dessins actuels. <strong>04 Démon ailé rouge :</strong> référence déjà validée, inchangée.</p>'''+section[end:]
    section = section.replace('Le joueur et le monstre sont synchronisés.', 'Le frontal actuel reste conservé à l’identique et sert de référence d’apparence.')
    section = section.replace('Joueur / monstre même cycle', 'Référence du jeu / marche').replace('Face actuelle / nouvelle marche', 'Frontal conservé / nouvelle marche')
    section = section.replace('Joueur de référence · monstre adapté. Taille native, cases de 64 px ; même décor et lumière du camp, traitement habituel de chaque acteur.', 'Frontal conservé · nouvelle marche. Taille native, cases de 64 px ; même position et même lumière du camp.')
    section = section.replace('Joueur et monstre sur la même phase de marche', 'Frontal conservé et marche du démon à taille native')
    section = section.replace('width="320"', 'width="384"').replace('width:320px', 'width:384px').replace('width="640"', 'width="768"').replace('width:640px', 'width:768px')
    section = section.replace('biped-lateral-03/r01/', 'biped-lateral-04/r01/').replace('current/MON-145-0/native.webp', 'current/MON-039-0/native.webp')
    start = section.index('<p class="note">Référence de démarche :'); end = section.index('</p>', start)+4
    section = section[:start]+'''<p class="note">Les huit phases reprennent la démarche du joueur, avec des appuis au sol et des ailes portées. Chaque frontal d’origine reste inchangé, y compris au retour au repos. Le petit démon conserve son petit gabarit ; le doré sa carrure ; le pâle son unique brassard sur le même bras. Les panneaux plus larges laissent de la place aux ailes sans agrandir les acteurs.</p>'''+section[end:]
    section = section.replace('</section>', '''<details><summary>Les trois candidats : face et marche dans les deux sens</summary><div class="native"><img src="biped-lateral-04/r01/winged-summary.png" width="768" height="576" style="display:block;width:768px;height:576px;max-width:none;image-rendering:pixelated" alt="Trois ailés : référence du jeu, frontal conservé et nouvelles marches gauche et droite à taille native"></div></details><p class="note">Lot précédent validé : <a href="biped-lateral-03/r04/current/MON-164-0/native.webp">cultiste au fusil</a> · <a href="biped-lateral-03/r04/current/MON-165-0/native.webp">cultiste à la hache</a>.</p></section>''')
    base.SECTION = section
    script = base.SCRIPT.replace('biped-lateral-03/r01/', 'biped-lateral-04/r01/').replace('BIPED-LATERAL03-r01', 'BIPED-LATERAL04-r01')
    script = script.replace('columns.length*160*scale', 'columns.length*192*scale').replace('source*160,0,160,192,col*160*scale,0,160*scale', 'source*192,0,192,192,col*192*scale,0,192*scale')
    labels = """function bipedLabels(){const item=bipedItems.find(i=>i.id===$('biped-item').value),front=!!item.identity_front,pair=$('biped-pair').value;$('biped-pair').querySelector('option[value="idle"]').textContent=front?'Référence du jeu / marche':'Joueur / monstre même cycle';$('biped-pair').querySelector('option[value="walk"]').textContent=front?'Frontal conservé / nouvelle marche':'Frontal actuel / marche validée';$('biped-columns').textContent=pair==='all'?(front?'Référence du jeu · frontal conservé · nouvelle marche.':'Joueur · frontal actuel · marche validée.'):pair==='walk'?(front?'Frontal conservé · nouvelle marche.':'Frontal actuel · marche validée.'):(front?'Référence du jeu · nouvelle marche.':'Joueur · monstre validé.');$('biped-columns').textContent+=' Taille native, cases de 64 px.';}
$('biped-pair').onchange=()=>{bipedLabels();bipedPainted='';paintBiped();};"""
    script = '\n'.join(labels if line.startswith("$('biped-pair').onchange=") else line for line in script.splitlines())
    script = script.replace("bipedReady=true;bipedPainted='';", "$('biped-pair').value=item.identity_front?'walk':'idle';bipedReady=true;bipedLabels();bipedPainted='';")
    base.SCRIPT = script
    base.main(page_only=page_only, labels=['01 · Petit démon ailé · candidat', '02 · Ailé doré · candidat', '03 · Ailé pâle au brassard · candidat', '04 · Démon ailé rouge · validé'], step_ms=400)
    html = base.PAGE.read_text(encoding='utf-8')
    html = html.replace('Trois classes actuelles, armures lourdes sans arme : étude du contact au sol et du passage de la jambe.', 'Sept modèles validés. Trois nouveaux ailés à examiner à taille native, dans les deux sens.', 1)
    html = re.sub(r'<title>.*?</title>', '<title>Project RL · Trois nouveaux ailés · BIPED-LATERAL04-r01</title>', html, count=1)
    html = re.sub(r'(<div class="tag">).*?(</div>)', r'\g<1>BIPED-LATERAL04-r01 · TROIS AILÉS · 400 MS\g<2>', html, count=1)
    for old in ['Les marches de 02 et 03 reprennent leur frontal retenu. Les cinq autres modèles sont validés et conservés, à 400 ms par pas.', 'Un nouvel ailé et deux cultistes armés reprennent le cycle joueur à 400 ms par pas ; les quatre modèles validés restent comparables.']:
        html = html.replace(old, 'Trois nouveaux ailés conservent leur frontal actuel et reprennent la démarche validée à 400 ms par pas.')
    base.PAGE.write_text(html, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(); parser.add_argument('--page-only', action='store_true'); args = parser.parse_args()
    main(page_only=args.page_only)
