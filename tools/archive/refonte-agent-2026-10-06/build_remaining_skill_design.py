"""Complete the proposed rules catalogue; do not generate roleplay copy or game code."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import importlib.util
import json
import math
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"
DOCUMENT = "docs/CADRAGE_COMPETENCES_AGENT_COMPLET.md"
DECISIONS = PREFIX + "cooldown-acceptance-and-complete-design-decisions.json"
REFERENCE = {"Force": 5, "Agilité": 5, "Perception": 5, "Intelligence": 5, "Volonté": 5}
CURRENT = {key: 10 for key in REFERENCE}
RULES = {}


def floor(value):
    value = Fraction(value)
    return value.numerator // value.denominator


def ceil(value):
    value = Fraction(value)
    return -((-value.numerator) // value.denominator)


def curve(base, limit, n, offset=9):
    return Fraction(base) + Fraction((limit - base) * (n - 1), n + offset)


def multiplier(profile, family):
    if family == "magie":
        return 1 + Fraction(profile["Volonté"] - 5, 50)
    if family == "technique":
        return 1 + Fraction(profile["Intelligence"] - 5, 50)
    stat = "Force" if family == "mêlée" else "Perception"
    return (1 + Fraction(profile[stat] - 5, 50)) * (1 + Fraction(profile["Agilité"] - 5, 200))


def fixed(label, value, unit=""):
    return (label, unit, str(value), lambda n, p: value)


def linear(label, base, step, unit=""):
    return (label, unit, f"{base} + {step}r", lambda n, p: base + step * (n - 1))


def stepped(label, base, every, unit="", cap=None, gain=1):
    step_expression = f"floor(r/{every})" if gain == 1 else f"{gain} × floor(r/{every})"
    expression = f"{base} + {step_expression}"
    if cap is not None:
        expression = f"min({cap}, {expression})"
    return (label, unit, expression,
            lambda n, p: min(cap, base + gain * ((n - 1) // every)) if cap is not None else base + gain * ((n - 1) // every))


def bounded(label, base, limit, unit="%", offset=9):
    return (label, unit, f"{base} + {limit - base}r/(n + {offset})", lambda n, p: curve(base, limit, n, offset))


def damage(label, base, step, family="magie"):
    return (label, "dégâts", f"floor(({base} + {step}r) × M_{family})",
            lambda n, p: floor((base + step * (n - 1)) * multiplier(p, family)))


def weapon_damage(label, base, step):
    return (label, "dégâts", f"floor(D × ({base} + {step}r)/100 × M_distance)",
            lambda n, p: floor(20 * Fraction(base + step * (n - 1), 100) * multiplier(p, "distance")))


def custom(label, expression, fn, unit=""):
    return (label, unit, expression, fn)


def duration(base, every, attribute=None):
    extra = f" + min(3, floor(max(0, {attribute} − 5)/20))" if attribute else ""
    return custom("Durée", f"{base} + floor(r/{every}){extra}",
                  lambda n, p: base + (n - 1) // every + (min(3, max(0, p[attribute] - 5) // 20) if attribute else 0), "tours")


def energy(base, every=3, label="Énergie"):
    return stepped(label, base, every, "points")


def power(attribute="Volonté", both=False):
    return custom("Puissance de contrôle", "100 + 5r + 2(Intelligence − 5) + 2(Volonté − 5)" if both else f"100 + 5r + 2({attribute} − 5)",
                  lambda n, p: max(1, 100 + 5 * (n - 1) + 2 * (p[attribute] - 5) + (2 * (p['Intelligence'] - 5) if both else 0)), "unités")


def add(identifier, metrics, detail, *, cost=None, cooldown=None, tags=()):
    assert identifier not in RULES
    fields = list(metrics)
    if cost is not None:
        fields.append(energy(cost))
    if cooldown is not None:
        fields.append(fixed("Recharge", cooldown, "tours"))
    RULES[identifier] = {"metrics": fields, "detail": detail, "tags": list(tags)}


# Distance: figures describe one projectile, never multiply an ordinary PM burst again.
add("DI01", [linear("Dégâts du premier projectile", 110, 6, "% de D"), weapon_damage("Premier impact de comparaison", 110, 6), stepped("Cibles traversées au maximum", 2, 5, cap=5), fixed("Dégâts conservés par traversée", 75, "%"), fixed("Munitions", 1)],
    "Un projectile traverse les acteurs alignés, sans traverser un mur. Chaque acteur est testé une seule fois ; les dégâts de l'impact k sont calculés avec 0,75 puissance k avant l'arrondi. Les alliés sur la ligne peuvent être touchés. Un échec de précision sur un acteur laisse le projectile poursuivre sa trajectoire. Portée ordinaire de l'arme.", cost=8, cooldown=4, tags=("arme", "distance", "principal"))
add("DI02", [stepped("Projectiles sur une cible", 3, 4, cap=6), linear("Dégâts par projectile", 60, 3, "% de D"), weapon_damage("Impact de comparaison", 60, 3), fixed("Précision par projectile", 90, "% de P"), custom("Munitions", "min(6, 3 + floor(r/4))", lambda n, p: min(6, 3 + (n - 1)//4))],
    "Toute la rafale est payée avant son premier projectile. Chaque projectile possède son jet de précision et ses défenses. Elle remplace la rafale ordinaire d'un PM ; elle ne multiplie pas ses trois projectiles par le nombre affiché. La cible principale est choisie avant confirmation. Portée de l'arme.", cost=10, cooldown=4, tags=("arme", "distance", "principal"))
add("DI03", [stepped("Cibles visées au maximum", 3, 5, cap=6), linear("Dégâts par cible", 80, 5, "% de D"), weapon_damage("Impact de comparaison", 80, 5), stepped("Portée du cône, limitée aussi par l'arme", 3, 4, "cases", cap=6), fixed("Angle du cône", 90, "degrés"), fixed("Munitions par cible visée", 1)],
    "Une balle par hostile visible retenu dans le cône, un tir au maximum par cible. Les cibles sont prises de la plus proche à la plus éloignée, avec des lignes de tir légales. Sans cible admissible, refus sans coût. Seules les balles des tirs réellement prévus sont débitées ; leur total doit être disponible. L'aperçu présente le cône et les cibles observées retenues.", cost=12, cooldown=6, tags=("arme", "distance", "principal"))
add("DI04", [linear("Dégâts après rebond", 90, 6, "% de D"), weapon_damage("Impact de comparaison", 90, 6), fixed("Rebonds", 1), fixed("Précision après rebond", 80, "% de P"), fixed("Munitions", 1)],
    "Choix d'une face de mur connue et visible, puis d'une cible actuellement observée. Les deux segments sont libres, sauf la surface du rebond, et leur longueur cumulée respecte la portée de l'arme. Un seul rebond ; pas de gain de portée ni de cible cachée révélée. L'aperçu montre les deux segments avant confirmation.", cost=8, cooldown=4, tags=("arme", "distance", "principal"))
add("DI05", [linear("Dégâts", 100, 4, "% de D"), weapon_damage("Impact de comparaison", 100, 4), stepped("Rapprochement maximal", 1, 5, "cases", cap=3), fixed("Munitions", 1)],
    "Un tir réussi rapproche la cible vers l'agent, jusqu'à la limite ou au premier obstacle. Aucun mur, acteur ou case interdite n'est traversé ; la cible ne finit pas sur l'agent. Une cible déclarée immobile ou ancrée garde sa position, mais peut subir les dégâts. Un échec du tir ne rapproche pas la cible.", cost=10, cooldown=6, tags=("arme", "distance", "principal"))
add("DI06", [linear("Dégâts du tir de réaction", 100, 5, "% de D"), weapon_damage("Impact de comparaison", 100, 5), duration(3,5), fixed("Angle du secteur", 60, "degrés"), fixed("Tirs par activation", 1), fixed("Munitions payées à l'activation", 1)],
    "L'action met immédiatement la surveillance en place ; le tir a lieu plus tard, au premier hostile visible entrant dans le secteur à portée de l'arme. La munition et l'énergie sont payées à l'activation, même si aucun tir ne se déclenche. Mouvement, changement d'arme, expiration ou premier tir terminent la surveillance. La réaction ne donne pas une action supplémentaire à l'ennemi et ne lance pas une nouvelle surveillance.", cost=8, cooldown=5, tags=("arme", "distance", "principal", "surveillance"))

# Evasion: do not turn armor absorption into a successful dodge.
add("T02", [fixed("Déplacement", 1, "case"), linear("Bonus d'esquive", 15, 3, "unités"), duration(2,5)],
    "Reprend T02 du lot 02. Case adjacente libre et connue, mouvement ordinaire avec ses réactions. Le bonus commence avant ces réactions. Pas de mur traversé, de frappe gratuite ou d'invulnérabilité. Une nouvelle application remplace l'ancienne.", cost=8, cooldown=6, tags=("magique", "déplacement"))
add("ES02", [fixed("Déplacement maximal", 2, "cases"), linear("Bonus d'esquive jusqu'au prochain tour", 10, 2, "unités"), fixed("Durée du bonus", 1, "tour")],
    "Deux cases au maximum en ligne droite, toutes connues, libres et franchissables. Chaque case garde ses réactions et dangers. Le bonus commence avant le trajet. Aucun mur, acteur ou coin interdit traversé et aucune attaque ajoutée.", cost=6, cooldown=4, tags=("technologique", "déplacement"))
add("ES03", [duration(3,4), fixed("Coups nécessaires pour détruire le leurre", 1), fixed("Distance de placement", 3, "cases")],
    "Un seul leurre à la fois, sur une case libre actuellement visible. Un hostile qui le voit peut le prendre comme cible de diversion ; un acte agressif de l'agent contre cet hostile lui rend la priorité. Le premier impact contre le leurre le détruit. Le leurre ne bloque pas le passage, ne fait pas de dégâts et ne donne ni XP ni loot. C'est un effet de perception, sans invocation de drone.", cost=8, cooldown=6, tags=("technologique", "leurre"))
add("ES04", [linear("Dégâts de riposte", 60, 4, "% d'une attaque de mêlée"), custom("Dégâts de comparaison", "floor(20 × (60 + 4r)/100 × M_mêlée)", lambda n,p: floor(20*Fraction(60+4*(n-1),100)*multiplier(p,'mêlée')), "dégâts"), fixed("Intervalle entre ripostes", 2, "tours")],
    "Après une attaque réellement esquivée, une riposte est possible uniquement si l'attaquant est visible et à portée de l'arme de mêlée ou de l'attaque naturelle compatible. Elle fait un jet de précision normal, sans énergie ni action manuelle supplémentaire. Un coup absorbé par l'armure n'est pas une esquive. La riposte ne déclenche ni nouvelle riposte ni autre attaque automatique.", tags=("passif", "secondaire", "mêlée"))
add("ES05", [fixed("Passage derrière un hostile adjacent", 2, "cases parcourues"), linear("Bonus d'esquive", 10, 2, "unités"), fixed("Durée du bonus", 2, "tours")],
    "Passe de l'autre côté d'un hostile adjacent, sur la case opposée connue et libre. Le passage à travers cet acteur est autorisé par ce pouvoir, mais pas à travers un mur, une porte fermée ou un coin interdit. Aucun dégât n'est infligé. Le bonus commence avant les réactions au déplacement.", cost=10, cooldown=8, tags=("magique", "déplacement"))
add("ES06", [linear("Esquive supplémentaire contre projectiles", 12, 3, "unités")],
    "Bonus seulement contre un projectile qui utilise un jet de précision. Il ne devient pas une réduction générale des dégâts et ne crée pas un jet d'esquive pour un sort qui n'en a pas.", tags=("passif", "défense"))

# Fire: areas and ongoing effects never generate new casts for passive chains.
add("FE01", [damage("Dégâts directs",8,3), damage("Brûlure par tour",2,1), duration(3,5), fixed("Portée",5,"cases")],
    "Une seule cible hostile, ligne de vue. L'impact est immédiat ; une brûlure propre à l'agent commence au prochain tour complet. Le même auteur ne cumule pas plusieurs exemplaires de cette brûlure. Ce projectile ne traverse pas gratuitement plusieurs cibles.", cost=8, cooldown=3, tags=("magique","distance","principal","feu"))
add("FE02", [damage("Dégâts par maintien",6,2), stepped("Portée du cône",3,5,"cases",cap=5), fixed("Angle",90,"degrés"), fixed("Résolutions maximales",3), energy(6,4,"Énergie par résolution")],
    "Le lancement produit le premier souffle immédiatement. Deux maintiens au maximum, chacun prenant une action et son énergie. Le joueur peut arrêter ; mouvement, autre action ou perte de PV termine la canalisation. La recharge ne bloque pas la commande distincte de maintien. Les murs arrêtent le cône ; les alliés peuvent être touchés.", cooldown=6, tags=("magique","distance","principal","feu","canalisation"))
add("FE03", [damage("Dégâts de zone par tour",5,2), stepped("Longueur maximale du mur",3,5,"cases",cap=7,gain=2), duration(3,4), fixed("Portée de placement",4,"cases")],
    "Ligne de flammes sur des cases visibles et franchissables ; les obstacles interrompent la ligne. Elle n'est pas un mur de déplacement. Une entrée ou un tour passé dans le feu blesse, au maximum une fois par acteur et par tour pour cet effet. Les alliés et l'agent peuvent être blessés.", cost=12, cooldown=8, tags=("magique","distance","zone","feu"))
add("FE04", [damage("Dégâts de l'explosion",14,4), fixed("Rayon",1,"case"), fixed("Portée",5,"cases"), fixed("Délai annoncé",1,"tour complet")],
    "Le marqueur est posé dès le lancement. L'explosion survient après un tour complet, en laissant une réponse aux ennemis. Le rayon est arrêté par les murs. Les acteurs peuvent quitter la zone ; les alliés peuvent être touchés. Pas de destruction de terrain ni de nouvelle carte.", cost=12, cooldown=6, tags=("magique","distance","principal","feu","différé"))
add("FE05", [damage("Dégâts par tour",4,2), fixed("Rayon initial",1,"case"), stepped("Rayon maximal après des morts dans le foyer",2,10,"cases",cap=3), duration(4,4), fixed("Portée",4,"cases")],
    "À chaque mort hostile dans le foyer, son rayon gagne une case jusqu'à sa limite. Un décès n'est compté qu'une fois. L'expansion n'allonge pas sa durée et respecte les murs. Les flammes blessent aussi les alliés ; une seule résolution par acteur et par tour.", cost=14, cooldown=10, tags=("magique","distance","zone","feu"))
add("FE06", [linear("Part de la brûlure restante convertie",100,4,"%"), fixed("Portée",5,"cases")],
    "Consomme la brûlure encore active appartenant à l'agent sur une cible visible. Le choc est calculé sur ses dégâts restants avant défenses, puis résolu en feu contre la cible. La Volonté déjà incluse dans la brûlure n'est pas multipliée une seconde fois. Sans brûlure admissible, refus sans coût. Pas de consommation des flammes d'un autre auteur ou d'une zone de sol.", cost=6, cooldown=3, tags=("magique","distance","principal","feu"))

# Cold: movement denial preserves attacks and other non-movement actions.
add("FR01", [damage("Dégâts directs",7,3), bounded("Surcoût du déplacement",30,60), duration(3,5), fixed("Portée",5,"cases"), power()],
    "L'impact blesse immédiatement. Le ralentissement, soumis à résistance, augmente le coût en temps du mouvement, pas celui de toutes les attaques. Il ne réduit pas la précision et ne supprime aucune action.", cost=6, cooldown=2, tags=("magique","distance","principal","froid","contrôle"))
add("FR02", [damage("Dégâts par hostile",4,2), duration(2,5), fixed("Rayon autour de l'agent",2,"cases"), power()],
    "Vise les hostiles autour de l'agent, avec obstacles. Le gel bloque leurs déplacements volontaires s'il passe la résistance ; les attaques et autres actions restent possibles. Les déplacements forcés peuvent sortir une cible de sa position. Les téléportations restent des actions de déplacement et sont bloquées pendant ce gel.", cost=10, cooldown=6, tags=("magique","zone","principal","froid","contrôle"))
add("FR03", [custom("PV de chaque bloc", "floor((10 + 3r) × M_magie)",lambda n,p: floor((10+3*(n-1))*multiplier(p,'magie')),"PV"), stepped("Longueur",3,5,"cases",cap=7,gain=2), duration(3,5), fixed("Portée",4,"cases")],
    "Blocs uniquement sur des cases connues, visibles, libres et franchissables. Ils bloquent déplacements et projectiles comme obstacles temporaires ; chaque bloc peut être détruit séparément, sans loot ni XP. Aucun acteur n'est repoussé pour faire place. La disparition ne cause pas de dégâts.", cost=12, cooldown=8, tags=("magique","obstacle","froid"))
add("FR04", [damage("Dégâts par éclat",4,2), fixed("Progression de l'orbe",1,"case par tour"), fixed("Étapes d'émission",3), fixed("Directions d'éclats par étape",4), stepped("Portée des éclats",1,5,"cases",cap=3)],
    "Trajet initial de trois cases connues et visibles, en ligne droite. L'orbe progresse d'une case par tour puis émet dans les quatre directions cardinales ; il disparaît contre un mur ou après trois émissions. Les éclats sont arrêtés par murs et premier acteur rencontré. Un acteur ne reçoit qu'un éclat par émission. Les alliés peuvent être touchés. Aucun gel ajouté implicitement.", cost=14, cooldown=8, tags=("magique","distance","zone","froid","différé"))
add("FR05", [damage("Dégâts par impact",6,2), stepped("Impacts par tour",2,5,cap=5), duration(3,5), fixed("Rayon",2,"cases"), fixed("Portée",5,"cases")],
    "À chaque tour, les impacts sont répartis sans remplacement parmi les cases de la zone accessibles à l'effet. Une case reçoit au maximum un impact ce tour. L'effet commence au prochain tour complet et peut blesser les alliés. Les murs arrêtent l'aire ; ce sort ne donne pas automatiquement une immobilisation.", cost=16, cooldown=10, tags=("magique","distance","zone","froid"))
add("FR06", [damage("Dégâts par éclat",3,1), stepped("Cibles secondaires maximales",2,5,cap=4), stepped("Allonge derrière la victime",1,5,"cases",cap=3), fixed("Déclenchements maximaux par tour",1)],
    "Un coup principal de l'agent contre un hostile immobilisé par le froid projette des éclats vers des hostiles visibles derrière lui. Le gel peut provenir d'une mine glacée. Les drones ne déclenchent pas ce passif. Les éclats ne redéclenchent pas Fragilité, ne libèrent pas automatiquement la cible et n'ajoutent pas de gel.", tags=("passif","magique","distance","secondaire","froid"))

# Lightning: limited paths, nonlethal percentage effect, and one charge source per cast.
add("FO01", [damage("Dégâts du premier arc",10,3), stepped("Cibles maximales",3,5,cap=6), fixed("Dégâts conservés par saut",80,"%"), fixed("Distance maximale d'un saut",2,"cases"), fixed("Portée initiale",5,"cases")],
    "Chaque saut prend le hostile visible admissible le plus proche, avec ligne de vue depuis la cible précédente. Pas de cible visitée deux fois. Les dégâts du saut k conservent 0,8 puissance k de la valeur de départ. Sans autre cible admissible, la chaîne s'arrête.", cost=10, cooldown=4, tags=("magique","distance","principal","foudre"))
add("FO02", [custom("Part des PV actuels avant résistances", "(20 + 25r/(n+9)) × M_magie",lambda n,p: curve(20,45,n)*multiplier(p,'magie'),"%"), fixed("Plancher des PV après ce seul effet",50,"% des PV maximum"), fixed("Rayon",2,"cases")],
    "Affecte les hostiles proches ; dégâts de foudre limités pour ne pas faire passer la cible sous la moitié de ses PV maximum. Une cible déjà sous ce plancher ne perd rien. Les résistances diminuent encore la perte. Le plancher concerne ce champ, pas un autre effet distinct déclenché pendant le tour. Le calcul ne peut pas donner un soin.", cost=12, cooldown=6, tags=("magique","zone","principal","foudre","proportionnel"))
add("FO03", [stepped("Recul maximal",1,5,"cases",cap=3), fixed("Rayon",2,"cases"), power()],
    "Repousse les hostiles proches si le contrôle passe leur résistance. Trajets radiaux libres, sans traversée d'acteur ni dégât de collision ajouté. Le sort ne donne pas de dégâts directs. Une cible ancrée ne bouge pas. Les dangers rencontrés conservent leurs règles.", cost=10, cooldown=6, tags=("magique","contrôle","foudre"))
add("FO04", [damage("Dégâts par éclair",10,3), duration(3,4), fixed("Rayon",2,"cases"), fixed("Cibles par tour",1), fixed("Portée du centre",5,"cases")],
    "Un éclair par tour complet contre un hostile actuellement observé dans la zone, le plus proche du centre à égalité départagée par case. Sans hostile admissible, aucun éclair. Pas de tir sur un acteur caché, pas de nouvelle charge passive pour chaque frappe et pas de dégât aux alliés.", cost=16, cooldown=10, tags=("magique","distance","zone","foudre"))
add("FO05", [damage("Dégâts au franchissement",6,2), duration(3,5), stepped("Distance maximale entre ancrages",3,4,"cases",cap=6), fixed("Portée de chaque ancrage depuis l'agent",5,"cases")],
    "Deux cases libres et visibles reliées sans mur. La ligne n'est pas un obstacle. La franchir blesse au maximum une fois par acteur et par tour ; rester immobile dessus ne crée pas une nouvelle traversée. Alliés et agent peuvent être touchés. Les déplacements forcés restent des franchissements.", cost=12, cooldown=8, tags=("magique","distance","zone","foudre"))
add("FO06", [damage("Dégâts de la décharge",4,2), fixed("Sorts requis",3), fixed("Rayon de décharge autour de l'agent",1,"case"), fixed("Expiration sans nouveau sort électrique",5,"tours")],
    "Un lancement légal d'un sort électrique offensif appris donne une charge, pas chaque cible, rebond ou tick. À trois charges, une décharge unique touche les hostiles proches puis vide les charges. Cette décharge ne produit aucune charge ni attaque automatique dérivée. Les charges expirent sans nouveau sort et au changement de carte.", tags=("passif","magique","secondaire","foudre"))

# Occult skills: existing three definitions are retained, with no roleplay rewrite.
add("A01", [damage("Dégâts directs",8,3), fixed("Portée",5,"cases")],
    "Reprend A01 du lot 02 : hostile visible avec ligne de vue, pas de jet de précision d'arme, défenses occultes ordinaires. Le sort ne donne aucun affaiblissement supplémentaire.", cost=6, cooldown=2, tags=("magique","distance","principal","occulte"))
add("A02", [damage("Dégâts par acteur",6,2), fixed("Zone centrée",'3 × 3 cases'), fixed("Portée",4,"cases")],
    "Reprend A02 du lot 02 : case centrale visible, murs qui coupent l'effet. Agent et alliés peuvent être touchés. Le terrain n'est ni incendié ni détruit. La zone reste de trois par trois cases.", cost=10, cooldown=4, tags=("magique","distance","principal","occulte"))
add("A03", [bounded("Réduction des coûts magiques",20,50)],
    "Reprend A03 du lot 02 et son rang initial accepté. Tous les modificateurs applicables sont combinés avant un unique arrondi supérieur ; un coût positif reste au moins égal à un. Pas de réduction des munitions, du temps ou des déploiements purement technologiques. Le pas furtif gratuit au rang 10 conserve sa propre exception.", tags=("passif","magique","coût"))
add("OC04", [custom("PV de chaque pilier", "floor((10 + 2r) × M_magie)",lambda n,p: floor((10+2*(n-1))*multiplier(p,'magie')),"PV"), duration(2,5), fixed("Périmètre",'8 cases autour d’un centre de 3 × 3'), fixed("Portée du centre",4,"cases")],
    "Le centre peut contenir un acteur ; toutes les cases du périmètre doivent être visibles, libres et franchissables. Les piliers deviennent des obstacles destructibles séparément, sans XP ni loot. Aucun acteur n'est déplacé à la création. Expiration et destruction n'ajoutent pas de dégâts.", cost=14, cooldown=10, tags=("magique","obstacle","occulte"))
add("OC05", [damage("Dégâts par résolution",6,2), bounded("Vie rendue sur les PV effectivement retirés",10,25), fixed("Résolutions maximales",3), fixed("Portée",5,"cases"), energy(4,4,"Énergie par résolution")],
    "Premier rayon immédiat puis deux maintiens au maximum, chacun coûtant son action et son énergie. Le vol de vie est calculé après défenses, arrondi au supérieur et plafonné aux PV manquants et aux PV retirés. Mouvement, autre action, perte de PV ou perte de cible termine le rayon. Pas de soin sur un obstacle, un leurre ou une cible ne perdant aucun PV.", cooldown=6, tags=("magique","distance","principal","occulte","canalisation"))
add("OC06", [duration(3,5), stepped("Distance maximale entre passages",4,5,"cases",cap=6), fixed("Portée de chaque case depuis l'agent",5,"cases")],
    "Deux cases actuellement visibles, libres et franchissables de la carte active. Entrer dans un passage mène à l'autre en un déplacement ordinaire ; une sortie bloquée refuse la traversée. Alliés et hostiles peuvent utiliser la liaison. Pas d'autre couche, de carte bonus générée ou d'échange gratuit depuis un menu. Un seul couple à la fois.", cost=16, cooldown=10, tags=("magique","passage","occulte"))

# Five forms: all actions belong to their transformation, not to extra purchased nodes.
add("M01", [damage("Griffe ordinaire",8,2,"mêlée"), duration(6,3,"Volonté"), fixed("Allonge de Griffe",1,"case"), fixed("Distance de Bond",2,"cases"), fixed("Énergie de Bond",4), fixed("Recharge de Bond",3,"tours")],
    "Reprend M01 du lot 02. Griffe : attaque naturelle physique, une action, sans énergie supplémentaire. Bond : action distincte, trajet connu et franchissable, sans frappe gratuite. L'entrée dans la forme est magique ; Bond reste naturel et n'est pas réduit par Canalisation.", cost=12, cooldown=10, tags=("magique","transformation","mêlée"))
add("TR61", [damage("Coup lourd ordinaire",12,3,"mêlée"), damage("Onde de choc par cible",6,2,"mêlée"), stepped("Recul de l'onde",1,5,"cases",cap=2), duration(5,3,"Volonté"), fixed("Rayon de l'onde",1,"case"), fixed("Énergie de l'onde",6), fixed("Recharge de l'onde",4,"tours")],
    "Coup lourd : attaque naturelle au contact. Onde : action physique distincte sur les hostiles adjacents, avec précision et recul sur trajet libre. Déplacements ordinaires de la forme coûtant 25 % de temps en plus, attaques conservant une action. Pas de dégâts de collision ni de hausse de PV qui soigne à l'entrée.", cost=14, cooldown=10, tags=("magique","transformation","mêlée"))
add("TR63", [damage("Heurt ordinaire",6,2,"mêlée"), bounded("Réduction des dégâts directs",10,35), bounded("Réserve maximale de riposte",10,30,"% des PV maximum"), duration(5,3,"Volonté"), damage("Dégâts de base de Décharge de plaques",5,2,"mêlée"), fixed("Énergie de Décharge de plaques",6), fixed("Recharge de Décharge de plaques",4,"tours")],
    "La réserve reçoit les PV réellement perdus sur les coups directs d'hostiles, dans sa limite. Décharge de plaques est une action au contact : ses dégâts de base plus la réserve sont résolus contre la cible, puis la réserve est vidée même sur échec. Pas de charge sur poison, feu de sol ou dégâts alliés. La sortie de forme efface la réserve.", cost=14, cooldown=10, tags=("magique","transformation","mêlée","défense"))
add("TR64", [damage("Trait corporel ordinaire",8,3), fixed("Portée de Trait corporel",4,"cases"), damage("Pulsation occulte par hostile",5,2), fixed("Rayon de Pulsation",2,"cases"), fixed("Énergie de Pulsation",6), fixed("Recharge de Pulsation",4,"tours"), duration(4,3,"Volonté")],
    "Trait corporel est l'attaque naturelle ordinaire de cette forme, sans coût supplémentaire, avec dégâts occultes et défenses de cible. Pulsation est une compétence active de forme, centrée sur l'agent et visant les hostiles proches. Silence laisse l'attaque ordinaire mais bloque Pulsation. La forme ne reçoit pas de munitions d'arme.", cost=16, cooldown=10, tags=("magique","transformation","distance","occulte"))
add("TR65", [damage("Allonge ordinaire",7,2,"mêlée"), fixed("Portée de l'attaque naturelle",2,"cases"), fixed("Portée de Pas d'ombre",3,"cases"), fixed("Énergie de Pas d'ombre",5), fixed("Recharge de Pas d'ombre",3,"tours"), duration(5,3,"Volonté")],
    "Allonge : attaque physique naturelle, arrêtée par les obstacles, sans munition. Pas d'ombre : action magique vers une case libre actuellement visible à trois cases maximum. Pour cette proposition, une ombre est une case adjacente à un obstacle opaque connu ; aucun nouveau système d'éclairage n'est requis pour définir la destination. Pas de traversée de carte ni d'attaque gratuite.", cost=14, cooldown=10, tags=("magique","transformation","mêlée"))
add("M03", [linear("Dégâts supplémentaires sous forme",10,2,"%")],
    "Reprend M03 du lot 02 : bonus seulement tant que l'agent est transformé, physique et magique, sans drones. Les effets persistants conservent leur auteur ; ce multiplicateur est lu au moment de leurs dégâts et disparaît au retour humain.", tags=("passif","transformation","dégâts"))

# Hybrid skills: limited extension, no cargo sacrifice, atomic double-cost relay.
add("M02", [damage("Dégâts directs",7,2), stepped("Tours ajoutés par impact admissible",1,10,"tours",cap=2), stepped("Tours ajoutés au maximum par transformation",2,5,"tours",cap=6), fixed("Portée",3,"cases")],
    "Reprend les dégâts de M02 et complète la prolongation. Il faut retirer réellement des PV à un hostile pour prolonger une forme en cours. Le total d'extension appartient à cette activation de forme ; il n'est pas réinitialisé par un équipement ou un autre lancer de M02. Hors forme, le sort donne seulement ses dégâts. Pas de prolongation sur une cible invulnérable, alliée ou un leurre.", cost=6, cooldown=2, tags=("magique","distance","principal","occulte","hybride"))
add("HY02", [bounded("Absorption créée",25,60,"% des PV actuels du drone sacrifié"), duration(3,5), fixed("Distance du drone",4,"cases")],
    "Sacrifie D58, D59 ou D66 contrôlé, visible et à portée. D56 et D57 sont exclus ; aucun chargement de Mule n'est perdu. Absorption arrondie au supérieur, dégâts après les défenses précédentes. Un seul bouclier de Fusion ; pas de cumul ou de nouvelle Fusion pendant celui-ci. Le sacrifice ne donne ni loot ni XP ni remboursement de déploiement.", cost=8, cooldown=8, tags=("magique","technologique","hybride","défense"))
add("HY03", [stepped("Distance du relais depuis l'agent",4,5,"cases",cap=8), linear("Dégâts du sort relayé",100,3,"% de ses dégâts ordinaires"), energy(4,label="Surcoût du relais"), fixed("Recharge propre du relais",6,"tours")],
    "Sorts proposés compatibles : FE01, FR01, FO01, A01, A02, FE04, FR05, FO04. Le sort doit être appris et disponible. Son origine, sa portée et sa ligne de vue sont calculées depuis un drone déployé réellement observé. Le lancement paie ensemble le sort et le surcoût et engage leurs deux recharges. Canalisation n'est appliquée qu'une fois à chaque coût admissible. Le relais ne prête pas la Volonté du drone, ne transmet pas le cargo, ne contourne pas l'observation et ne fait pas apprendre le sort. Un seul lancement, une seule attribution des déclenchements passifs.", tags=("magique","technologique","hybride","relais"))
add("HY04", [stepped("Distance d'échange",4,5,"cases",cap=8), custom("Énergie", "ceil(10 − 4r/(n+9))",lambda n,p: ceil(10-Fraction(4*(n-1),n+9)),"points"), custom("Recharge", "ceil(3 + 50/(n+9))",lambda n,p: ceil(3+Fraction(50,n+9)),"tours")],
    "Échange l'agent avec un drone déployé contrôlé et visible. Les deux destinations doivent être légales pour leurs occupants, sur la même carte. Aucun échange avec une réserve de drones, une autre couche ou un ennemi. Les dangers de destination se résolvent normalement ; le cargo reste dans son drone.", tags=("magique","technologique","hybride","déplacement"))
add("HY05", [stepped("Renforcements retirés au maximum",1,5,cap=3), bounded("Énergie récupérée au total",5,15,"% de l'énergie maximale"), fixed("Portée",4,"cases")],
    "Choisit des buffs temporaires ennemis portant explicitement la compatibilité de dissipation. Pas de traits, équipement ou passif permanent effacé. Sans effet admissible, refus sans coût. Le gain total, arrondi au supérieur, est accordé une fois par lancement, pas par buff ; il ne dépasse pas la réserve manquante. Les alliés et drones du joueur sont exclus.", cost=6, cooldown=6, tags=("magique","technologique","hybride","dissipation"))
add("HY06", [duration(2,5,"Intelligence"), fixed("Ennemis contrôlés simultanément",1), fixed("Portée",4,"cases"), power(both=True)],
    "Contrôle temporairement un hostile admissible s'il échoue à sa résistance. L'ennemi conserve ses PV, statistiques et compétences ; il devient un allié temporaire, pas un drone. Un flag explicite indomptable pourra exclure certains monstres importants, sans exclusion par nature machine/organique/démon. La fin rend la cible hostile sans soin. Sortir de carte termine le lien ; aucun captif n'est transporté. Une nouvelle domination réussie remplace l'ancienne. Le classement des monstres importants sera défini lors de leur conception.", cost=16, cooldown=10, tags=("magique","technologique","hybride","contrôle"))

# Melee buffs: no recursive lifesteal, reflection or automatic attack production.
add("BM01", [damage("Feu ajouté par coup principal réussi",3,1), duration(4,5)],
    "Arme de mêlée ou attaque naturelle compatible. Le feu est résolu séparément contre les résistances ; il ne crée pas une brûlure pour Combustion. Aucun effet sur les drones, projectiles ou dégâts secondaires de riposte. Le buff ne remplace ni arme ni équipement.", cost=8, cooldown=6, tags=("magique","buff","mêlée","feu"))
add("BM02", [bounded("Vie rendue sur les PV retirés au contact",5,20), duration(4,5)],
    "Coup principal de mêlée contre un hostile : vie volée sur les PV vraiment retirés après défenses. Les dégâts directs ajoutés à ce coup sont inclus, pas les zones ni le renvoi. Soin total arrondi une seule fois, limité aux PV manquants et aux PV retirés. Le renvoi et les attaques automatiques secondaires ne volent pas de vie.", cost=10, cooldown=8, tags=("magique","buff","mêlée","soin"))
add("BM03", [linear("Bonus par charge de série",5,1,"%"), fixed("Charges maximales",3)],
    "Un coup principal de mêlée retirant des PV donne une charge pour les coups suivants. Au maximum trois charges. Un coup manqué, un déplacement ou une autre action prenant du temps rompt la série. Menu et marquage sans passage de tour ne la rompent pas. Le bonus actuel est lu sur le prochain coup ; un équipement ne crée pas de charge.", tags=("passif","mêlée","dégâts"))
add("BM04", [duration(3,5)],
    "Empêche uniquement les déplacements forcés de l'agent pendant sa durée. Il peut toujours décider de se déplacer ou d'employer ses propres mouvements magiques. Ne donne ni armure, ni réduction de dégâts, ni immunité aux attaques.", cost=6, cooldown=6, tags=("magique","buff","défense"))
add("BM05", [bounded("Dégâts au contact renvoyés",10,35), duration(4,5)],
    "Renvoi occulte sur les PV effectivement perdus après protections lors d'une attaque de mêlée hostile. L'auteur doit encore être une cible admissible au contact. Le renvoi respecte sa résistance, ne vole pas de vie, ne charge pas les séries et ne déclenche pas une nouvelle riposte ou un autre renvoi. Aucun renvoi de poison ou de zone.", cost=10, cooldown=8, tags=("magique","buff","mêlée","secondaire","occulte"))
add("BM06", [bounded("Absorption ajoutée par coup admissible",2,8,"% des PV maximum"), fixed("Contributions maximales",3), fixed("Plafond total",15,"% des PV maximum"), fixed("Durée après le dernier coup admissible",2,"tours"), fixed("Contributions ajoutées par tour au maximum",1)],
    "Un coup principal de mêlée retirant des PV à un hostile ajoute une petite réserve absorbante. Trois contributions au maximum et plafond total de 15 % des PV maximum, sans soin. Un seul ajout par tour même pour Tourbillon. Les dégâts consomment la réserve restante ; une nouvelle frappe peut la compléter dans ses limites. Les changements de maximum peuvent la plafonner mais ne la remplissent pas gratuitement.", tags=("passif","mêlée","défense"))

# Ranged buffs: permanent initial damage passive does not depend on staying still.
add("T01", [linear("Précision supplémentaire contre la marque",15,2,"% de P"), duration(4,5), fixed("Portée",6,"cases"), fixed("Temps d'activation",0,"tour")],
    "Reprend T01 du lot 02 : instantané, sans progression du temps ni réaction ennemie. Une seule marque ; prochaine attaque à distance admissible contre elle, même manquée, consomme la marque. Dans une rafale, le multiplicateur s'applique une fois à chaque jet contre la cible principale. Pas de bonus aux drones ni aux sorts sans précision. Le tir de couverture ne la consomme qu'au déclenchement de son tir, si elle est encore présente.", cost=6, cooldown=3, tags=("magique","buff","instantané"))
add("T03", [linear("Dégâts supplémentaires à distance",10,2,"%")],
    "Reprend T03 du lot 02. Permanent, avec ou sans déplacement. Concerne armes et compétences d'attaque classées à distance, dont sorts et actions de forme compatibles ; ni drones ni contact. Une compétence sans dégâts ne reçoit pas de dégâts artificiels.", tags=("passif","distance","dégâts"))
add("BD03", [linear("Dégâts du projectile secondaire",50,3,"% du premier projectile principal"), duration(3,5), fixed("Projectiles secondaires maximaux",1), fixed("Munition supplémentaire si le second tir est possible",1)],
    "Le prochain tir d'arme compatible peut envoyer un seul projectile secondaire vers un autre hostile visible à portée et avec ligne de tir. Une rafale n'est pas dupliquée : seul son premier projectile sert de base. Pas de duplication de grenade ou de zone. Le total de munitions du tir prévu est vérifié avant exécution. Sans second hostile admissible, le tir principal consomme le buff sans balle supplémentaire. Le projectile miroir ne génère pas de miroir ou d'autre attaque automatique.", cost=8, cooldown=6, tags=("magique","buff","distance","secondaire"))
add("BD04", [linear("Précision par tour immobile",5,1,"unités"), fixed("Tours cumulables",3)],
    "Un tour réellement écoulé sans changement de position ajoute un palier, jusqu'à trois. Le bonus vaut le nombre de paliers multiplié par la valeur du rang actuel. Tout changement de position, volontaire, forcé ou magique, vide les paliers. Menu, attente réelle et action instantanée n'en ajoutent pas. T03 conserve son fonctionnement indépendant.", tags=("passif","distance","précision"))
add("BD05", [bounded("Dégâts du projectile dévié",40,80,"% du projectile hostile"), duration(3,5), fixed("Projectiles arrêtés par activation",1)],
    "Intercepte le prochain projectile hostile compatible qui aurait touché l'agent après son jet de précision. Projectiles d'armes sans explosion seulement ; pas de rayon, zone ou sort automatique. Retour vers le tireur uniquement s'il est observé et si le trajet est légal, avec un nouveau jet normal ; sinon le projectile est neutralisé. Le retour ne génère aucun autre renvoi, miroir ou riposte. Un projectile déjà esquivé ne consomme pas l'écran.", cost=10, cooldown=8, tags=("magique","buff","distance","secondaire","défense"))
add("BD06", [bounded("Énergie par élimination admissible",2,6,"% de l'énergie maximale"), fixed("Intervalle entre gains",3,"tours")],
    "Une élimination attribuée à une attaque principale à distance de l'agent récupère de l'énergie, arrondie au supérieur et limitée à la réserve manquante. Un gain au maximum toutes les trois unités de tour. Les drones, leurres, obstacles et dégâts secondaires gratuits ne donnent pas ce gain. Aucun temps ne passe pour ouvrir le menu.", tags=("passif","distance","récupération"))

SECONDARY = {}


def secondary(identifier, metrics, detail):
    assert identifier not in SECONDARY
    SECONDARY[identifier] = {"metrics": metrics, "detail": detail}


secondary("SF00", [fixed("Énergie par pas sans passif",10), fixed("Temps d'activation ou de sortie",1,"action"), fixed("Énergie d'activation",0), fixed("Recharge",0), custom("Facteur de portée visuelle avant SF02", "0,75 / (1 + 0,02max(0, Agilité−5))",lambda n,p: Fraction(3,4)/(1+Fraction(max(0,p['Agilité']-5),50))), fixed("Intensité des pas",50,"% de leur bruit ordinaire")],
    "Mode technologique. Pas réussi seulement : énergie payée avant la régénération normale du tour. Un pas furtif sans énergie est refusé sans déplacement ou perte de ressources. Attaque, sort offensif, pose de mine ou lancer de grenade termine le mode ; soins, consultation et récupération ne le terminent pas. Pas d'entretien au repos. Vision et écoute utilisent les règles réelles du moteur, pas des cercles à travers les murs ; une portée visuelle effective garde un minimum d'une case et ne supprime pas une position déjà mémorisée.")
secondary("SF01", [custom("Énergie par pas sur la base 10", "ceil(10 × max(0,1−n/10))",lambda n,p: ceil(10*max(Fraction(0),1-Fraction(n,10))))],
    "Même formule que la proposition précédente, avec 10 désormais proposé comme vrai coût de base. Rang 10 et plus : zéro énergie par pas, sans minimum artificiel de un. Le coût nul ne crée pas d'énergie et ne change pas les autres coûts.")
secondary("SF02", [custom("Facteur visuel supplémentaire", "(4/5) puissance n",lambda n,p: Fraction(4,5)**n)],
    "Même progression que précédemment : 80 % au rang 1, environ 10,7 % au rang 10. Agilité et le facteur de base SF00 se combinent. L'écoute ne devient pas aveugle ; les bruits de pas restent des événements réels. Bonus d'équipement proposés applicables au-delà de dix, avec le minimum d'une case de repérage.")
secondary("SM00", [damage("Explosion de comparaison",12,0,"technique"), fixed("Énergie",10), fixed("Recharge",1,"tour"), fixed("Rayon sans SM04",1,"case"), fixed("Armement après la pose",1,"tour complet"), fixed("Capacité sans SM01",1,"mine")],
    "Feu par défaut, poison ou froid selon passif appris. Pose sur la case de l'agent ou une case adjacente visible et franchissable, sans autre mine ni hostile déjà présent ; objets et alliés peuvent partager la case. Déclenchement par contact hostile une fois armée, y compris si un hostile reste dessus à l'armement. L'explosion peut toucher agent et alliés. Les mines ne déclenchent pas une chaîne d'explosions entre elles. Toute pose refusée laisse la plus ancienne mine intacte.")
secondary("SM01", [custom("Mines simultanées", "1 + ceil(n/3)",lambda n,p: 1+ceil(Fraction(n,3)))],
    "Conserve cinq mines au rang 10, cinq aussi au rang 12 ; proposition de poursuite par équipement : six au rang 13. La plus ancienne, armée ou non, est supprimée seulement après validation complète de la nouvelle pose. Pas d'explosion ou de remboursement à cette suppression.")
secondary("SM02", [linear("Dégâts supplémentaires de la variante poison",0,5,"%")],
    "Premier rang ouvre le choix poison ; rangs suivants renforcent explosion et nuage poison. Pas de poison périodique sans SM05. Le rang de ce passif ne modifie pas les variantes feu et glace.")
secondary("SM03", [linear("Dégâts supplémentaires de la variante glace",0,5,"%")],
    "Premier rang ouvre le froid ; progression identique à SM02. Le gel de zone nécessite SM05, laisse les attaques et autres actions non mobiles disponibles et ne devient pas un étourdissement général.")
secondary("SM04", [custom("Rayon total avec base une case", "1 + ceil(n/4)",lambda n,p: 1+ceil(Fraction(n,4)),"cases")],
    "Conserve le rayon supplémentaire précédent. Explosion et résidu utilisent la même aire radiale euclidienne, coupée par les murs et bornée par les limites de carte. Le rang effectif d'équipement poursuit la progression.")
secondary("SM05", [linear("Durée du résidu",3,1,"tours"), damage("Feu ou poison par tour avant bonus de variante",3,0,"technique")],
    "Durée égale à 2+n comme précédemment. Feu et poison : première résolution au prochain tour complet, une par acteur et par tour. Glace : immobilisation des hostiles dans la zone active, sans dégâts périodiques ajoutés ; leur mouvement redevient disponible après sortie forcée ou disparition de la zone. Les mines, leurs minuteries et leurs résidus sont proposés comme effacés à tout changement de carte, y compris portail bonus ; la suppression au changement de couche reste acquise. Retourner dans une carte ne recrée pas les anciennes mines.")
secondary("SG00", [damage("Dégâts explosifs de comparaison",8,0,"technique"), fixed("Énergie",8), fixed("Recharge",2,"tours"), fixed("Portée avant SG03",4,"cases"), fixed("Rayon de l'explosive ou aveuglante",1,"case")],
    "Une action : grenade résolue à l'impact sans tour d'attente ajouté. Énergie seule, aucun objet consommé. Explosion physique cinétique, dégâts modestes et alliés exposés. Les variantes utilitaires ont zéro dégât. Toutes utilisent leur aperçu réel avant confirmation, avec un trajet et une cible actuellement visibles.")
secondary("SG01", [fixed("Précision conservée",70,"%"), linear("Durée d'aveuglement",2,1,"tours"), power("Intelligence")],
    "Conserve la durée 1+n et la précision ×0,7. Le premier rang ouvre l'aveuglante. Acteurs de la zone soumis à leur résistance, alliés compris dans cette proposition. Plusieurs aveuglements ne multiplient pas entre eux la perte de précision ; la meilleure valeur active est retenue. Aucune action supprimée.")
secondary("SG02", [linear("Intensité sonore",8,2), fixed("Émissions successives",2,"tours")],
    "Conserve 6+2n. Le centre de l'impact est la destination de recherche des monstres capables de l'entendre ; ils suivent un chemin légal. Une menace immédiate à portée d'attaque reste prioritaire. La vraie propagation avec murs et seuils d'écoute détermine les cases atteintes. Le rang 1 doit porter plus loin que le rayon un des autres grenades en terrain ouvert : ce critère fait partie des essais moteur, sans remplacer le son par un cercle ou garantir l'audition d'un monstre sourd.")
secondary("SG03", [stepped("Portée de lancer supplémentaire",1,3,"cases")],
    "Conserve 1+floor(r/3), applicable à toutes les variantes de l'actif, pas aux armes lance-grenades. Le rayon sonore reste distinct du trajet de lancer et la cible reste actuellement visible.")
secondary("SG04", [linear("Dégâts supplémentaires des grenades",10,10,"%")],
    "Conserve 10n, actif secondaire et armes lance-grenades compris. Pas de dégâts artificiels pour les variantes sonore et aveuglante. La variante de l'arme conserve sa gestion de munitions et ses attributs d'attaque ordinaires.")

TIERS = {
 "Mêlée": [["B01","ME02","ME03"],["ME04","ME05","ME06"]],
 "Survie": [["B03","B02","SU03"],["SU04","SU05","SU06"]],
 "Drones": [["D59","D57","O03"],["D56","D58","DP01"],["D66","DP02"]],
 "Affaiblissements": [["O02","AF02","AF03"],["AF04","AF05","AF06"]],
 "Attaques à distance": [["DI01","DI02","DI05"],["DI03","DI04","DI06"]],
 "Esquive": [["T02","ES02","ES03"],["ES04","ES05","ES06"]],
 "Feu": [["FE01","FE02","FE06"],["FE03","FE04","FE05"]],
 "Froid": [["FR01","FR02","FR03"],["FR04","FR05","FR06"]],
 "Foudre": [["FO01","FO02","FO03"],["FO04","FO05","FO06"]],
 "Énergie occulte": [["A01","A02","A03"],["OC04","OC05","OC06"]],
 "Transformations": [["M01","TR61","M03"],["TR63","TR64","TR65"]],
 "Pouvoirs hybrides": [["M02","HY02","HY04"],["HY03","HY05","HY06"]],
 "Buffs de mêlée": [["BM01","BM03","BM04"],["BM02","BM05","BM06"]],
 "Buffs à distance": [["T01","T03","BD04"],["BD03","BD05","BD06"]],
}


def read(path):
    return (ROOT/path).read_text(encoding="utf-8")


def digest(path):
    return hashlib.sha256(read(path).encode("utf-8")).hexdigest()


def load(name):
    return json.loads((OUT/name).read_text(encoding="utf-8"))


def save(name, data):
    target = OUT/name
    assert not target.exists(), name
    target.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def serial(value):
    if isinstance(value, Fraction):
        return {"numerator":value.numerator,"denominator":value.denominator,"display":round(float(value),6)}
    return value


def display(value, unit):
    if isinstance(value,(int,Fraction)):
        value = Fraction(value)
        if value.denominator == 1:
            result = str(value.numerator)
        else:
            numeric = float(value)
            places = 4 if unit == "" and 0 < numeric < 1 else 1
            result = ("< 0,001" if 0 < numeric < 0.001 else f"{numeric:.{places}f}".rstrip("0").rstrip(".").replace(".",","))
    else:
        result = str(value)
    if result == "1":
        unit = {"cases": "case", "tours": "tour", "points": "point", "unités": "unité", "dégâts": "dégât"}.get(unit, unit)
    return result + (" " + unit if unit else "")


def catalogue():
    branch = None
    entries = []
    for line in read("docs/PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md").splitlines():
        if re.match(r"^## \d+ — ",line):
            branch = line.split(" — ",1)[1]
        match = re.match(r"^\| ([A-Z]+\d{2}) \| (.*?) \| ([AP]) \| (.*?) \| (.*?) \|$",line)
        if match:
            entries.append({"id":match[1],"name":match[2],"type":match[3],"branch":branch,"effect":match[4],"visual":match[5]})
    assert len(entries)==86 and len({row['id'] for row in entries})==86
    return entries


INTRO = """# Cadrage complet des compétences de l'agent

