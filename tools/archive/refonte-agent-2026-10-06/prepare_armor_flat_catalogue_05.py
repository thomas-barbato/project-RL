"""Consolidate existing review images without altering any painting or pose."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import shutil

from PIL import Image

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
OUT = BASE / "armor-flat-review-05"
OUT.mkdir(exist_ok=True)


def read(name):
    return json.loads((BASE / name / "review.json").read_text("utf-8"))


reviews = {number: read(f"armor-flat-review-{number:02}") for number in (1, 2, 4)}
removed = {"AR-02", "AR-04", "AR-13"}
corrected = {e["id"] for e in reviews[4]["entries"]}
accepted = json.loads(
    (BASE / "armor-flat-review-02/visual-acceptance-01.json").read_text("utf-8")
)
correction_receipt=OUT/'visual-acceptance-01.json'
accepted_corrections=set(json.loads(correction_receipt.read_text('utf-8'))['accepted_visual_ids']) if correction_receipt.exists() else set()
accepted['accepted_visual_ids']=sorted(set(accepted['accepted_visual_ids'])|accepted_corrections)
entries = {}
origins = {}
for number in (1, 2, 4):
    for original in reviews[number]["entries"]:
        if original["id"] in removed:
            continue
        entry = deepcopy(original)
        entry["origin_review"] = f"ARMOR-FLAT{number:02}"
        entry["origin_directory"] = f"armor-flat-review-{number:02}"
        for key in ("previous", "source"):
            if entry.get(key):
                entry[key] = f"../armor-flat-review-{number:02}/{entry[key]}"
        for view in entry["views"].values():
            for key, relative in list(view.items()):
                view[key] = f"../armor-flat-review-{number:02}/{relative}"
        if entry["id"] in corrected and entry['id'] not in accepted_corrections:
            entry["review_state"] = "corrected"
        elif entry["id"] in accepted["accepted_visual_ids"]:
            entry["review_state"] = "accepted"
        else:
            entry["review_state"] = "pending"
        if entry.get("canonical"):
            entry["review_note"] = "Référence validée · variantes à examiner"
        elif entry["id"] == "AR-N06":
            entry["review_note"] = "Qualité appréciée · palettes à examiner"
        entries[entry["id"]] = entry
        origins[entry["id"]] = (number, original)


def order(entry):
    ident = entry["id"]
    group = 2 if ident.startswith("AR-P") else 1 if ident.startswith("AR-N") else 0
    return group, int(ident.split("-")[1].lstrip("NP"))


entries = sorted(entries.values(), key=order)
outfits = [e for e in entries if e["kind"] == "outfit"]
pieces = [e for e in entries if e["kind"] != "outfit"]
assert (len(outfits), len(pieces)) == (28, 18)
assert {e["id"] for e in entries}.isdisjoint(removed)
for data in reviews.values():
    assert data["common_rig"] == reviews[1]["common_rig"]
    assert data["weapon_transforms"] == reviews[1]["weapon_transforms"]

manifest = {
    "review_id": "ARMOR-FLAT05",
    "status": "review_only",
    "scope": "Consolidated numbered catalogue; existing exact review images",
    "entries": entries,
    "configurations": reviews[1]["configurations"],
    "common_rig": reviews[1]["common_rig"],
    "weapon_transforms": reviews[1]["weapon_transforms"],
    "removed_from_future_catalogue": sorted(removed),
    "corrections_pending_visual_acceptance": sorted(corrected-accepted_corrections),
    "accepted_visual_ids": accepted["accepted_visual_ids"],
    "active_game_content_changed": False,
}
(OUT / "review.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2), "utf-8")
(OUT / "review-data.js").write_text(
    "window.armorReview=" + json.dumps(manifest, ensure_ascii=False) + ";", "utf-8"
)
shutil.copy2(BASE / "armor-flat-review-01/floor32.png", OUT / "floor32.png")

# Every composition still points to its original image. No image is repainted,
# rescaled or recomposed when preparing this consolidated catalogue.
references = {}
native_checks = 0
for entry in entries:
    expected = 48 if entry["kind"] == "outfit" else 1
    assert len(entry["views"]) == expected
    number, original = origins[entry["id"]]
    for key, view in entry["views"].items():
        for size, relative in view.items():
            path = (OUT / relative).resolve()
            assert path == (BASE / f"armor-flat-review-{number:02}" / original["views"][key][size]).resolve()
            assert path.is_file(), path
            references[path.relative_to(ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
        native = Image.open(OUT / view["native32"]).convert("RGBA")
        doubled = Image.open(OUT / view["display64"]).convert("RGBA")
        assert doubled.tobytes() == native.resize(
            (native.width * 2, native.height * 2), Image.Resampling.NEAREST
        ).tobytes()
        assert native.size == ((64, 64) if entry["kind"] == "outfit" else (32, 32))
        native_checks += 1

for relative, expected in accepted["frozen_files"].items():
    assert hashlib.sha256((BASE / "armor-flat-review-02" / relative).read_bytes()).hexdigest() == expected
for relative, expected in reviews[4]["protected_active_files"].items():
    assert hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == expected

validation = {
    "review_id": "ARMOR-FLAT05",
    "outfits": len(outfits),
    "inventory_pieces": len(pieces),
    "palettes_per_outfit": 3,
    "configurations_per_outfit": 16,
    "exact_nearest_doubling_checked": native_checks,
    "original_image_references": len(references),
    "paintings_and_compositions_changed": False,
    "accepted_flat02_frozen_files_verified": len(accepted["frozen_files"]),
    "active_art_files_verified": len(reviews[4]["protected_active_files"]),
    "common_rig_and_weapon_transforms_unchanged": True,
    "active_game_content_changed": False,
}
(OUT / "validation.json").write_text(json.dumps(validation, indent=2), "utf-8")
(OUT / "source-image-hashes.json").write_text(json.dumps(references, indent=2), "utf-8")

html = (BASE / "armor-flat-review-01/index.html").read_text("utf-8")
html = html.replace("ARMOR-FLAT01", "ARMOR-FLAT05")
html = html.replace("Armures · gabarit frontal partagé", "Armures · catalogue complet")
html = html.replace(
    "Les 17 tenues actuelles et six nouvelles propositions, chacune avec trois palettes. Les casques, gants et bottes sont présentés séparément.",
    "Les 28 tenues conservées et proposées, avec trois couleurs chacune. Retrouve ici les départs légers, les robes, les armures de combat et les dernières corrections.",
)
html = html.replace(
    "Utilise les numéros AR-… pour désigner les modèles à conserver, modifier ou supprimer. Les nouvelles propositions attendent ton avis.",
    "Les numéros restent identiques aux revues précédentes. Les huit modèles AR-N07 à AR-N14 sont validés ; les corrections signalées restent à examiner.",
)
html = html.replace(
    '<span>17 tenues actuelles</span><span>6 propositions</span><span>18 pièces séparées</span>',
    '<span>28 tenues</span><span>18 pièces séparées</span><span>6 corrections</span>',
)
html = html.replace(
    '<label>Afficher<select id="filter">',
    '<label>Afficher<select id="filter"><option value="corrected">Les six corrections</option><option value="starters">Les cinq départs légers</option><option value="progression">Les trois armures de progression</option>',
)
html = html.replace('<option value="all">Toutes les tenues</option>', '<option value="all" selected>Toutes les tenues</option>')
html = html.replace('Tenues actuelles · AR-01 à AR-17', 'Tenues conservées · AR-01 à AR-17')
html = html.replace(
    '<label>Configuration<select id="config">',
    '<label>Revue<select id="state"><option value="all">Tous les états</option><option value="accepted">Dessins déjà validés</option><option value="corrected">Corrections à examiner</option><option value="pending">Autres dessins à examiner</option></select></label><label>Configuration<select id="config">',
)
html = html.replace(
    '<label>Fond<select id="background">',
    '<label>Présentation<select id="presentation"><option value="large">Grands dessins et tuiles</option><option value="tiles">Tuiles 32 et 64 px</option></select></label><label>Fond<select id="background">',
)
html = html.replace('</style>', '.tiles-only .portrait{display:none}</style>')
html = html.replace('loading="lazy" class="native', 'class="native')
html = html.replace(
    '<div class="grid" id="cards"></div>',
    '<p class="muted">Retirées à ta demande : AR-02 · Tenue Cognefer, AR-04 · Carapace composite, AR-13 · Tenue d’aviateur.</p><div class="grid" id="cards"></div>',
)
html = html.replace("current:'Modèle actuel'", "current:'Tenue conservée'")
html = html.replace(
    "const kinds=",
    "Object.assign(categories,{new_starter:'Départ léger',new_progression:'Progression',quality_reference:'Métamorphe · référence de qualité',revision:'Correction'});\nconst reviewStates={accepted:'Dessin validé',corrected:'Correction à examiner',pending:'Dessin à examiner'};\nconst kinds=",
)
start = html.index("let entries=data.entries.filter(")
end = html.index(";", start) + 1
html = html[:start] + """const groups={new_mage:['AR-N01','AR-N02','AR-N10','AR-N12'],new_combat:['AR-N03','AR-N04','AR-N07','AR-N13','AR-N14'],metamorphe_replacement:['AR-N05','AR-N06','AR-N11'],progression:['AR-N12','AR-N13','AR-N14']};
let entries=data.entries.filter(e=>filter==='everything'||filter==='all'&&e.kind==='outfit'||filter==='corrected'&&e.review_state==='corrected'||filter==='starters'&&e.category==='new_starter'||filter==='current'&&!e.id.startsWith('AR-N')&&e.kind==='outfit'||filter==='new'&&e.id.startsWith('AR-N')||filter==='current_piece'&&e.kind!=='outfit'||groups[filter]?.includes(e.id));""" + html[end:]
html = html.replace(
    "entries=entries.filter(e=>`${e.id}",
    "entries=entries.filter(e=>$('state').value==='all'||e.review_state===$('state').value);\nentries=entries.filter(e=>`${e.id}",
)
html = html.replace(
    "${e.replaces_candidate?",
    "<p class=\"muted\">${esc(e.kind==='outfit'?(e.review_note||reviewStates[e.review_state]):'Illustration d’inventaire · une seule couleur')}</p>\n${e.replaces_candidate?",
)
html = html.replace('Alternative à AR-03 · tenue de départ du Métamorphe', 'Alternative visuelle à AR-03')
html = html.replace("for(const id of ['filter','config'])", "for(const id of ['filter','state','config'])")
html = html.replace(
    "$('search').addEventListener('input',render);",
    "$('presentation').addEventListener('change',()=>document.querySelector('main').classList.toggle('tiles-only',$('presentation').value==='tiles'));\n$('search').addEventListener('input',render);",
)
html = html.replace("e.old_review_id+' · identité de contenu conservée'", "(e.old_review_id||e.id)+' · '+reviewStates[e.review_state]")
html = html.replace(
    '<a href="generation-specs.json">Prompts des dessins</a>',
    '<a href="source-image-hashes.json">Sources conservées</a>',
)
# Keep deep links stable even when a filter previously hid the requested card.
html = html.replace(
    "$('close').addEventListener('click',()=>$('comparison').close());render();",
    "$('close').addEventListener('click',()=>$('comparison').close());render();\nfunction revealHash(){const id=decodeURIComponent(location.hash.slice(1));if(!data.entries.some(e=>e.id===id))return;$('filter').value='everything';$('state').value='all';$('search').value='';render();requestAnimationFrame(()=>$(id).scrollIntoView({block:'start'}));}window.addEventListener('hashchange',revealHash);revealHash();",
)
(OUT / "index.html").write_text(html, "utf-8")
(OUT / "README.md").write_text(
    "# ARMOR-FLAT05\n\nCatalogue consolidé, préparé après la reprise du travail sur les armures.\n\n"
    "28 tenues, trois palettes et seize configurations d’équipement par tenue. "
    "Les 18 pièces de tête, mains et pieds gardent une seule illustration. "
    "AR-02, AR-04 et AR-13 sont absentes du catalogue futur à la demande du joueur.\n\n"
    "Les images sont référencées depuis leur revue d’origine, sans modification ni recopie. "
    "Les six corrections de ARMOR-FLAT04 sont conservées exactement. "
    "L’accord sur la transmogrification ne vaut pas validation de ces corrections.\n\n"
    "Cette étape n’intègre pas de nouveaux modèles, statistiques, noms d’objets ou prérequis. "
    "Les repères de progression restent ceux des propositions visuelles.\n\n"
    "Servir le dossier parent `artifacts/refonte-agent-2026-10-06`, puis ouvrir "
    "`armor-flat-review-05/index.html`. Les références `../armor-flat-review-…` "
    "font partie du catalogue et doivent rester accessibles.\n",
    "utf-8",
)
print(json.dumps(validation), flush=True)
