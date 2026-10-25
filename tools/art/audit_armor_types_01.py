"""Audit cyberpunk equipment families and the requested 5-6 models per family."""
from collections import Counter
from pathlib import Path
import csv
import hashlib
import html
import json
import re

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'docs/reviews/class-readability-01'
OUT = ROOT / 'artifacts/class-armor-review-01'
OFFICIAL_D2 = 'https://classic.battle.net/diablo2exp/items/normal/armor.shtml'

# Classification is an editorial proposal, not a new gameplay enum or item rename.
# Keep stable review IDs for the earlier torso proposal, including archived rows.
TYPES = [
    ('AT-01', 'Ancien textile matelassé', 'core:body_armor', ['veste_matelassee'],
     'Famille historique exclue du programme : le matelassé n’est pas un manque imposé par Diablo II.', False),
    ('AT-02', 'Ancien cuir', 'core:body_armor', ['patched_plating'],
     'Tablier retiré à la demande du joueur. Le cuir n’est pas un manque à combler automatiquement.', False),
    ('AT-03', 'Textiles renforcés', 'core:body_armor', ['veste_de_veille'],
     'Vêtement technique court, renforcé et distinct des blindages rigides.', True),
    ('AT-04', 'Manteaux de protection', 'core:body_armor', ['veste_a_fibres_croisees', 'manteau_de_mue', 'veste_de_la_seconde_ombre'],
     'Coupe longue ; les matières, volumes et signes propres doivent distinguer les trois modèles.', True),
    ('AT-05', 'Blindages rigides', 'core:body_armor', ['plastron_bastion', 'armure_cerbere', 'carapace_chrysalide', 'brigandine_cognefer', 'composite_carapace'],
     'Construction segmentée rigide. Le matériau exact de Chrysalide reste à préciser.', True),
    ('AT-06', 'Combinaisons tactiques', 'core:body_armor', ['combinaison_fantome'],
     'Construction près du corps avec zones articulées ; classification visuelle sans nouveau bonus.', True),
    ('AT-07', 'Protections biomembranaires', 'core:body_armor', ['veste_de_membranes'],
     'Construction explicitement décrite comme des membranes souples superposées.', True),
    ('AT-08', 'Enveloppes symbiotiques', 'core:body_armor', ['veste_de_peau_seconde'],
     'Construction décrite comme une enveloppe vivante, à rendre lisible comme telle.', True),
    ('AH-01', 'Casques blindés', 'core:head_armor', ['calotte_cognefer', 'casque_cerbere', 'heaume_bastion'],
     'Regroupement provisoire des protections de tête rigides ; aucun matériau précis n’est déduit du nom.', True),
    ('AH-02', 'Capuches de protection', 'core:head_armor', ['capuche_de_mue'],
     'Couverture souple de la tête, distincte d’un casque et d’une visière.', True),
    ('AH-03', 'Masques et visières', 'core:head_armor', ['masque_chrysalide', 'visiere_fantome'],
     'Protections du visage regroupées par leur forme ; aucune fonction électronique nouvelle n’est supposée.', True),
    ('AG-01', 'Gantelets blindés', 'core:hand_armor', ['gantelets_bastion', 'gantelets_chrysalide'],
     'Gantelets couvrants ; l’architecture exacte des futurs modèles reste à dessiner.', True),
    ('AG-02', 'Gants techniques', 'core:hand_armor', ['gants_cognefer', 'gants_de_mue', 'gants_fantome'],
     'Gants couvrants regroupés par leur forme ; technique désigne ici une famille proposée, pas une statistique.', True),
    ('AG-03', 'Mitaines renforcées', 'core:hand_armor', ['mitaines_cerbere'],
     'Doigts partiellement découverts ; forme à distinguer des gants couvrants à 64 px.', True),
    ('AB-01', 'Bottes de protection', 'core:foot_armor', ['bottes_cerbere', 'bottes_de_mue'],
     'Bottes montantes regroupées par leur coupe ; leurs descriptions actuelles sont génériques.', True),
    ('AB-02', 'Bottines techniques', 'core:foot_armor', ['bottines_chrysalide', 'bottines_fantome', 'brodequins_cognefer'],
     'Bottines et brodequins regroupés par leur coupe ; matière et rigidité restent à confirmer visuellement.', True),
    ('AB-03', 'Blindages de pied', 'core:foot_armor', ['solerets_bastion'],
     'Protection de pied en coque proposée à partir des solerets ; sans ajout de fonction mécanique.', True),
]
RETIRED = {'core:brigandine_cognefer', 'core:composite_carapace', 'core:veste_matelassee', 'core:patched_plating'}
STARTERS = {'core:tenue_assaut', 'core:tenue_technique', 'core:tenue_traque', 'core:tenue_rituelle', 'core:tenue_metamorphe'}
SLOTS = [('core:head_armor','Tête'),('core:hand_armor','Mains'),('core:foot_armor','Pieds')]
ALL_SLOTS = [('core:body_armor', 'Torse'), *SLOTS]
TARGET_MIN, TARGET_MAX = 5, 6
STATUS_LABELS = {'generated_catalogue':'Catalogue de tirage courant', 'legacy_binding':'Définition et anciens raccords ; hors tirage commun',
                 'retired':'Retiré des nouveaux tirages', 'class_base_compatibility':'Ancienne tenue de départ / compatibilité',
                 'accessory':'Pièce séparée'}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def string_field(text, key):
    found = re.search(r'(?<!\w)(?:"'+key+r'"|'+key+r')\s*:\s*"([^"\n]+)"', text)
    return found.group(1) if found else None