6 octobre 2026. Le joueur demande de terminer le cadrage avant de reprendre
les descriptifs et de décider ce qu'il garde ou modifie. Ce document achève
les propositions de règles des dix branches restantes et complète les
paramètres des trois arbres secondaires. Il ne crée aucun nouveau texte
roleplay, dessin, animation ou code de compétence.

Les noms, rôles et intentions déjà acceptés sont conservés. Chaque nouveau
chiffre et cas de fonctionnement demeure une proposition pour la revue
suivante. Les accords antérieurs sur des textes ou tableaux précis restent
valables ; achever ce document ne vaut pas approbation de son contenu.

## Références, rangs et attributs

Le catalogue principal contient 86 compétences dans 14 branches. Mêlée,
Survie, Drones et Affaiblissements possèdent déjà leurs lots de travail,
référencés ci-dessous. Les 60 autres compétences ont ici leurs paramètres.
Les 14 compétences secondaires sont distinctes des trois fonctionnalités
d'aperçu et d'inspection SP00, SP01, SP02, qui ne coûtent aucun point.

Par défaut : rang effectif 1, attributs à 5, sans autre compétence ou implant.
Exemple actuel : rang appris 10, +2 d'équipement, rang effectif 12 et
attributs à 10. Un actif secondaire reste acheté une seule fois : son exemple
actuel n'est pas présenté comme douze achats. Ses passifs seuls ont des rangs.

