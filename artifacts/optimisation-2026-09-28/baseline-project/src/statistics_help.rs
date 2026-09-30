//! Player-facing glossary, separate from input bindings and simulation rules.
use super::*;

pub(super) struct StatisticSection {
    pub title: &'static str,
    pub entries: Vec<(String, String)>,
}

fn entries(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|(name, text)| ((*name).to_owned(), (*text).to_owned()))
        .collect()
}

impl AsciiApp {
    #[cfg(debug_assertions)]
    pub(super) fn write_statistics_review(&self, path: &std::path::Path) -> Result<(), String> {
        let mut text = String::from(
            "# Statistiques et jauges — textes à relire\n\nChaque entrée ci-dessous reprend le texte de l'onglet F1 « Statistiques et jauges ». La numérotation sert à commenter chaque description individuellement ; elle n'apparaît pas dans le jeu.\n\nLes réserves, attributs, seuils et rythmes de récupération chiffrés sont ceux d'une partie de diagnostic. Dans le jeu, ils sont lus dans la partie en cours ; les anciennes parties peuvent avoir d'autres règles. Ces chiffres illustrent l'affichage et ne constituent pas de nouvelles propositions d'équilibrage.\n\n",
        );
        let mut number = 1;
        for section in self.statistics_help() {
            text.push_str(&format!("## {}\n\n", section.title));
            for (name, description) in section.entries {
                text.push_str(&format!("### {number:02}. {name}\n\n{description}\n\n"));
                number += 1;
            }
        }
        std::fs::write(path, text).map_err(|error| error.to_string())
    }

