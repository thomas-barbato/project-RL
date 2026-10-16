"""Versioned before/after gallery with native and doubled views."""
from pathlib import Path
BASE=Path(__file__).resolve().parent
OUT=BASE/'armor-flat-review-03'
html=(BASE/'armor-flat-review-01/index.html').read_text('utf-8')
html=html.replace('ARMOR-FLAT01','ARMOR-FLAT03')
html=html.replace('Armures · gabarit frontal partagé','Armures · corrections du catalogue')
html=html.replace('Les 17 tenues actuelles et six nouvelles propositions, chacune avec trois palettes. Les casques, gants et bottes sont présentés séparément.',
 'AR-08 : casque cuivré. AR-10 et AR-11 : nouvelles palettes. AR-12 : technicien redessiné. AR-N14 : artefacts jaunes corrigés.')
html=html.replace('Utilise les numéros AR-… pour désigner les modèles à conserver, modifier ou supprimer. Les nouvelles propositions attendent ton avis.',
 'Les huit nouvelles tenues AR-N07 à AR-N14 sont validées. La correction des taches jaunes de l’armure très lourde est ajoutée à cette revue ; sa silhouette reste identique.')
html=html.replace('<span>17 tenues actuelles</span><span>6 propositions</span><span>18 pièces séparées</span>',
 '<span>5 corrections</span><span>16 prises d’équipement</span><span>Avant / après</span>')
html=html.replace('Les prérequis d’attributs des armures seront définis plus tard. Une variante de couleur conservera les mêmes statistiques et prérequis. Les casques, gants et bottes gardent une seule illustration.',
 'Même gabarit frontal, mêmes positions de mains et mêmes placements d’armes. Les noms et les identifiants des quatre objets sont conservés.')
start=html.index('<label>Afficher<select');end=html.index('</select></label>',start)+len('</select></label>')
html=html[:start]+'''<label>Afficher<select id="filter"><option value="all">Les cinq corrections</option></select></label>'''+html[end:]
start=html.index('const categories=');end=html.index(';',start)+1
html=html[:start]+"const categories={revision:'Correction proposée'};"+html[end:]
html=html.replace('<div class="grid" id="cards"></div>',
 '<h2>Comparaison · avant et après</h2><p class="muted">Armure très lourde : variante rouge comparée ci-dessous. Les vues 32 et 64 px exactes et les autres palettes sont présentées sur chaque fiche.</p><img src="audit/overview.png" alt="Cinq armures, avant et après, numérotées" style="display:block;width:100%;height:auto;border:1px solid #344655;margin-bottom:24px"><div class="grid" id="cards"></div>')
html=html.replace('Ancien dessin du catalogue','Revue précédente · même gabarit')
html=html.replace('Proposition frontale · configuration actuelle','Correction · configuration actuelle')
html=html.replace("e.old_review_id+' · identité de contenu conservée'","(e.old_review_id||e.id)+' · identifiant de revue conservé'")
html=html.replace('<div class="controls">','<p><a href="#AR-N14">Voir les variantes corrigées de l’armure très lourde</a></p><div class="controls">')
html=html.replace('localPalettes=new Map;','localPalettes=new Map([["AR-N14",2]]);')
html=html.replace('<footer><p>', '<footer><p><a href="http://127.0.0.1:59142/index.html">Huit nouvelles tenues validées</a> · <a href="http://127.0.0.1:59141/index.html">Catalogue précédent</a></p><p>')
(OUT/'index.html').write_text(html,'utf-8')
print(OUT/'index.html')