Les attaques de comparaison utilisent D = 20 dégâts par projectile ou coup,
avant attributs. Cette valeur fictive ne change pas les armes du jeu. Les
P désigne la précision ordinaire après attributs, équipement
et conditions, avant le modificateur propre à la technique. Les
fractions sont conservées ; les dégâts sont arrondis à l'entier inférieur à la fin
de leur calcul, les soins et absorptions au supérieur. Les pourcentages
affichés sont arrondis au dixième. Un chiffre est annoncé avant défenses,
pas comme une promesse de toucher ou d'infliger des dégâts nets.

Dans les formules, n est le rang effectif et r = n − 1. Les multiplicateurs
reprennent les propositions d'attributs : M_mêlée combine Force et Agilité,
M_distance Perception et Agilité, M_magie Volonté, M_technique Intelligence.
Au profil actuel, ils valent respectivement 1,1275, 1,1275, 1,1 et 1,1.
Les coûts affichés précèdent Canalisation et les implants. La présence d'un
autre passif n'est pas supposée pour embellir le résultat d'une compétence.

## Temps, disponibilité et ciblage

Un lancement normal prend une action et met immédiatement son effet en
place, avant les réactions ennemies. Le marquage T01 conserve son activation
sans passage de tour. Un effet explicitement différé garde son délai annoncé :
mine armée après un tour, explosion d'Éruption, progression de l'Orbe ou
tir de réaction après mise en surveillance. Ces délais ne demandent pas
une action cachée de préparation au joueur.