    pub(super) fn statistics_help(&self) -> Vec<StatisticSection> {
        let energy = self.game.player_energy();
        let regeneration = self.game.rules().player_energy_regeneration;
        let mut resources = entries(&[
            (
                "Points de vie (PV)",
                "Votre état actuel sur votre maximum. Les dégâts retirent des PV ; à zéro, la partie est terminée. Augmenter le maximum avec un bonus d'équipement n'est pas un soin. Les effets du passage de niveau sont indiqués dans Progression.",
            ),
            (
                "Munitions",
                "Stock commun aux armes qui utilisent des projectiles. Chaque tir ou projectile d'une salve prélève son coût, même s'il manque sa cible. Le coût de l'arme active est indiqué près de son nom. Changer d'arme ne recharge pas le stock. La barre utilise une échelle visuelle par tranches de 40 : ce n'est pas une capacité maximale. Les anciennes parties à chargeurs affichent plutôt le stock et la capacité de l'arme.",
            ),
        ]);
        resources.insert(1, ("Énergie (E)".to_owned(), format!(
            "Réserve commune aux techniques et aux équipements alimentés : {} / {} actuellement. {} Ouvrir un menu ne recharge rien. À zéro, seules les actions demandant de l'énergie sont bloquées ; cela ne tue pas le personnage.",
            energy.available(), energy.capacity(),
            if regeneration > 0 { format!("Vous récupérez {regeneration} point{} d'énergie à chaque tour terminé, sans dépasser la capacité. Attendre fait aussi agir le monde.", if regeneration > 1 { "s" } else { "" }) }
            else { "Cette partie n'accorde pas de récupération automatique à la fin d'un tour.".to_owned() }
        )));
        let bandwidth = self.game.player_bandwidth().map_or_else(
            || "Cette ressource n'est pas active dans cette partie.".to_owned(),
            |b| {
                format!(
                    "Actuellement : {} occupés, {} libres, capacité {}.",
                    b.occupied(),
                    b.available(),
                    b.capacity()
                )
            },
        );
        resources.push(("Bande passante (B)".to_owned(), format!("Capacité de contrôle partagée par vos drones, certaines liaisons et certains effets maintenus. La jauge montre la part OCCUPÉE : plus elle est pleine, moins il reste de place. Une réservation est libérée lorsque son effet ou sa liaison prend fin ; attendre ne libère pas une liaison toujours active. Apprendre une technique ne réserve rien à lui seul. {bandwidth}")));
        let heat = self.game.player_heat().map_or_else(
            || "Cette ressource n'est pas active dans cette partie.".to_owned(),
            |h| format!("Actuellement : {} ; alerte à {} ; seuil critique à {} ; dissipation de {} par tour terminé.", h.current(), h.alert_threshold(), h.critical_threshold(), h.dissipation_per_phase()));
        resources.push(("Chaleur (H) et dissipation".to_owned(), format!("Chaleur accumulée par certaines techniques et certains modules. Elle redescend à la fin des tours grâce à la dissipation. Moins il y en a, mieux c'est. La barre va jusqu'au seuil critique, mais la valeur peut le dépasser : ce seuil n'est pas une capacité maximale. {heat}")));
        resources.push(("Alerte thermique et seuil critique".to_owned(), "La jauge devient ambre au seuil d'alerte, puis rouge au seuil critique. Pour la réserve personnelle affichée ici, franchir ces seuils signale le danger sans infliger, à lui seul, une perte de PV. Les conséquences dépendent du module ou de l'effet : un module surcadencé peut s'user au-dessus de son seuil sûr et refuser une attaque qui dépasserait sa limite. Une surchauffe hostile appliquée à un système électronique possède ses propres dégâts thermiques.".to_owned()));

        let primary = PrimaryAttribute::ALL
            .into_iter()
            .map(|attribute| {
                let title = primary_attribute_label(attribute).to_owned();
                let current = self
                    .game
                    .player_effective_primary_attributes()
                    .map(|values| values.value(attribute));
                let text = format!(
                    "{}{}",
                    primary_attribute_description(attribute),
                    current.map_or(String::new(), |v| format!(" Votre valeur actuelle : {v}."))
                );
                (title, text)
            })
            .collect();

        let mut sections = vec![
            StatisticSection {
                title: "Jauges et ressources",
                entries: resources,
            },
            StatisticSection {
                title: "Attributs primaires",
                entries: primary,
            },
            StatisticSection {
                title: "Défenses et résistances",
                entries: entries(&[
                    (
                        "Armure",
                        "Réduit les dégâts physiques des impacts compatibles, après prise en compte de la pénétration et de la fragilisation. Les dégâts cinétiques, perforants et explosifs rencontrent cette protection. L'armure n'est ni de l'esquive ni une réserve de PV.",
                    ),
                    (
                        "Fragilisation",
                        "Réduit temporairement l'armure prise en compte. La valeur et la durée de l'effet indiquent la protection perdue ; elles ne représentent pas des dégâts immédiats.",
                    ),
                    (
                        "Esquive",
                        "Score opposé à la précision des attaques ciblées qui peuvent être évitées. Une valeur de 10 n'est pas une probabilité de 10 %. Une zone dangereuse ou une attaque sans jet d'esquive suit ses propres règles.",
                    ),
                    (
                        "Stabilité",
                        "Aide à résister aux interruptions et aux perturbations qui testent cette défense. Ce score n'est pas une réduction générale des dégâts et ne remplace pas l'armure.",
                    ),
                    (
                        "Défense numérique",
                        "Réduit les chances qu'une intrusion ou un programme hostile franchisse les protections logicielles d'un système compatible. Ne protège pas des décharges électriques et ne retire pas un programme déjà implanté.",
                    ),
                    (
                        "Résistance thermique",
                        "Réduit les dégâts de feu et les autres dégâts thermiques. Elle ne vide pas la jauge de chaleur et n'améliore pas la dissipation.",
                    ),
                    (
                        "Résistance électrique",
                        "Réduit les dégâts électriques. Elle ne remplace pas la défense numérique contre un programme hostile.",
                    ),
                    (
                        "Résistance chimique",
                        "Réduit les dégâts chimiques. Les autres effets d'une substance gardent leurs propres conditions.",
                    ),
                    (
                        "Résistance aux radiations",
                        "Réduit la composante Radiation des dégâts reçus.",
                    ),
                    (
                        "Résistance à la corruption",
                        "Réduit la composante Corruption des dégâts reçus.",
                    ),
                    (
                        "Pourcentages de résistance",
                        "Une valeur positive réduit les dégâts du type indiqué ; une valeur négative les augmente. Zéro n'apporte pas de réduction. Chaque composante d'un impact est traitée selon son type.",
                    ),
                ]),
            },
            StatisticSection {
                title: "Attaques, armes et délais",
                entries: entries(&[
                    (
                        "Précision et chance de toucher",
                        "La précision de l'arme, les attributs, les états et la défense de la cible participent au jet. Le pourcentage Touche de la fiche de personnage est une référence sans cible, pas une garantie contre chaque adversaire.",
                    ),
                    (
                        "Dégâts et types",
                        "La fiche indique les dégâts avant les protections de la cible. Un impact peut combiner plusieurs types. KIN : cinétique ; PIR : perforant ; EXP : explosif ; THR : thermique ; ELE : électrique ; CHM : chimique ; RAD : radiation ; COR : corruption.",
                    ),
                    (
                        "Pénétration d'armure",
                        "Ignore une partie de l'armure pour cet impact. Elle ne détruit pas l'équipement et ne diminue pas les résistances spécialisées.",
                    ),
                    (
                        "Impact disponible, transmis et plafond",
                        "Au corps à corps, la Puissance contribue à l'impact disponible. L'arme n'en transmet que ce que son matériau supporte. L'impact transmis intervient dans les dégâts physiques ; augmenter la Puissance au-delà du plafond de cette arme n'augmente pas cette part.",
                    ),
                    (
                        "Portée, rayon et zone",
                        "La portée fixe la distance maximale de l'action. Le rayon et la forme décrivent les cases affectées. Visibilité, obstacles, trajectoire et conditions de ciblage restent applicables ; la précision n'allonge pas la portée.",
                    ),
                    (
                        "Préparation, action et récupération",
                        "La préparation précède l'effet ; l'action le résout ; la récupération est le délai imposé ensuite. Une attente ou un déplacement peut faire avancer la récupération si l'action est autorisée. Les valeurs indiquent la durée propre à la technique ou à l'arme.",
                    ),
                    (
                        "Durée, recharge et entretien",
                        "La durée indique combien de temps un effet persiste. La recharge indique le délai avant de réutiliser une technique. Un coût d'entretien est prélevé pendant le maintien, en plus du coût initial lorsqu'il existe. Consulter la fiche pour les conditions d'arrêt.",
                    ),
                    (
                        "E, H et B dans un coût",
                        "E est l'énergie dépensée ; H la chaleur ajoutée ; B la bande passante réservée. Les munitions et autres consommables sont indiqués séparément. Ces nombres décrivent des usages différents et ne s'additionnent pas en une seule réserve.",
                    ),
                ]),
            },
            StatisticSection {
                title: "Équipement, perception et compagnons",
                entries: entries(&[
                    (
                        "Base, bonus et valeur actuelle",
                        "Base désigne les attributs du personnage ; les bonus viennent de l'équipement et des effets applicables. La valeur actuelle tient compte de ces contributions et de leurs limites. Un bonus de capacité ne remplit pas automatiquement la réserve concernée.",
                    ),
                    (
                        "Durabilité",
                        "État propre d'un équipement, d'un composant ou d'un objet destructible. Une pièce défaillante peut perdre ses fonctions. La réparer ne soigne pas automatiquement les PV de son porteur.",
                    ),
                    (
                        "Places d'inventaire et quantité",
                        "Les places comptent les entrées et piles de votre sac, par rapport à sa capacité. La quantité est le nombre d'exemplaires dans une pile ; une pile peut contenir plusieurs objets. Les emplacements d'équipement déterminent où un objet compatible peut être porté.",
                    ),
                    (
                        "Masse, ancrage et traction",
                        "La masse du corps et de sa charge, ainsi que l'ancrage, interviennent dans la résistance aux déplacements forcés. La traction limite la masse qu'un dispositif peut déplacer. Ces valeurs ne réduisent pas directement les dégâts.",
                    ),
                    (
                        "Temps de déplacement",
                        "Temps nécessaire pour parcourir une case dans votre état actuel. Certains états ou terrains peuvent le modifier ; les autres acteurs continuent d'agir pendant ce temps. La Coordination n'accélère pas toutes les actions.",
                    ),
                    (
                        "Portée des capteurs",
                        "Distance maximale d'observation par vos capteurs. Les obstacles et le canal utilisé restent déterminants. La Perception améliore les observations compatibles, mais agrandir la fenêtre ou augmenter cet attribut ne permet pas de voir à travers les murs.",
                    ),
                    (
                        "Signature, discrétion et détection",
                        "Une signature est un indice émis, par exemple un bruit ou une silhouette. La discrétion et la détection sont comparées selon le canal et l'effet concernés. Masquer un canal ne rend pas invisible sur tous les autres, et n'efface pas une observation passée.",
                    ),
                    (
                        "PV, batterie et liaison d'un compagnon",
                        "Les PV et la batterie appartiennent au compagnon, pas au joueur. La liaison conditionne les ordres à distance. Sa bande passante peut rester réservée pendant son contrôle ; attendre ne crée ni batterie ni actions supplémentaires pour lui.",
                    ),
                    (
                        "Distance et informations inconnues",
                        "Les distances sont exprimées en cases. Les statistiques d'une cible ne sont affichées que lorsqu'elles sont connues, notamment après analyse. Une donnée inconnue ne signifie pas zéro.",
                    ),
                ]),
            },
            StatisticSection {
                title: "Progression",
                entries: entries(&[
                    (
                        "Niveau et expérience (XP)",
                        "L'expérience cumulée permet d'atteindre les seuils de niveau. La jauge XP représente l'avancement entre le seuil du niveau actuel et celui du suivant ; le compteur affiche l'expérience cumulée et le prochain seuil.",
                    ),
                    (
                        "Points de compétence",
                        "Servent à apprendre les techniques et améliorations dont les conditions sont remplies. Le coût dépend des choix déjà effectués dans la discipline et apparaît avant l'achat. Vous pouvez conserver ces points pour plus tard.",
                    ),
                    (
                        "Conditions et améliorations",
                        "Une technique peut demander un niveau, des attributs et un apprentissage préalable. Une amélioration modifie une technique connue. Les fiches indiquent les effets, coûts, limites et conditions d'utilisation de chaque choix.",
                    ),
                ]),
            },
        ];
        let rules = self.game.rules();
        if rules.maintained_energy_reservations {
            sections[0].entries[1].1 = format!(
                "L'énergie alimente les techniques et les équipements. Actuellement : {} E libres, {} E réservés, capacité totale {} E. Une action ponctuelle dépense son coût ; un effet maintenu immobilise son coût jusqu'à son arrêt. Vous récupérez {} E à chaque tour terminé, uniquement dans la partie libre. Marcher ou attendre fait passer le temps ; ouvrir un menu ne recharge rien. À zéro énergie libre, les actions payantes sont bloquées, mais les effets maintenus continuent.",
                energy.available(),
                energy.reserved(),
                energy.capacity(),
                regeneration
            );
            sections[0].entries[3] = (
                "Compagnons".to_owned(),
                format!(
                    "{} / {} places occupées. Les compagnons recrutés, achetés ou invoqués occupent chacun une place, même s'ils restent dans une autre zone. Les PNJ qui vous accompagnent pour une quête ne comptent pas. À la limite, aucun nouveau compagnon ne peut être ajouté. Sa mort ou son renvoi libère une place. Cliquez sur la jauge pour gérer les compagnons présents.",
                    self.game.player_companion_count(),
                    rules.player_companion_limit.unwrap_or(0)
                ),
            );
            sections[0].entries[4] = ("Énergie réservée et couleurs de la jauge".to_owned(), "Le bleu représente l'énergie disponible, le gris l'énergie dépensée qui peut se régénérer, et le violet hachuré à droite l'énergie réservée. Chaque effet maintenu réserve son propre coût, affiché sur sa fiche : il faut disposer de cette énergie pour l'activer. Plusieurs effets peuvent coexister. Leur arrêt libère leur réservation sans rembourser vos autres dépenses. Cliquez sur la jauge pour voir chaque réservation et arrêter un effet. Exemple : sur 100 E, 20 E réservés limitent la partie utilisable à 80 E. Si vous en dépensez 30, il reste 50 E libres ; arrêter l'effet rend les 20 E réservés, soit 70 E libres.".to_owned());
            sections[0].entries.truncate(5);
            sections[2].entries[5].1 =
                "Réduit les dégâts de feu et les autres dégâts thermiques reçus.".to_owned();
            sections[3].entries[6].1 = "La durée limite la persistance d'un effet ; la recharge fixe le délai avant sa réutilisation. Un effet maintenu réserve de l'énergie sans la dépenser à chaque tour. Sa fiche précise les autres conditions d'arrêt. Changer de zone interrompt les liaisons et les dispositifs maintenus dans la zone quittée ; le camouflage peut vous suivre.".to_owned();
            sections[3].entries[7] = ("Coût dépensé ou réservé".to_owned(), "E dépensés : retirés de l'énergie libre, puis récupérables par régénération. E réservés : immobilisés pendant l'effet et rendus à son arrêt, sans paiement initial supplémentaire. Une invocation de compagnon utilise une place et dépense son énergie de création. La batterie du compagnon est ensuite indépendante. Munitions et consommables suivent leurs propres coûts.".to_owned());
            sections[4].entries[7].1 = "Les PV et la batterie appartiennent au compagnon, pas au joueur. Sa liaison conditionne les ordres à distance ; une place reste occupée tant qu'il fait partie du groupe. Certains ordres maintenus réservent en plus de l'énergie du joueur : leur fiche l'indique. Attendre ne recharge pas automatiquement la batterie du compagnon.".to_owned();
        }
        let progression = &mut sections.last_mut().unwrap().entries[0].1;
        if rules.player_hit_points_per_level > 0 {
            progression.push_str(&format!(
                " Dans cette partie, chaque niveau gagné ajoute {} PV au maximum.",
                rules.player_hit_points_per_level
            ));
        }
        progression.push_str(if rules.player_full_heal_on_level_up {
            " Le passage de niveau restaure aussi tous vos PV."
        } else {
            " Le passage de niveau ne restaure pas automatiquement vos PV."
        });
        sections
    }

    pub(super) fn draw_statistics_help(&self, body: Rect) {
        crate::ui_theme::begin_text_pane(body, self.ux.help_scroll.offset);
        let mut y = body.y + 24.0;
        let mut offsets = [0.0; 6];
        for (index, section) in self.statistics_help().into_iter().enumerate() {
            offsets[index] = y - body.y - 24.0;
            draw_text_bold(section.title, body.x, y, 22.0, UiTheme.accent());
            y += 31.0;
            for (name, description) in section.entries {
                draw_text_bold(name, body.x, y, 17.0, UiTheme.text());
                y = draw_wrapped_text(
                    &description,
                    body.x,
                    y + 25.0,
                    body.w - 18.0,
                    usize::MAX,
                    16,
                    UiTheme.muted(),
                ) + 25.0;
            }
            y += 16.0;
        }
        crate::ui_theme::end_text_pane();
        self.ux.statistics_section_offsets.set(offsets);
        self.ux.help_scroll.finish(body, y);
    }
}
