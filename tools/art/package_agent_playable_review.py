"""Replace the shared review page with one compact native-game animation.

Input: temporary output of --ui-cold-agent-spell-playable. Raw QA captures
are removed separately after inspection, never retained as gallery copies.
"""
import argparse
import json
from pathlib import Path

from PIL import Image


def replace_text(path, text):
    temporary = path.with_name(path.name + ".agent-tmp")
    assert not temporary.exists()
    temporary.write_text(text, encoding="utf-8")
    temporary.replace(path)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("raw", type=Path)
    parser.add_argument("--weapons-only", action="store_true")
    parser.add_argument("--mixed-only", action="store_true")
    parser.add_argument("--integration-only", action="store_true")
    parser.add_argument("--blast-only", action="store_true")
    parser.add_argument("--fireball-only", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    base = root / "artifacts/refonte-agent-2026-10-06"
    output = base / "skill-playable-review-01"
    checks = json.loads((args.raw / ("fireball-checks.json" if args.fireball_only else "blast-checks.json" if args.blast_only else "checks.json")).read_text(encoding="utf-8"))
    if args.fireball_only:
        assert checks["review"]=="AGENT-PLAY05" and checks["victims"]>=2
        assert checks["caster_unharmed"] and checks["preview_free"] and checks["preview_hidden_after_confirmation"]
    elif args.blast_only:
        assert checks["review"]=="AGENT-PLAY04" and checks["cells"]==9
        assert len(checks["damage"])==3 and checks["caster_unharmed"]
        assert checks["preview_free"] and checks["preview_hidden_after_confirmation"]
    elif args.integration_only:
        assert checks["review"]=="AGENT-PLAY02" and checks["damage"]>0
        drone=json.loads((args.raw/"drone-checks.json").read_text(encoding="utf-8"))
        assert drone["review"]=="AGENT-PLAY03" and drone["real_companions"]==3
        assert drone["taunted_targets"]>0 and drone["previews_free"]
        checks={"review":"AGENT-PLAY02-03","melee":checks,"drones":drone}
    elif args.mixed_only:
        assert checks["review"] == "AGENT-MIXED01"
        assert checks["actual_command_slots"] == [0, 1] and checks["ammunition_spent"] == 1
    elif args.weapons_only:
        assert checks["weapon_switch_cycle"] == [1, 2, 3, 1]
        assert checks["weapon_switch_without_time_or_resources"]
    else:
        assert checks["casts_without_waiting"] == 2 and checks["preview_hidden_after_confirmation"]
    sequence = [("skills.png", 1600)]
    sequence += [(f"weapon-switch-{i}.png", 850) for i in range(3)]
    if args.fireball_only:
        sequence=[("fireball-hover-0.png",650),("fireball-keyboard.png",650),
                  ("fireball-hover-1.png",650),("fireball-hover-2.png",650),
                  ("fireball-aim.png",650)] if checks.get("targeting_review") in ("AGENT-AIM01", "AGENT-AIM02") else [("fireball-aim.png",1000)]
        sequence += [(f"fireball-{i}.png",ms) for i,ms in enumerate([90,120,120,40,80,120,160,140,120,400])]
        sequence += [(f"fireball-burn-{i}.png",125) for i in range(8)]
        sequence += [("fireball-skill.png",2000)]
    elif args.blast_only:
        sequence=[("blast-aim.png",1000)]
        sequence += [(f"blast-{i}.png",ms) for i,ms in enumerate([50,70,100,100,100,100,130,1000])]
        sequence += [("blast-skill.png",1600)]
    elif args.integration_only:
        sequence=[("rupture-aim.png",1000)]
        sequence += [(f"rupture-{i}.png",ms) for i,ms in enumerate([60,80,80,120,150,170,200,700])]
        sequence += [(f"drone-{i}.png",1300 if i in [5,6] else 850) for i in range(7)]
    elif args.mixed_only:
        sequence = [(f"native-preview-{i}.png", 1700) for i in range(3)]
    elif not args.weapons_only:
        sequence += [("aim.png", 1000)]
        sequence += [(f"cast-{i:02}.png", 20) for i in range(28)]
        sequence += [("cast-27.png", 450)]
        sequence += [(f"cast-{i:02}.png", 20) for i in range(28, 56)]
        sequence += [("cast-55.png", 1100)]
    frames = [Image.open(args.raw / name).convert("RGB") for name, _ in sequence]
    assert all(frame.size == (1280, 800) for frame in frames)
    movie = output / "trait-arcanique.webp"
    temporary_movie = movie.with_suffix(".agent-tmp.webp")
    assert not temporary_movie.exists()
    frames[0].save(temporary_movie, save_all=True, append_images=frames[1:],
                   duration=[ms for _, ms in sequence], loop=0, lossless=True, method=3)
    temporary_movie.replace(movie)
    replace_text(output / "checks.json", json.dumps(checks, ensure_ascii=False, indent=2) + "\n")
    page = """<!doctype html>
<html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · Barre de compétences</title><style>
:root{color-scheme:dark;font-family:system-ui,sans-serif;background:#0b0d10;color:#e4e4e9}
body{max-width:1280px;margin:auto;padding:28px 20px}h1{font-size:26px;margin:0 0 10px}
p{line-height:1.65;max-width:1000px}.note{color:#a9acb9;font-size:14px}
.keys{color:#c6b4ff}img{display:block;max-width:100%;height:auto;border:1px solid #30313d;margin-top:24px}
li{margin:8px 0;line-height:1.6}code{color:#c6b4ff}a{color:#baa0ff}
</style></head><body><h1>Barre de compétences</h1>
<p class="note">AGENT-BAR01 · Six emplacements partagés entre le jeu et l’arbre de compétences.</p>
<p>Glisse l’icône d’une compétence active disponible dans un emplacement.
Un clic sur la barre en jeu prépare son utilisation ; dans l’arbre, il sélectionne sa fiche.
Un clic droit retire le raccourci. Les emplacements sont sauvegardés.</p>
<p class="keys"><strong>1–6 : compétences · 7–9 : armes</strong> — raccourcis personnalisables dans les options.</p>
<p>En jeu, le bouton à gauche de la barre, avec une flèche circulaire, passe à l’arme suivante.
Il ignore les emplacements vides, sans dépenser de tour.</p>
<p>Une compétence donnée par un objet équipé peut aussi être placée dans la barre.
Retirer sa dernière source équipée retire le raccourci, sauf si la compétence est apprise définitivement.</p>
<p class="note">Cet aperçu montre la barre partagée et le changement d’arme en jeu.
A01 est le premier effet du nouveau catalogue raccordé au combat ; les autres suivront par lots.
Les objets accordant de nouvelles compétences seront définis lors de leur création.</p>
<img src="skill-playable-review-01/trait-arcanique.webp" width="1280" height="800"
alt="Arbre de compétences, puis changement d’arme entre les trois emplacements équipés">
<p class="note">Pour essayer sans toucher à ta partie : lance <code>Voir_trait_arcanique.cmd</code> dans le dossier du projet.</p>
</body></html>
"""
    if args.mixed_only:
        page = page.replace("<h1>Barre de compétences</h1>", "<h1>Mêlée et distance dans les deux mains</h1>")
        page = page.replace("AGENT-BAR01 · Six emplacements partagés entre le jeu et l’arbre de compétences.", "AGENT-MIXED01 · Choix automatique de l’arme dans la configuration active.")
        page = page.replace("Cet aperçu montre la barre partagée et le changement d’arme en jeu.", "Cet aperçu montre trois états réels du jeu : avant l’attaque, après le coup au contact, puis après le tir à distance. Une action par attaque, une seule munition consommée au total.")
        page = page.replace("A01 est le premier effet du nouveau catalogue raccordé au combat ; les autres suivront par lots.", "Les compétences choisissent l’arme du type requis parmi les deux mains actives. Les effets du nouveau catalogue restent intégrés par lots.")
        page = page.replace("<p>Une compétence donnée", "<p><strong>Avec une arme de mêlée et une arme à distance à une main :</strong> l’attaque ordinaire frappe au contact et tire plus loin. Les compétences utilisent automatiquement la main compatible. L’arme à deux mains en réserve reste exclue.</p>\n<p>Une compétence donnée")
        page = page.replace("Voir_trait_arcanique.cmd", "Voir_armes_mixtes.cmd")
        page = page.replace("Arbre de compétences, puis changement d’arme entre les trois emplacements équipés", "Paire d’armes à une main : coup au contact puis tir à distance sans changement manuel")
        page = page.replace("trait-arcanique.webp\"", "trait-arcanique.webp?review=AGENT-MIXED01\"")
    if args.integration_only:
        page = """<!doctype html><html lang="fr"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · Compétences jouables</title><style>
:root{color-scheme:dark;font-family:system-ui,sans-serif;background:#0b0d10;color:#e4e4e9}
body{max-width:1280px;margin:auto;padding:26px 20px}h1{font-size:26px}p{line-height:1.6;max-width:1000px}
img{display:block;max-width:100%;height:auto;border:1px solid #30313d}a{color:#baa0ff}.note{color:#a9acb9}
code{color:#c6b4ff}</style></head><body><h1>Frappe de rupture et drones de combat</h1>
<p class="note">AGENT-PLAY02 / AGENT-PLAY03 · Captures natives du vrai moteur, cases de 64 px.</p>
<p>Frappe de rupture inflige réellement ses dégâts. Traqueur, Artilleur et Gardien sont de vrais compagnons persistants : ils suivent le joueur et combattent pendant leurs phases normales.
La barre, le bouton Utiliser et le menu clic droit passent par la même exécution.</p>
<p>Le Gardien provoque dans un carré de 5 × 5 cases arrêté par les obstacles, pendant 2 tours.
Sa provocation remplace son attaque et possède une réutilisation interne de 4 tours.
Une cible déjà provoquée conserve son Gardien et sa durée ; le symbole au-dessus de sa tête indique l’effet.</p>
<img src="skill-playable-review-01/trait-arcanique.webp?review=AGENT-PLAY02-03" width="1280" height="800" alt="Frappe de rupture réelle, placement de trois drones, provocation et fiche du Gardien">
<p>Pour essayer sans toucher à ta partie : <code>Voir_frappe_rupture.cmd</code> ou <code>Voir_drones_agent.cmd</code> à la racine du projet.
Dans le second essai : <strong>1 Traqueur · 2 Artilleur · 3 Gardien</strong>. Choisir une case libre voisine puis confirmer l’attaque. Passer un tour fait agir les drones.</p>
<p class="note">Trait arcanique reste utilisable. Les autres effets du catalogue doivent encore être raccordés ; apprendre ou placer une compétence dans la barre n’implémente pas son effet.
Les mannequins et le bestiaire conservent ici leurs dessins actuels.</p>
</body></html>"""
    if args.blast_only:
        page="""<!doctype html><html lang="fr"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Déflagration occulte</title>
<style>:root{color-scheme:dark;font-family:system-ui,sans-serif;background:#0b0d10;color:#e4e4e9}
body{max-width:1280px;margin:auto;padding:26px 20px}h1{font-size:26px}p{line-height:1.6;max-width:1000px}
img{display:block;max-width:100%;height:auto;border:1px solid #30313d}.note{color:#a9acb9}code{color:#c6b4ff}</style>
</head><body><h1>Déflagration occulte · intégration jouable</h1>
<p class="note">AGENT-PLAY04 · Images natives du vrai moteur · Cases de 64 px.</p>
<p>Au premier rang, l’explosion couvre un carré de 3 × 3 cases. Les dégâts diminuent du centre vers la bordure, où ils valent la moitié des dégâts centraux, avant les protections de chaque cible.
Chaque cible possède son propre jet de critique. Le joueur est protégé contre ses propres sorts ; ses alliés et drones peuvent être touchés par cette explosion.</p>
<p>Les obstacles arrêtent la zone et l’animation. Le dessin peut déborder sur les cases libres voisines ; les dégâts restent dans la zone prévisualisée.
La prévisualisation disparaît à la confirmation. Une case libre peut servir de centre.</p>
<img src="skill-playable-review-01/trait-arcanique.webp?review=AGENT-PLAY04" width="1280" height="800" alt="Prévisualisation de la zone, explosion occulte, dégâts réels et fiche de compétence">
<p>Essai isolé : <code>Voir_deflagration_occulte.cmd</code>, à la racine du projet. Utilise le premier raccourci, choisis le centre et confirme avec l’action Attaquer.
La partie courante n’est pas chargée ou modifiée.</p>
<p class="note">La barre, Utiliser et le clic droit partagent le même effet. Le lot porte à six le nombre de compétences actives raccordées ; les autres restent à intégrer.</p>
</body></html>"""
    if args.fireball_only:
        page="""<!doctype html><html lang="fr"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Ciblage au survol</title>
<style>:root{color-scheme:dark;font-family:system-ui,sans-serif;background:#0b0d10;color:#e4e4e9}
body{max-width:1280px;margin:auto;padding:26px 20px}h1{font-size:26px}p{line-height:1.6;max-width:1000px}
img{display:block;max-width:100%;height:auto;border:1px solid #30313d}.note{color:#a9acb9}code{color:#c6b4ff}</style>
</head><body><h1>Ciblage au survol · Boule de feu</h1>
<p class="note">AGENT-PLAY05 · AGENT-AIM02 · Captures natives du moteur · Cases de 64 px.</p>
<p>La zone suit le survol sans clic. Le clavier garde la main tant que la souris reste immobile ; Un clic gauche sur la carte ou la touche Attaquer confirme.</p>
<p>Le projectile explose au premier obstacle ou personnage rencontré, ou sur la case visée. L’aperçu montre la zone réellement touchée, avec les murs pris en compte. Les dégâts diminuent vers le bord de l’explosion.</p>
<p>La brûlure éventuelle suit la cible pendant trois tours complets après l’impact. Elle garde la puissance du lancement et utilise la résistance au feu actuelle à chaque tour. Une nouvelle application remplace la précédente et recommence la durée.</p>
<img src="skill-playable-review-01/trait-arcanique.webp?review=AGENT-AIM02" width="1280" height="800" alt="Visée, interception du projectile, explosion, brûlure sur les victimes et fiche de Boule de feu">
<p>Essai isolé depuis le dossier du projet : <code>./target/debug/project-rl.exe --essai-boule-de-feu</code>. Le premier raccourci prépare le sort ; un clic gauche ou l’action Attaquer confirme. La campagne n’est pas chargée.</p>
<p class="note">Sept compétences actives sont raccordées. La barre, Utiliser et le clic droit exécutent le même effet. Le joueur est protégé contre son propre sort ; ses alliés peuvent être touchés.</p>
</body></html>"""
    replace_text(base / "apercu-animations.html", page)
    print(json.dumps({"animation": str(movie), "bytes": movie.stat().st_size, "frames": len(frames)}))


if __name__ == "__main__":
    main()
