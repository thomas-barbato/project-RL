//! Stable indices for the urban and science-fiction editor catalogues.
pub const MATERIAL_SLOTS: usize = 28;
pub const FURNITURE_COUNT: usize = 120;
pub const WALL_STYLE_COUNT: usize = 8;
pub const WALL_STYLES: [&str; 8] = [
    "Métal", "Brique", "Crépi", "Laser", "Caméra", "Béton", "Pierre", "Grillage",
];
pub const OBJECT_GROUPS: [&str; 7] = [
    "Tout",
    "Ville",
    "Tech",
    "Serveurs",
    "Habitat",
    "Armurerie",
    "Nature",
];
pub const NATURE_OBJECTS: [&str; 16] = [
    "Arbre feuillu clair",
    "Arbre feuillu sombre",
    "Conifère",
    "Arbre mort",
    "Buisson vert",
    "Buisson ocre",
    "Grand rocher",
    "Petits rochers",
    "Roseaux",
    "Tronc couché",
    "Herbes et fleurs",
    "Souche",
    "Entrée de grotte rocheuse",
    "Entrée de grotte envahie",
    "Affleurement rocheux",
    "Muret en ruine",
];
pub fn has_directions(index: usize) -> bool {
    matches!(index, 1 | 4 | 65 | 72..=75)
}
pub fn default_size(index: usize) -> (u8, u8) {
    match index {
        4 | 65 => (2, 1),
        72..=75 => (1, 2),
        _ => (1, 1),
    }
}
pub const SF_OBJECTS: [&str; 48] = [
    "Baie de serveurs noire",
    "Baie de serveurs claire",
    "Baie câblée ouverte",
    "Stockage de données",
    "Refroidissement",
    "Onduleur",
    "Réseau et commutateurs",
    "Distribution de câbles",
    "Bureau informatique",
    "Console opérateur",
    "Caméra sur pied",
    "Caméra sur support",
    "Station d'analyse",
    "Réfrigérateur de labo",
    "Cœur d'alimentation",
    "Atelier de robotique",
    "Lit bleu",
    "Lit rouge",
    "Lit gris",
    "Lit orange",
    "Baril rouge",
    "Baril bleu",
    "Baril jaune",
    "Baril vert",
    "Escalier béton descendant",
    "Escalier métal descendant",
    "Trappe et échelle",
    "Capsule de repos",
    "Cuve de stase",
    "Vestiaires",
    "Lavabo technique",
    "Cuisine technique",
    "Râtelier de fusils",
    "Armoire de pistolets",
    "Support d'armure",
    "Étagère de casques",
    "Caisse de munitions ouverte",
    "Coffre de munitions",
    "Établi d'armurier",
    "Outils d'armurier",
    "Chargeur de cellules",
    "Armes à énergie",
    "Armoire d'arme lourde",
    "Support de boucliers",
    "Coffre d'armes",
    "Console tactique",
    "Cible d'entraînement",
    "Drone sur station",
];

pub fn default_blocking(sprite: usize, furniture: bool) -> bool {
    !furniture || !matches!(sprite, 67 | 80..=82 | 111 | 112 | 114 | 116 | 117)
}

pub fn object_in_group(sprite: usize, furniture: bool, group: usize) -> bool {
    match group {
        0 => true,
        1 => !furniture || sprite < 40,
        2 => furniture && (40..56).contains(&sprite),
        3 => furniture && (56..72).contains(&sprite),
        4 => furniture && (72..88).contains(&sprite),
        5 => furniture && (88..104).contains(&sprite),
        6 => furniture && (104..120).contains(&sprite),
        _ => false,
    }
}
pub const CITY_FLOORS: [&str; 12] = [
    "Asphalte",
    "Asphalte usé",
    "Trottoir béton",
    "Pavés gris",
    "Pavés bruns",
    "Briques de sol",
    "Carrelage ivoire",
    "Ardoise",
    "Terrazzo",
    "Parquet",
    "Béton d'atelier",
    "Pierre ocre",
];
pub const CITY_OBJECTS: [&str; 48] = [
    "Comptoir et caisse",
    "Vitrine d'épicerie",
    "Étal de produits",
    "Rayonnage de conserves",
    "Bureau et dossiers",
    "Fauteuil d'accueil",
    "Canapé",
    "Bibliothèque",
    "Cuisinière",
    "Évier",
    "Réfrigérateur",
    "Buffet et vaisselle",
    "Table de soins",
    "Armoire médicale",
    "Coffre-fort",
    "Machine d'atelier",
    "Réverbère",
    "Banc public",
    "Poubelle",
    "Jardinière de rue",
    "Pompe à eau",
    "Armoire électrique",
    "Distributeur",
    "Potelet",
    "Étal de marché",
    "Charrette de marchandises",
    "Cageots de produits",
    "Kiosque",
    "Palette de caisses",
    "Tonneaux",
    "Sacs de grains",
    "Matériaux de chantier",
    "Feu de circulation",
    "Climatisation",
    "Borne de recharge",
    "Billetterie",
    "Parcmètre",
    "Distributeur bancaire",
    "Consignes à colis",
    "Panneau de transport",
    "Borne holographique",
    "Robot de nettoyage",
    "Interphone d'urgence",
    "Portique de sécurité",
    "Poste de contrôle",
    "Fabricateur",
    "Station de diagnostic",
    "Batterie de stockage",
];

pub fn is_floor(index: usize) -> bool {
    index < 10 || (16..MATERIAL_SLOTS).contains(&index)
}

pub fn review_requested() -> bool {
    std::env::args().any(|arg| arg == "--urban-review" || arg == "--capture-urban")
}