def number_field(text, key):
    found = re.search(r'(?<!\w)(?:"'+key+r'"|'+key+r')\s*:\s*(-?\d+)', text)
    return int(found.group(1)) if found else None


def load_items():
    translations = {}
    sources = []
    for path in sorted((ROOT/'content/core/locales').glob('fr*.json5')):
        for key,value in re.findall(r'"([^"\n]+)"\s*:\s*("(?:[^"\\]|\\.)*")', path.read_text('utf-8')):
            translations[key] = json.loads(value)
        sources.append(path)
    catalogue_files = sorted((ROOT/'content/core/equipment_loot').glob('*.json5'))
    pooled_ids = {value for path in catalogue_files
                  for value in re.findall(r'(?:"item"|\bitem)\s*:\s*"([^"]+)"', path.read_text('utf-8'))}
    art = {}
    for path in (ROOT/'src/game_art_catalog.rs', ROOT/'src/game_art.rs'):
        text = path.read_text('utf-8')
        art.update({item:int(number) for item,number in re.findall(r'"(core:[^"]+)"\s*=>\s*ItemArt\s*\{[^}]*illustration:\s*(\d+)',text)})
        art.update({item:int(number) for item,number in re.findall(r'"(core:tenue_[^"]+)"\s*=>\s*(\d+)',text)})
    items=[]
    for path in sorted((ROOT/'content/core/items').glob('*.json5')):
        text=path.read_text('utf-8'); slot=string_field(text,'slot')
        if slot not in ('core:body_armor', *(slot for slot,_ in SLOTS)):
            continue
        item_id=string_field(text,'id'); name_key=string_field(text,'name_key')
        if item_id in STARTERS: status='class_base_compatibility'
        elif item_id in RETIRED: status='retired'
        elif slot!='core:body_armor': status='accessory'
        elif item_id in pooled_ids: status='generated_catalogue'
        else: status='legacy_binding'
        icon=ROOT/f'assets/sprites/torso-icons-01/TOR-{art.get(item_id)}-A.png'
        items.append({'id':item_id,'name':translations.get(name_key,name_key),'slot':slot,
                      'description':translations.get(string_field(text,'description_key')),
                      'status':status,'source':path.relative_to(ROOT).as_posix(),
                      'in_raw_equipment_catalogue':item_id in pooled_ids,
                      'current_common_generator_candidate':item_id in pooled_ids and item_id not in RETIRED,
                      'art_illustration':art.get(item_id),
                      'current_icon':icon.relative_to(ROOT).as_posix() if icon.exists() else None})
        sources.append(path)
    sources+=catalogue_files
    sources += [ROOT/'src/equipment_generation.rs',ROOT/'src/armor_models.rs',ROOT/'src/accessory_models.rs',
                ROOT/'src/ascii_app.rs',ROOT/'src/equipment_commerce.rs',ROOT/'src/game_art_catalog.rs',ROOT/'src/game_art.rs']
    return items,sources


