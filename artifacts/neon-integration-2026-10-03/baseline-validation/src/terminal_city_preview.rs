//! Review-only native rendering proposal. No gameplay renderer selects it.
use super::*;

const ROOM: [&str; 14] = [
    "####################",
    "#..................#",
    "#..ss..cc....vv....#",
    "#..ss..cc..........#",
    "#..ss.....hh..hh...#",
    "#.........hh..hh...#",
    "#..................#",
    "#..t.....@....i....+",
    "#..................#",
    "#..b....T.....p....#",
    "#.............p....#",
    "#..u....l..........#",
    "#..................#",
    "####################",
];

fn kind(symbol: char) -> Decor {
    match symbol {
        '#' => Decor::Wall,
        's' => Decor::Server,
        'c' => Decor::ShopCounter,
        'v' => Decor::VendingMachine,
        'h' => Decor::ShopShelf,
        't' => Decor::ControlReady,
        'b' => Decor::Bench,
        'T' => Decor::CafeTable,
        'p' => Decor::Planter,
        'u' => Decor::Bin,
        'l' => Decor::LampPost,
        '+' => Decor::DoorOpen,
        _ => Decor::Deck,
    }
}

pub(super) fn draw() {
    clear_background(Color::from_rgba(7, 11, 14, 255));
    draw_ui_text_bold(
        "DIRECTION VISUELLE · PROPOSITION À VALIDER",
        32.0,
        42.0,
        25.0,
        WHITE,
    );
    draw_ui_text(
        "Même composition et même grille · aperçu natif isolé du jeu",
        32.0,
        73.0,
        17.0,
        LIGHTGRAY,
    );
    let width = (screen_width() - 96.0) / 2.0;
    let cell = (width / 20.0).floor().min(28.0);
    for proposed in [false, true] {
        let ox = 32.0 + if proposed { width + 32.0 } else { 0.0 };
        draw_ui_text_bold(
            if proposed {
                "PISTE · TERMINAL INDUSTRIEL"
            } else {
                "RENDU ACTUEL"
            },
            ox,
            114.0,
            20.0,
            if proposed { SKYBLUE } else { LIGHTGRAY },
        );
        for (y, line) in ROOM.iter().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let p = GridPos::new(x as i32, y as i32);
                let tile_kind = kind(symbol);
                let joins = p.cardinal_neighbors().map(|n| {
                    ROOM.get(n.y as usize)
                        .and_then(|line| line.chars().nth(n.x as usize))
                        .is_some_and(|s| kind(s) == tile_kind)
                });
                let rect = Rect::new(ox + x as f32 * cell, 136.0 + y as f32 * cell, cell, cell);
                if proposed {
                    draw_proposed_tile(rect, tile_kind, joins);
                } else {
                    draw_tile(rect, tile_kind, joins, true, p);
                }
                if matches!(symbol, '@' | 'i') {
                    draw_entity(
                        rect,
                        symbol,
                        TerminalGlyphPalette::monochrome(if symbol == '@' {
                            Color::from_rgba(93, 241, 199, 255)
                        } else {
                            Color::from_rgba(110, 196, 210, 255)
                        }),
                        false,
                        false,
                        None,
                    );
                }
            }
        }
        let y = 136.0 + cell * 14.0;
        for (i, label) in if proposed {
            [
                "Sol discret · silhouettes mieux détachées",
                "Mobilier gris · console active en cyan",
                "Volumes reliés sur plusieurs cases",
            ]
        } else {
            [
                "Sol rayé sur chaque case",
                "Couleurs semblables pour décor et service",
                "Objets répétés case par case",
            ]
        }
        .into_iter()
        .enumerate()
        {
            draw_ui_text(label, ox, y + 35.0 + i as f32 * 27.0, 17.0, LIGHTGRAY);
        }
    }
    let y = (136.0 + cell * 14.0 + 145.0).min(screen_height() - 150.0);
    draw_ui_text_bold("AUTRES PISTES", 32.0, y, 20.0, SKYBLUE);
    draw_ui_text(
        "Quartiers composés : étals, ateliers, réserves et logements reconnaissables.",
        32.0,
        y + 31.0,
        18.0,
        LIGHTGRAY,
    );
    draw_ui_text(
        "Inspection : nom précis, obstacle ou passage, action réellement disponible.",
        32.0,
        y + 60.0,
        18.0,
        LIGHTGRAY,
    );
    draw_ui_text(
        "Animation : voyants des installations actives, discrètement et sans brouiller la carte.",
        32.0,
        y + 89.0,
        18.0,
        LIGHTGRAY,
    );
    draw_ui_text(
        "La piste de droite n'est pas encore appliquée aux parties normales.",
        32.0,
        screen_height() - 21.0,
        16.0,
        Color::from_rgba(224, 185, 111, 255),
    );
}