Le menu des actions affiche le temps restant de recharge ; le bouton grisé
est désactivé jusqu'à la fin du délai, avec survol conservé. Le libellé proposé
« Interférence — Recharge : 3 tours » est accepté lors de la dernière réponse.
Une commande distincte de maintien poursuit une canalisation déjà lancée :
elle n'est pas une nouvelle activation contournant sa recharge. Cette
présentation de maintien sera revue avec les descriptifs et l'interface.

Un refus pour cible, trajet, forme, ressource, munition ou recharge illégale
ne change ni ressources, ni position, ni temps, ni tirage. Les coûts d'un
lancement exécuté restent payés sur échec ou résistance. Les compétences
qui combinent deux systèmes valident tous leurs besoins avant la première
dépense. Consulter, déplacer le curseur ou annuler un choix ne simule rien.

Portées et zones sont limitées à la carte active, aux obstacles et aux
observations réellement disponibles. Les previews présentent seulement le
terrain connu et les acteurs observés. Les zones radiales emploient la
distance euclidienne ; A02 conserve explicitement ses trois par trois cases.
La propagation sonore reste celle du moteur. Une zone peut réellement
affecter un acteur caché sans que l'interface le révèle avant l'événement.

Les techniques DI01 à DI06 utilisent une arme à projectile physique sans
explosion ni cône de plombs, notamment pistolet, PM ou fusil compatible.
Elles définissent leur propre nombre de projectiles à partir du dégât d'un
seul projectile, et ne multiplient pas une rafale native une seconde fois.
L'arme sélectionnée et ses munitions restent utilisées. Les lance-grenades,
rayons, lasers, attaques naturelles et sorts ne sont pas implicitement
convertis en balles. Chaque famille sera déclarée compatible au contenu.

