//! Stable indices for the proposed urban lot. Palette exposure is opt-in.
pub const MATERIAL_SLOTS: usize = 28;
pub const FURNITURE_COUNT: usize = 56;
pub const WALL_STYLE_COUNT: usize = 3;
pub const WALL_STYLES: [&str; 3] = ["Métal", "Brique", "Crépi"];
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