fn draw_proposed_tile(rect: Rect, tile_kind: Decor, joins: [bool; 4]) {
    let floor = Color::from_rgba(13, 18, 20, 255);
    let edge = Color::from_rgba(110, 118, 120, 255);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, floor);
    if matches!(
        tile_kind,
        Decor::Wall | Decor::Server | Decor::ShopCounter | Decor::ShopShelf | Decor::VendingMachine
    ) {
        let inset = if tile_kind == Decor::Wall { 2.0 } else { 3.0 };
        let [north, east, south, west] = joins;
        let left = rect.x + if west { 0.0 } else { inset };
        let right = rect.x + rect.w - if east { 0.0 } else { inset };
        let top = rect.y + if north { 0.0 } else { inset };
        let bottom = rect.y + rect.h - if south { 0.0 } else { inset };
        draw_rectangle(
            left,
            top,
            right - left,
            bottom - top,
            Color::from_rgba(27, 33, 36, 255),
        );
        if !north {
            draw_line(left, top, right, top, 1.0, edge);
        }
        if !east {
            draw_line(right, top, right, bottom, 1.0, edge);
        }
        if !south {
            draw_line(left, bottom, right, bottom, 1.0, edge);
        }
        if !west {
            draw_line(left, top, left, bottom, 1.0, edge);
        }
        if tile_kind != Decor::Wall {
            let inner = Color::from_rgba(68, 78, 82, 255);
            for step in [0.32, 0.65] {
                draw_line(
                    rect.x + 6.0,
                    rect.y + rect.h * step,
                    rect.x + rect.w - 6.0,
                    rect.y + rect.h * step,
                    1.0,
                    inner,
                );
            }
            if tile_kind == Decor::Server {
                draw_rectangle(
                    rect.x + 8.0,
                    rect.y + rect.h * 0.82,
                    2.0,
                    2.0,
                    Color::from_rgba(108, 120, 114, 255),
                );
            }
        }
        return;
    }
    let pattern = match tile_kind {
        Decor::Bench => [
            "........", "........", ".######.", ".#....#.", ".######.", "..#..#..", "........",
            "........",
        ],
        Decor::CafeTable => [
            "........", "...##...", "..####..", ".##..##.", ".##..##.", "..####..", "...##...",
            "........",
        ],
        Decor::Planter => [
            "........", ".######.", ".#..#.#.", ".#.##.#.", ".##...#.", ".#.#..#.", ".######.",
            "........",
        ],
        Decor::Bin => [
            "........", "..####..", ".#....#.", ".#....#.", ".#....#.", ".#....#.", "..####..",
            "........",
        ],
        Decor::LampPost => [
            "........", "...##...", "..#..#..", ".#....#.", ".#....#.", "..#..#..", "...##...",
            "........",
        ],
        Decor::ControlReady => CONTROL_READY,
        Decor::DoorOpen => OPEN_DOOR,
        _ => ["........"; 8],
    };
    if tile_kind == Decor::Deck {
        draw_rectangle(
            rect.x + rect.w * 0.5,
            rect.y + rect.h * 0.5,
            1.0,
            1.0,
            Color::from_rgba(38, 48, 50, 255),
        );
    } else {
        let color = if tile_kind == Decor::ControlReady {
            Color::from_rgba(92, 206, 191, 255)
        } else if tile_kind == Decor::Planter {
            Color::from_rgba(98, 116, 91, 255)
        } else {
            edge
        };
        draw_pixel_glyph(rect, &pattern, color);
    }
}