## Cumuls, sources et effets prolongés

Une même compétence ne multiplie pas ses propres buffs, leurres, surveillances
ou passages : un seul exemplaire, remplacé ou refusé selon sa règle. Des
sources différentes peuvent coexister. Les réductions de dégâts distinctes
se multiplient après armure et résistances ; elles ne s'additionnent pas
jusqu'à une immunité. L'absorption suit les protections précédentes, puis
Dernier sursaut conserve sa place finale. Chaque réserve d'absorption est
limitée et ne soigne pas les PV.

Dans les dix branches complétées ici, relancer un buff, un leurre, une
surveillance ou un couple de passages remplace sa version précédente après
validation, sans additionner les durées. Une création persistante propre
à un sort — mur, foyer, orbe, blizzard, orage, circuit ou prison — est limitée
à une par compétence et par agent ; la nouvelle remplace l'ancienne. Les
exceptions explicitement refusées restent refusées, notamment une forme
déjà active et Fusion défensive pendant son bouclier. Les mines gardent
leur capacité distincte et les quatre lots précédents leurs règles propres.

Pour des zones identiques de même élément qui se chevauchent sur une case,
la plus forte résolution périodique active est proposée, sans addition des
ticks. Une traversée et une attente sur la même zone ne donnent pas deux
résolutions du même effet dans un tour. La durée d'une zone n'est pas
réinitialisée par la mort d'une cible, sauf mention explicite de la compétence.