def study_diablo():
    path=OUT/'diablo2-reference/patch_d2-Armor.txt'
    rows=list(csv.DictReader(path.read_text('cp1252').splitlines(),delimiter='\t'))
    torsos=[r for r in rows if r.get('type')=='tors']
    normal=[r for r in torsos if r.get('code')==r.get('normcode')]
    all_codes={r['code'] for r in torsos}
    assert len(normal)==15 and len(torsos)==45
    assert all(all(r[k] in all_codes for k in ('normcode','ubercode','ultracode')) for r in normal)
    return {'source_table':path.relative_to(ROOT).as_posix(),'source_sha256':digest(path),
            'official_reference':OFFICIAL_D2,'normal_body_bases':15,'body_records_across_three_tiers':45,
            'bases':[{k:r[k].strip() for k in ('name','code','normcode','ubercode','ultracode')} for r in normal],
            'interpretation':'Base type, progression tier, item quality and class-specific appearance are distinct dimensions.'}


def classification(items):
    by_id={i['id']:i for i in items}; groups=[]; assigned=set()
    for key,label,slot,slugs,note,production_family in TYPES:
        members=[by_id['core:'+s] for s in slugs]
        for item in members:
            assert item['id'] not in assigned
            assert item['slot'] == slot
            assigned.add(item['id']); item['proposed_type']=key
        retained=[i for i in members if i['status'] not in ('retired','class_base_compatibility')]
        groups.append({'type_id':key,'label':label,'slot':slot,'status':'proposed_classification','note':note,
                       'production_family':production_family,
                       'review_count':len(retained),'generator_count':sum(i['status']=='generated_catalogue' for i in retained),
                       'retired_count':sum(i['status']=='retired' for i in members),'items':[i['id'] for i in members],
                       'target_min':TARGET_MIN if production_family else None,
                       'target_max':TARGET_MAX if production_family else None,
                       'missing_to_min':max(0,TARGET_MIN-len(retained)) if production_family else None,
                       'missing_to_max':max(0,TARGET_MAX-len(retained)) if production_family else None})
    body=[i for i in items if i['slot']=='core:body_armor']
    assert assigned=={i['id'] for i in items if i['id'] not in STARTERS}
    counts=Counter(i['status'] for i in body)
    assert len(body)==19 and counts=={'generated_catalogue':10,'retired':4,'class_base_compatibility':5}, counts
    return groups,{**dict(counts),'legacy_binding':0}


def production_plan(groups):
    result=[]
    for slot,label in ALL_SLOTS:
        selected=[g for g in groups if g['production_family'] and g['slot']==slot]
        result.append({'slot':slot,'label':label,'families':len(selected),
                       'current_models':sum(g['review_count'] for g in selected),
                       'target_min':sum(g['target_min'] for g in selected),
                       'target_max':sum(g['target_max'] for g in selected),
                       'missing_to_min':sum(g['missing_to_min'] for g in selected),
                       'missing_to_max':sum(g['missing_to_max'] for g in selected)})
    return result


