"""Reuse the review controls; labels remain explicitly non-canon."""
from pathlib import Path

BASE=Path(__file__).resolve().parent
OUT=BASE/'armor-flat-review-02'
html=(BASE/'armor-flat-review-01/index.html').read_text('utf-8')
html=html.replace('ARMOR-FLAT01','ARMOR-FLAT02')
html=html.replace('Armures · gabarit frontal partagé','Armures · des silhouettes distinctes')
html=html.replace('Les 17 tenues actuelles et six nouvelles propositions, chacune avec trois palettes. Les casques, gants et bottes sont présentés séparément.',
 'Cinq départs légers, trois armures plus avancées et le Manteau de mutation conservé comme référence de qualité. Trois palettes par tenue.')
html=html.replace('Utilise les numéros AR-… pour désigner les modèles à conserver, modifier ou supprimer. Les nouvelles propositions attendent ton avis.',
 'Les nouveaux dessins AR-N07 à AR-N14 attendent ton avis. Chaque tenue conserve les poses et les placements d’armes validés ; ses formes et ses matières changent.')
html=html.replace('<span>17 tenues actuelles</span><span>6 propositions</span><span>18 pièces séparées</span>',
 '<span>5 départs légers</span><span>3 armures de progression</span><span>AR-N06 intact</span>')
start=html.index('<label>Afficher<select');end=html.index('</select></label>',start)+len('</select></label>')
html=html[:start]+'''<label>Afficher<select id="filter"><option value="all">Toutes les silhouettes</option><option value="new_starter">Les cinq départs légers</option><option value="new_progression">Les trois armures de progression</option><option value="quality_reference">Manteau de mutation · référence conservée</option></select></label>'''+html[end:]
start=html.index('const categories=');end=html.index(';',start)+1
html=html[:start]+"const categories={new_starter:'Départ léger proposé',new_progression:'Progression proposée',quality_reference:'Qualité appréciée'};"+html[end:]
html=html.replace("${e.replaces_candidate?'<p class=\"muted\">Alternative à AR-03 · tenue de départ du Métamorphe</p>':''}",
 "${e.category==='quality_reference'?'<p class=\"muted\">Dessin conservé intact · référence de qualité</p>':''}")
html=html.replace('<div class="grid" id="cards"></div>',
 '<h2>Vue d’ensemble · huit propositions et la référence</h2><p class="muted">Illustrations agrandies ci-dessous. Les vues exactes en 32 et 64 px se trouvent sur chaque fiche.</p><img src="audit/silhouettes.png" alt="Neuf silhouettes d’armures numérotées" style="display:block;width:100%;height:auto;border:1px solid #344655;margin-bottom:24px"><div class="grid" id="cards"></div>')
html=html.replace('<footer><p>', '<footer><p><a href="http://127.0.0.1:59141/index.html">Revue précédente · catalogue complet</a></p><p>')
html=html.replace('Les prérequis d’attributs des armures seront définis plus tard. Une variante de couleur conservera les mêmes statistiques et prérequis. Les casques, gants et bottes gardent une seule illustration.',
 'Les familles et les libellés sont proposés pour la revue. Leurs prérequis et statistiques seront définis plus tard. Les casques, gants et bottes séparés gardent une seule illustration.')
(OUT/'index.html').write_text(html,'utf-8')
print(OUT/'index.html')