Les attaques secondaires automatiques — miroir, éclats, ripostes, renvois,
décharges passives — ne déclenchent pas une autre attaque secondaire. Les
bonus numériques permanents applicables restent pris en compte. Le nombre
de leurs déclenchements est limité explicitement ; menus et changements
d'équipement ne réinitialisent ni séries, ni réserves, ni recharges.

Le vol de vie se calcule sur les PV réellement retirés à un hostile, pas
sur dégâts affichés, murs, leurres ou victimes alliées. Les sources applicables
au même coup sont réunies avant un seul arrondi ; le soin proposé ne dépasse
ni 50 % des PV retirés ni les PV manquants. Une altération anti-soin peut
bloquer ce gain. Le renvoi ne renvoie pas du renvoi et ne vole pas de vie.

Les rangs de base, la géométrie et la durée des effets persistants sont
fixés au lancement. Leur auteur et leurs catégories sont conservés. Les
modificateurs conditionnels explicitement dynamiques, notamment Puissance
métamorphe, sont lus à la résolution de chaque dégât. Un retour humain ne
laisse pas ce bonus sur des zones anciennes. Les effets magiques de contrôle
utilisent puissance contre résistance occulte avec probabilités bornées
entre 5 % et 95 %. Aucun classement d'importance des monstres n'est fixé ici.