def markdown(data):
    items=data['items']; by_id={i['id']:i for i in items}; totals=data['production_totals']
    text=['# ARMOR-TYPES01-r02 — familles futuristes et objectif de modèles','',
          '21 octobre 2026 — nomenclature et regroupements proposés ; objectif de 5 à 6 modèles par type demandé par le joueur.','',
          '## Règles de comptage acceptées','',
          '- Viser **5 à 6 modèles distincts par famille et par emplacement** : torse, tête, mains et pieds.',
          '- Une couleur, une rareté, un bonus ou un palier ne constitue pas un modèle supplémentaire.',
          '- Les cinq adaptations d’une même armure aux classes ne sont pas cinq modèles. '
          'Exemple : cinq modèles de blindage de torse, chacun adapté aux cinq classes, représentent cinq objets et vingt-cinq variantes de classe.',
          '- Les armes gardent un visuel commun aux classes ; ce programme ne change pas cette décision.',
          '- Familles adaptées à l’univers industriel, cyberpunk et organique du jeu. '
          'Le cuir et le matelassé ne sont pas des catégories obligatoires à ajouter.',
          '- Le Tablier de cuir est retiré à la demande du joueur, sans réactivation ni future variante.','',
          '## Bilan du programme proposé','',
          '| Emplacement | Familles proposées | Modèles actuels | Cible totale | À ajouter |',
          '|---|---:|---:|---:|---:|']
    for plan in data['production_plan']:
        text.append(f"| {plan['label']} | {plan['families']} | {plan['current_models']} | {plan['target_min']}–{plan['target_max']} | {plan['missing_to_min']}–{plan['missing_to_max']} |")
    text += [f"| **Total** | **{totals['families']}** | **{totals['current_models']}** | **{totals['target_min']}–{totals['target_max']}** | **{totals['missing_to_min']}–{totals['missing_to_max']}** |",'',
             'Ces volumes sont conditionnés à la conservation des familles proposées ci-dessous. '
             'Ils ne comprennent pas les variantes de classe, animations, directions ou couleurs. '
             'Les modèles existants doivent encore passer la revue de lisibilité à 64 × 64.','']
    for slot,label in ALL_SLOTS:
        text += [f'## {label} — listing par famille proposé','',
                 '| Réf. | Famille proposée | Existants | Cible | À ajouter | Modèles actuels |',
                 '|---|---|---:|---:|---:|---|']
        for group in data['types']:
            if not group['production_family'] or group['slot']!=slot:
                continue
            kept=[by_id[i]['name'] for i in group['items'] if by_id[i]['status'] not in ('retired','class_base_compatibility')]
            text.append(f"| {group['type_id']} | {group['label']} | {group['review_count']} | 5–6 | {group['missing_to_min']}–{group['missing_to_max']} | {', '.join(kept)} |")
        text += ['']
    text += ['## Arbitrages à faire avant de dessiner toute la gamme','',
             'Les familles sont une proposition de classement, pas de nouvelles catégories mécaniques déjà intégrées. '
             'Les descriptions des casques, gants et bottes sont actuellement génériques : les regroupements reposent '
             'sur leur nom et leur coupe présumée. Ils restent à confirmer avec les dessins. '
             'La matière de Chrysalide n’est pas établie par ses statistiques.','',
             '- **Manteaux** : Nomade, mue et Seconde Ombre doivent conserver des coupes et matières reconnaissables.',
             '- **Blindages** : Bastion, Cerbère et Chrysalide comptent comme trois modèles d’une famille proposée, pas trois familles à remplir séparément.',
             '- **Biomembranes et symbiotes** : deux familles proposées car une superposition de membranes et une enveloppe vivante décrivent deux constructions. '
             'Si cette différence se révèle trop fine à 64 px, les regrouper réduira le programme.',
             '- **Accessoires** : masques/visières et bottines/brodequins sont regroupés pour éviter de multiplier les familles sur de simples noms.',
             '- Aucun groupe ne dépasse actuellement six modèles. Il n’y a donc aucun modèle à supprimer pour excès de quantité dans ce classement.',
             '- Les futurs modèles doivent varier par leur forme, leur découpe et leur construction, pas seulement par leur couleur. '
             'Exosquelettes portés ou tenues industrielles spécialisées restent des pistes facultatives, pas des familles ajoutées à ce total. '
             'Le corps robotique du Technomancien n’est pas une armure équipée.','',
             '## Exclusions et compatibilité','',
             '**37 définitions** ont été inventoriées : 19 torses et 6 pièces pour chacun des trois autres emplacements. '
             '**28 modèles sont retenus** pour ce programme : 10 torses et 18 accessoires.','',
             '**4 torses retirés** : Tenue Cognefer, Armure Azurite, Tenue d’aviateur et Tablier de cuir. '
             'Leurs anciennes définitions restent disponibles pour les sauvegardes historiques. '
             '**5 anciennes tenues de départ** sont conservées uniquement pour la compatibilité ; '
             'les classes actuelles commencent sans objet de torse équipé.','',
             'Les anciennes lignes AT-01 et AT-02 sont archivées ; elles ne créent pas une obligation '
             'de produire du cuir ou du matelassé. Le retrait du Tablier est enregistré dans `leather-apron-retirement.json`. '
             'Le retrait d’un torse ne supprime pas les accessoires qui portent le même nom de gamme.','',
             '## Inventaire détaillé et identifiants conservés','',
             '| Objet | Famille proposée | Statut | Identifiant |',
             '|---|---|---|---|']
    for item in items:
        text.append(f"| {item['name']} | {item.get('proposed_type','Ancienne base de classe')} | {STATUS_LABELS[item['status']]} | `{item['id']}` |")
    text += ['','## Référence et contrôle','',
             'Diablo II sert de référence pour distinguer base, modèle, palier et apparence par classe. '
             'Ses noms médiévaux ne définissent pas la nomenclature du projet. La table locale étudiée contient '
             '15 bases normales de torse et 45 lignes à travers trois paliers ; '
             'la [liste officielle](https://classic.battle.net/diablo2exp/items/normal/armor.shtml) reste une référence documentaire.','',
             'Inventaire machine : `armor-types.json`. Script reproductible : `tools/art/audit_armor_types_01.py`. '
             'Les comptages et empreintes des sources y sont enregistrés. Le script ne modifie aucun contenu du jeu. '
             'Le seul changement de règles de cette phase est le retrait explicitement demandé du Tablier '
             'pour les nouvelles parties, avec préservation des anciennes sauvegardes. '
             'Les familles proposées ne valident ni nouveaux noms d’objets, ni statistiques, ni dessins à intégrer.','']
    return '\n'.join(text)


def section(data):
    by_id={i['id']:i for i in data['items']}; esc=html.escape
    totals=data['production_totals']; overview=[]; tables=[]
    for plan in data['production_plan']:
        overview.append(f"<tr><td>{plan['label']}</td><td>{plan['families']}</td><td>{plan['current_models']}</td><td>{plan['target_min']}–{plan['target_max']}</td><td>{plan['missing_to_min']}–{plan['missing_to_max']}</td></tr>")
        rows=[]
        for group in data['types']:
            if not group['production_family'] or group['slot']!=plan['slot']:
                continue
            names=', '.join(by_id[i]['name'] for i in group['items'] if by_id[i]['status'] not in ('retired','class_base_compatibility'))
            rows.append(f"<tr><td><small>{group['type_id']}</small><br>{esc(group['label'])}</td><td>{group['review_count']}</td><td>5–6</td><td>{group['missing_to_min']}–{group['missing_to_max']}</td><td>{esc(names)}</td></tr>")
        tables.append(f"<h3>{plan['label']} · {plan['families']} familles proposées</h3><div class='type-table'><table><thead><tr><th>Famille proposée</th><th>Existants</th><th>Cible</th><th>À ajouter</th><th>Modèles actuels</th></tr></thead><tbody>"+''.join(rows)+"</tbody></table></div>")
    overview.append(f"<tr><td><strong>Total proposé</strong></td><td>{totals['families']}</td><td>{totals['current_models']}</td><td>{totals['target_min']}–{totals['target_max']}</td><td>{totals['missing_to_min']}–{totals['missing_to_max']}</td></tr>")
    detail=[]
    for group in data['types']:
        if not group['production_family']:
            continue
        images=[]
        for item_id in group['items']:
            item=by_id[item_id]
            if item['status'] in ('retired','class_base_compatibility'):
                continue
            image=f'<img src="../../{item["current_icon"]}" width="64" height="64" alt="{esc(item["name"])}">' if item['current_icon'] else ''
            images.append(f'<div class="type-item">{image}<div><strong>{esc(item["name"])}</strong><br><small>{esc(STATUS_LABELS[item["status"]])}</small></div></div>')
        detail.append(f'<details><summary>{group["type_id"]} · {esc(group["label"])}</summary><p class="note">{esc(group["note"])}</p>'+''.join(images)+'</details>')
    return '''<section id="armor-types"><style>
#armor-types{scroll-margin-top:20px}#armor-types table{border-collapse:collapse;width:100%;font-size:14px}#armor-types th,#armor-types td{padding:10px 8px;border-bottom:1px solid #425058;text-align:left;vertical-align:top}#armor-types th{color:#eddfc4;background:#29373d}#armor-types td:nth-child(2),#armor-types td:nth-child(3),#armor-types td:nth-child(4){font-weight:700}#armor-types small{color:#b5c6c9}#armor-types details{border:1px solid #34464c;padding:12px;margin:8px 0;border-radius:5px}#armor-types summary{cursor:pointer;color:#e6c895}.type-item{display:flex;align-items:center;gap:15px;padding:7px}.type-item img{image-rendering:pixelated;background:#343b3f;flex:none}.type-counts{padding:14px;background:#29373d;border-left:3px solid #c4a76c}.type-table{overflow-x:auto}.type-table table{min-width:620px}#armor-types .type-overview table{min-width:0}@media(max-width:600px){#armor-types .type-overview th,#armor-types .type-overview td{padding:9px 4px;font-size:12px}}
</style><h2>Armures · familles futuristes et modèles</h2>
<p><strong>Objectif demandé : 5 à 6 modèles différents par type, pour chaque emplacement.</strong> Les familles ci-dessous sont proposées pour notre univers cyberpunk, industriel et organique.</p>
<p class="type-counts"><strong>28 modèles existants retenus · 15 familles proposées</strong><br>Si ces familles sont conservées : <strong>75 à 90 modèles au total</strong>, donc <strong>47 à 62 à ajouter</strong>.<br>Couleurs, raretés et cinq adaptations de classe ne comptent pas comme des modèles supplémentaires.</p>
<div class="type-table type-overview"><table><thead><tr><th>Emplacement</th><th>Familles</th><th>Existants</th><th>Cible totale</th><th>À ajouter</th></tr></thead><tbody>'''+''.join(overview)+'''</tbody></table></div>
<p class="note">Une famille n’est pas une gamme nommée : Bastion, Cerbère et Chrysalide sont ici trois modèles de blindage, pas trois familles. Les 15 familles restent à revoir ; ce total n’est pas une liste de nouveaux objets déjà validés.</p>
'''+''.join(tables)+'''
<p class="note">Les accessoires ont des descriptions génériques : leur classement par coupe reste provisoire. Aucun type ne dépasse six modèles actuellement. Les nouvelles formes devront se distinguer à 64 × 64, au-delà d’une simple recoloration.</p>
<p>Le cuir et le matelassé ne sont plus présentés comme des manques obligatoires. Les anciennes lignes AT-01 et AT-02 sont archivées.</p>
<p class="note">Le Tablier de cuir est retiré à ta demande. Il ne sera pas décliné sur les cinq classes. Ses anciens fichiers sont conservés pour la compatibilité ; aucun autre modèle n’est supprimé par ce listing.</p>
<details><summary>Voir les modèles et leurs icônes existantes</summary>'''+''.join(detail)+'''</details>
<p><a href="../../docs/reviews/class-readability-01/ARMOR_TYPES.md">Listing complet, objectifs et statuts</a> · <a href="../../docs/reviews/class-readability-01/armor-types.json">Inventaire détaillé</a></p></section>'''