## Transformations communes

Cinq transformations temporaires, une seule forme à la fois. Chaque achat
de forme donne ses actions propres, sans points supplémentaires pour celles-ci.
Les armes ordinaires sont rangées pendant la forme ; leurs dégâts, munitions
et affixes d'attaque ne passent pas sur les attaques naturelles. Les pièces
d'armure, attributs, bonus de compétences et protections restent conservés.
La sortie retrouve l'équipement et n'offre ni soin ni recharge gratuite.

L'entrée et le retour volontaire prennent une action ; l'expiration est
automatique. Entrer dans une autre forme exige d'abord le retour humain,
et reste soumis à son coût et sa recharge. Les recharges des actions de
forme sont conservées lorsqu'on quitte puis reprend cette forme. Les PV
maximum ne changent pas du seul fait de se transformer. Les cinq dessins
existants sont conservés et les animations attendent leur revue native.

Les actions ordinaires de forme restent utilisables sous Silence, même
si elles ont un aspect magique. Ses actions spéciales sont des actifs et
peuvent être bloquées. Un sort appris indépendant d'une arme reste utilisable
si ses autres conditions sont satisfaites. Ancrage empêche les mouvements
forcés ; l'immobilisation de froid interdit les mouvements volontaires,
y compris téléportation, mais laisse attaques et actions sans déplacement.

## Proposition de paliers pour les quatorze branches

Pour six compétences, proposition de deux paliers de trois : entrée à zéro
rang appris et second palier à cinq rangs appris dans la branche. Pour les
huit drones, troisième palier de deux compétences à dix rangs appris.
Cette nouvelle grille adapte le catalogue accepté ; elle reste à valider,
sans rétablir automatiquement les anciennes grilles retirées.

Tous les choix restent apprenables. Seuls les rangs appris comptent, y
compris les rangs de départ offerts ; les bonus d'objets ne débloquent pas
les paliers. Un seuil ouvre un achat, il ne donne aucune compétence gratuite.
Les branches sont indépendantes et toutes les classes peuvent les développer.

"""


def render(entries):
    lines = [INTRO, "| Branche | Palier 0 | Palier 5 | Palier 10 |", "| --- | --- | --- | --- |"]
    for branch, tiers in TIERS.items():
        lines.append(f"| {branch} | {', '.join(tiers[0])} | {', '.join(tiers[1])} | {', '.join(tiers[2]) if len(tiers)>2 else '—'} |")
    lines.extend(["", "Cette grille conserve les compétences initiales connues dans le premier palier.",
                  "Le choix final du premier sort et de la première forme à la création sera",
                  "traité avec cette interface ; aucune affectation supplémentaire n'est imposée ici.",
                  "", "## Lots précédents conservés", "",
                  "| Branche | Document de travail |", "| --- | --- |",
                  "| Mêlée | [Lot 03](REVUE_MELEE_AGENT_LOT_03.md) |",
                  "| Survie | [Lot 04](REVUE_SURVIE_AGENT_LOT_04.md) |",
                  "| Drones | [Lot 05](REVUE_DRONES_AGENT_LOT_05.md) et [persistance](PROPOSITION_DRONES_PERSISTANTS_AGENT.md) |",
                  "| Affaiblissements | [Lot 06](REVUE_AFFAIBLISSEMENTS_AGENT_LOT_06.md) |", "",
                  "Leur statut reste celui de leurs revues : les textes et tableaux effectivement",
                  "acceptés restent retenus ; Maintenance et les détails non présentés restent",
                  "des propositions. Aucune règle de ces quatre lots n'est remplacée silencieusement.", ""])
    branch = None
    for entry in entries:
        if entry['id'] not in RULES:
            continue
        if entry['branch'] != branch:
            branch = entry['branch']
            lines.extend([f"## {branch}", ""])
        row = RULES[entry['id']]
        lines.extend([f"### {entry['id']} {entry['name']}", "", f"Type : {'actif' if entry['type']=='A' else 'passif'}. Rôle retenu : {entry['effect']}", "",
                      "| Paramètre | Par défaut | Actuel dans l'exemple |", "| --- | --- | --- |"])
        for label, unit, expression, fn in row['metrics']:
            lines.append(f"| {label} | {display(fn(1,REFERENCE),unit)} | {display(fn(12,CURRENT),unit)} |")
        lines.extend(["", "**Progression et calculs proposés :** " + " ; ".join(f"{label} = {expression}" for label, unit, expression, fn in row['metrics']) + ".", "",
                      "**Fonctionnement proposé :** " + row['detail'], "", "Intention visuelle conservée du catalogue : " + entry['visual'] + " Rendu non créé dans cette passe.", ""])
    lines.extend(["## Compléments des trois arbres secondaires", "",
                  "Ces quatorze compétences conservent les achats libres. Les actifs SF00,",
                  "SM00 et SG00 s'achètent une seule fois. Le rang 12 de leurs passifs signifie",
                  "dix rangs appris et deux d'équipement, pas douze achats de l'actif.", "",
                  "Les règles nouvelles ci-dessous complètent la [revue secondaire](REVUE_COMPETENCES_SECONDAIRES_AGENT.md)",
                  "sans approuver ses anciens textes. Ce document devient la proposition",
                  "technique actuelle pour les paramètres que cette revue laissait ouverts.", ""])
    secondary_source = read("docs/REVUE_COMPETENCES_SECONDAIRES_AGENT.md")
    secondary_names = dict(re.findall(r"^### ([A-Z]+\d{2}) (.+)$",secondary_source,re.M))
    for identifier,row in SECONDARY.items():
        lines.extend([f"### {identifier} {secondary_names[identifier]}", "",
                      "| Paramètre | Par défaut | Actuel dans l'exemple |", "| --- | --- | --- |"])
        fixed_active = identifier in {"SF00","SM00","SG00"}
        for label, unit, expression, fn in row['metrics']:
            lines.append(f"| {label} | {display(fn(1,REFERENCE),unit)} | {display(fn(1 if fixed_active else 12,CURRENT),unit)} |")
        lines.extend(["", "**Calculs proposés :** " + " ; ".join(f"{label} = {expression}" for label, unit, expression, fn in row['metrics']) + ".", "",
                      "**Fonctionnement proposé :** " + row['detail'], ""])
    lines.extend(["## Interfaces et ordre de travail", "",
                  "SP00 : aperçu de pose ou de lancer pour tous les types. SP01 : rayon d'une",
                  "mine seule au survol ou au curseur. SP02 : sélecteur des éléments connus",
                  "d'une case partagée. Ces trois fonctionnalités gardent les demandes déjà",
                  "retenues, sans achat ni tirage ; elles ne sont pas des compétences supplémentaires.", "",
                  "Le panneau des buffs/debuffs, le compteur d'hostiles, Combat/Accalmie/Calme",
                  "et les recharges du menu conservent la [revue de l'interface](REVUE_INTERFACE_EFFETS_AGENT.md).",
                  "Le survol ne révèle ni mine inconnue ni acteur caché. Une recharge restant",
                  "positive interdit l'activation, sans empêcher la consultation de la fiche.", "",
                  "La prochaine passe présentera les descriptions une par une : texte roleplay,",
                  "effets par défaut, valeurs réelles actuelles et indications de durée/recharge.",
                  "Le joueur pourra garder ou modifier chacune. Les noms déjà retenus ne sont",
                  "pas renommés automatiquement ; toute révision conserve son identifiant.", "",
                  "Les compétences et animations seront ensuite intégrées après leurs accords",
                  "respectifs. Le personnage configurable de la salle de test, avec niveaux et",
                  "respécialisation illimités, vient après leur implémentation. Les monstres",
                  "auront un catalogue propre selon rôle, niveau et importance, à définir lors",
                  "de leur phase ; il n'est pas remplacé par les arbres du joueur.", "",
                  "Le cadrage chiffré est complet comme proposition. L'équilibrage final, les",
                  "descriptions, les illustrations, les contrôles d'interface et la validation",
                  "en partie restent des étapes séparées. Aucune compétence n'est déclarée",
                  "implémentée ou testée dans le moteur par ce document.", ""])
    return "\n".join(lines)


def check_and_sample(entries):
    remaining = {row['id'] for row in entries if row['branch'] not in {'Mêlée','Survie','Drones','Affaiblissements'}}
    assert set(RULES)==remaining and len(RULES)==60 and len(SECONDARY)==14
    all_tiers = [identifier for tiers in TIERS.values() for tier in tiers for identifier in tier]
    assert len(all_tiers)==len(set(all_tiers))==86
    assert set(all_tiers)=={row['id'] for row in entries}
    for entry in entries:
        assert entry['id'] in [value for tier in TIERS[entry['branch']] for value in tier]
    samples = []
    for group,rules in [('primary',RULES),('secondary',SECONDARY)]:
        for identifier,row in rules.items():
            fixed_active = identifier in {'SF00','SM00','SG00'}
            for rank in ((1,) if fixed_active else (1,5,10,12,100,1000)):
                values = {}
                for label,unit,expression,fn in row['metrics']:
                    value = fn(rank,REFERENCE)
                    if isinstance(value,(int,Fraction)):
                        assert value>=0,(identifier,label,rank)
                    values[label] = {'value':serial(value),'unit':unit}
                samples.append({'group':group,'id':identifier,'rank':rank,'profile':'reference','values':values})
            samples.append({'group':group,'id':identifier,'rank':1 if fixed_active else 12,'profile':'current',
                            'values':{label:{'value':serial(fn(1 if fixed_active else 12,CURRENT)),'unit':unit}
                                      for label,unit,expr,fn in row['metrics']}})
    def get(identifier,label,rank=12,profile=CURRENT):
        row = RULES.get(identifier) or SECONDARY[identifier]
        return next(fn(rank,profile) for name,unit,expr,fn in row['metrics'] if name==label)
    assert get('A01','Dégâts directs')==45
    assert get('A02','Dégâts par acteur')==30
    assert get('M01','Griffe ordinaire')==33
    assert get('M01','Durée')==9
    assert get('M02','Tours ajoutés par impact admissible')==2
    assert get('M02','Tours ajoutés au maximum par transformation')==4
    assert get('T01',"Temps d'activation")==0
    assert get('T01','Précision supplémentaire contre la marque',1)==15
    assert get('T03','Dégâts supplémentaires à distance',1)==10
    assert get('M03','Dégâts supplémentaires sous forme',1)==10
    assert get('A03','Réduction des coûts magiques',1)==20
    assert get('SM01','Mines simultanées',10)==5
    assert get('SM01','Mines simultanées',13)==6
    assert get('SM05','Durée du résidu',10)==12
    assert get('SG01',"Durée d'aveuglement",10)==11
    assert get('SG02','Intensité sonore',10)==26
    assert get('SF01','Énergie par pas sur la base 10',10)==0
    assert get('SF01','Énergie par pas sur la base 10',1000)==0
    for rank in (1,10,12,100,1000):
        assert get('A03','Réduction des coûts magiques',rank)<50
        assert get('TR63','Réduction des dégâts directs',rank)<35
        assert get('OC05','Vie rendue sur les PV effectivement retirés',rank)<25
        assert get('BM02','Vie rendue sur les PV retirés au contact',rank)<20
        assert get('BD05','Dégâts du projectile dévié',rank)<80
        assert get('M02','Tours ajoutés au maximum par transformation',rank)<=6
        assert get('FO01','Cibles maximales',rank)<=6
        assert get('DI02','Projectiles sur une cible',rank)==get('DI02','Munitions',rank)
    return {'status':'passed_for_design_calculations_not_game_runtime', 'primary_count':60,'secondary_count':14,
            'sample_count':len(samples),'checks':['86 unique catalogue IDs covered by branch-local tiers',
              '60 remaining primary and 14 secondary skills have finite nonnegative reference/current values',
              'legacy numeric anchors retained for A01/A02/A03/M01/T01/T03/M03',
              'free stealth at rank 10, five mines at rank 10, and existing grenade curves retained',
              'percentage protections and secondary chains bounded at sample ranks through 1000'],
            'samples':samples}


def main():
    outputs = ('remaining-skill-design-calculations-01.json','review-complete-skill-design-01.json',
               'cooldown-acceptance-and-complete-design-decisions.json','review-interface-effects-06.json',
               'review-interface-effects-revision-05.md','review-description-format-06.json',
               'review-description-format-revision-05.md')
    assert not (ROOT/DOCUMENT).exists()
    assert all(not (OUT/name).exists() for name in outputs)
    plans = [('review-interface-effects-05.json','review-interface-effects-06.json','review-interface-effects-revision-05.md',{'HUD05'}),
             ('review-description-format-05.json','review-description-format-06.json','review-description-format-revision-05.md',{'CD03'})]
    for old_name,new_name,snapshot,allowed in plans:
        old=load(old_name)
        assert digest(old['document'])==old['document_sha256']
    entries=catalogue()
    checks=check_and_sample(entries)
    source=render(entries)
    assert '\ufffd' not in source
    spec=importlib.util.spec_from_file_location('review',OUT/'build_drone_persistence_review.py')
    review=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(review)
    save(outputs[2],{'date':'2026-10-06','runtime_integration':'not_integrated',
                    'cooldown_user_reply':'parfait tu peux continuer',
                    'accepted_cooldown_presentation':['Interférence — Recharge : 3 tours','gray disabled action button','hover consultation remains possible','disabled activation consumes no energy or turn'],
                    'latest_user_instruction':'tu sais quoi tu continues jusqu’à terminer puis ensuite on commencer à faire les descriptifs des compétences et là je pourrais voir si on garde ou modifie :) .',
                    'work_scope':'finish all remaining skill rule proposals before creating additional roleplay descriptions',
                    'new_numbers_approval':'pending later skill-by-skill user review',
                    'new_roleplay_copy_created':False,'native_visuals_created':False,'test_character_timing':'after all skills and animations are implemented'})
    (ROOT/DOCUMENT).write_text(source,encoding='utf-8')
    save(outputs[0],checks)
    for old_name,new_name,snapshot,allowed in plans:
        previous=load(old_name)
        original=read(previous['document'])
        (OUT/snapshot).write_text(original,encoding='utf-8')
        if 'interface' in old_name:
            updated=original.replace('**Libellés proposés :** « Interférence — Recharge : 3 tours », puis\n« Interférence — Recharge : 1 tour ». Une fois la recharge terminée,',
                                     '**Libellé accepté :** « Interférence — Recharge : 3 tours ». La déclinaison\nsingulière proposée est « Interférence — Recharge : 1 tour ». Une fois la recharge terminée,')
            updated=updated.replace('**Rendu proposé, à revoir :** conserver l\'action visible et griser son bouton',
                                     '**Présentation retenue :** conserver l\'action visible et griser son bouton')
            old_note='HUD05 enregistre une demande explicite d\'affichage et de blocage des actions\nen recharge. Son libellé exact, son rendu grisé et les détails de présentation\nci-dessus restent à valider séparément.'
            new_note='HUD05 enregistre une demande explicite d\'affichage et de blocage des actions\nen recharge. Le joueur accepte ensuite le libellé présenté, le bouton grisé,\nle blocage et le survol en répondant « parfait tu peux continuer ». Les détails\nnon présentés et le rendu natif du menu restent à revoir.'
            assert old_note in updated
            updated=updated.replace(old_note,new_note)
        else:
            updated=original.replace('continue avec cinq actifs et un passif. Leurs nouveaux textes, chiffres\net cas détaillés attendent une validation individuelle.',
                                     'continue avec cinq actifs et un passif. Les textes, tableaux et limites\nprésentés sont acceptés comme base de travail avec activation immédiate ;\nles détails non présentés et les animations restent à revoir.')
            updated=updated.rstrip()+'\n\nLe joueur demande maintenant de terminer le [cadrage des branches restantes](CADRAGE_COMPETENCES_AGENT_COMPLET.md)\navant de poursuivre les descriptifs. Les nouvelles descriptions roleplay sont\nreportées à la revue suivante, où il choisira ce qu’il conserve ou modifie.\nLes accords existants restent valables ; les nouvelles règles chiffrées\nsont des propositions, pas des effets intégrés.\n'
        current=deepcopy(previous)
        prior={row['id']:row for row in previous['blocks']}
        rows=[]
        for hashes in review.sections(updated):
            row=deepcopy(prior[hashes['id']])
            if row['block_sha256']!=hashes['block_sha256']:
                assert row['id'] in allowed
                row['revision']+=1
                for key in ('block_sha256','text_sha256','mechanic_sha256'):
                    if key in row:
                        row[key]=hashes[key]
            if row['id']=='HUD05':
                row['text_approval']='displayed_cooldown_label_accepted_other_copy_pending'
                row['mechanic_approval']='displayed_cooldown_block_countdown_hover_and_atomic_refusal_accepted_other_details_pending'
                row['visual_intention_approval']='gray_disabled_action_presentation_accepted_native_render_pending'
            rows.append(row)
        (ROOT/previous['document']).write_text(updated,encoding='utf-8')
        current.update(batch_revision=previous['batch_revision']+1,document_sha256=digest(previous['document']),blocks=rows,
                       previous_manifest=PREFIX+old_name,previous_document_snapshot=PREFIX+snapshot,
                       latest_decisions=DECISIONS,latest_decisions_sha256=digest(DECISIONS))
        if 'description-format' in new_name:
            current['complete_design_document']=DOCUMENT
            current['next_stage']='individual descriptions and approval after completed numeric design'
        else:
            current['status']='displayed_cooldown_menu_presentation_accepted_native_menu_pending'
        save(new_name,current)
    blocks=[]
    for hashes in review.sections(source):
        blocks.append({**hashes,'revision':1,'roleplay_copy_created':False,
                       'name_approval':'existing_catalogue_or_secondary_name_preserved',
                       'numeric_rules_approval':'proposed_pending_later_individual_user_review',
                       'detailed_conditions_approval':'proposed_pending_later_individual_user_review',
                       'visual_approval':'pending_native_review','runtime_integration':'not_integrated'})
    assert len(blocks)==74
    save(outputs[1],{'date':'2026-10-06','document':DOCUMENT,'document_sha256':digest(DOCUMENT),
                    'runtime_integration':'not_integrated','batch_revision':1,'latest_decisions':DECISIONS,'latest_decisions_sha256':digest(DECISIONS),
                    'primary_catalogue_count':86,'new_primary_rule_blocks':60,'secondary_rule_blocks':14,
                    'earlier_primary_rule_blocks_preserved':26,'blocks':blocks,
                    'reference_profile':REFERENCE,'current_example_profile':CURRENT,
                    'calculations':PREFIX+outputs[0],'calculations_sha256':digest(PREFIX+outputs[0]),
                    'tier_proposal':TIERS,'tier_thresholds':[0,5,10],
                    'tier_approval':'pending_later_user_review','descriptions':'deferred_by_explicit_user_instruction'})
    print(json.dumps({'remaining_primary_skills':60,'secondary_skills':14,'tier_branches':14,
                      'calculated_samples':checks['sample_count'],'new_roleplay_texts':0,'runtime_changes':0},ensure_ascii=False))


if __name__=='__main__':
    main()