def main():
    items,sources=load_items(); groups,counts=classification(items)
    # Validate the exact retirements against live Rust declarations, not old review labels.
    retired_code=(ROOT/'src/equipment_generation.rs').read_text('utf-8')
    retired_section=retired_code.split('const RETIRED_CORE_ITEMS:',1)[1].split('];',1)[0]
    assert all(item in retired_section for item in RETIRED if item not in ('core:veste_matelassee','core:patched_plating'))
    assert 'RETIRED_AVIATOR: &str = "core:veste_matelassee"' in retired_code
    assert 'RETIRED_LEATHER_APRON: &str = "core:patched_plating"' in retired_code
    assert len(items)==37 and all(sum(i['slot']==slot for i in items)==6 for slot,_ in SLOTS)
    hashes={p.relative_to(ROOT).as_posix():digest(p) for p in sources}
    plan=production_plan(groups)
    totals={key:sum(p[key] for p in plan) for key in ('families','current_models','target_min','target_max','missing_to_min','missing_to_max')}
    assert totals=={'families':15,'current_models':28,'target_min':75,'target_max':90,'missing_to_min':47,'missing_to_max':62}
    assert all(g['review_count']<=TARGET_MAX for g in groups if g['production_family'])
    data={'review_id':'ARMOR-TYPES01','revision':'r02','date':'2026-10-21','status':'proposed_classification_user_target_accepted',
          'scope':'Cyberpunk physical equipment families for torso, head, hands and feet; explicit retirement of the leather apron, no other removals.',
          'accepted_user_targets':{'models_per_type_min':TARGET_MIN,'models_per_type_max':TARGET_MAX,
              'applies_to_slots':[slot for slot,_ in ALL_SLOTS],
              'class_adaptations_count_as_additional_models':False,
              'palette_rarity_or_tier_counts_as_additional_model':False,
              'medieval_diablo_type_names_required':False,
              'type_names_and_membership':'proposed_not_integrated'},
          'counts':{'all_armor_definitions':37,'body_definitions':19,**counts,
                    'body_models_for_review':10,'present_physical_body_groups_proposed':6,'head':6,'hands':6,'feet':6},
          'types':groups,'production_plan':plan,'production_totals':totals,'items':items,'diablo2':study_diablo(),
          'source_hashes':hashes,'validation':{'all_nonstarter_item_ids_classified_once':True,
          'retained_plus_legacy_plus_retired_plus_starters_equal_body_total':True,
          'all_retained_models_counted_once_in_production_plan':True,
          'target_range_consistent_for_all_active_families':True,
          'source_files_unchanged_during_inventory_run':True,'inventory_script_modified_game_files':False}}
    REVIEW.mkdir(parents=True,exist_ok=True); OUT.mkdir(parents=True,exist_ok=True)
    (REVIEW/'armor-types.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf-8')
    (REVIEW/'ARMOR_TYPES.md').write_text(markdown(data),'utf-8')
    (OUT/'armor-types-section.html').write_text(section(data),'utf-8')
    assert all(digest(ROOT/path)==value for path,value in hashes.items())
    print(json.dumps({'counts':data['counts'],'production_totals':totals},ensure_ascii=False))


if __name__=='__main__':
    main()
