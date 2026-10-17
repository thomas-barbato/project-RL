//! Isolated, review-only 64px equipment composition. Never used by the game.
//! cargo run --locked --example held_weapon_review
use macroquad::prelude::*;
use serde_json::Value;
use std::path::{Path, PathBuf};

const SIDE: usize = 128;
const HEIGHT: i32 = 944;
const HEAVY_HANDS: [[f32; 2]; 2] = [[55.93273, 63.41818], [73.91273, 63.41818]];
const SHOULDERS: [[f32; 2]; 2] = [[53.45455, 54.19091], [74.80909, 54.19091]];
const POLE_HANDS: [[f32; 2]; 2] = [[63.20909, 62.78545], [72.01455, 55.45636]];
// Existing canonical sleeve masks, measured in the original paintings.
const INDEPENDENT: [&[[f32; 2]]; 2] = [
    &[
        [105., 435.],
        [482., 435.],
        [483., 473.],
        [463., 519.],
        [425., 556.],
        [390., 600.],
        [340., 660.],
        [100., 660.],
    ],
    &[
        [770., 435.],
        [1150., 435.],
        [1160., 660.],
        [910., 660.],
        [865., 600.],
        [825., 556.],
        [792., 519.],
        [770., 473.],
    ],
];
const HEAVY: [&[[f32; 2]]; 2] = [
    &[
        [340., 435.],
        [477., 435.],
        [485., 500.],
        [550., 537.],
        [550., 674.],
        [426., 674.],
        [284., 608.],
        [283., 526.],
    ],
    &[
        [778., 435.],
        [919., 435.],
        [960., 526.],
        [966., 608.],
        [894., 674.],
        [742., 674.],
        [742., 537.],
        [779., 500.],
    ],
];

struct Candidate {
    number: usize,
    source: &'static str,
    label: &'static str,
    note: &'static str,
    grip: [f32; 2],
    support: [f32; 2],
    kind: u8, // 0: unchanged control, 1: heavy firearm, 2: axe, 3: hammer pose study.
}
const CANDIDATES: [Candidate; 6] = [
    Candidate {
        number: 1,
        source: "BD-W032",
        label: "Mitrailleuse lourde",
        note: "Volume plus imposant",
        grip: [0.21, 0.44],
        support: [0.62, 0.45],
        kind: 1,
    },
    Candidate {
        number: 2,
        source: "BD-W024",
        label: "Lance-grenades",
        note: "Volume plus imposant",
        grip: [0.34, 0.80],
        support: [0.82, 0.85],
        kind: 1,
    },
    Candidate {
        number: 3,
        source: "BD-W016",
        label: "Derven F-80",
        note: "Lance-flammes",
        grip: [0.25, 0.56],
        support: [0.58, 0.38],
        kind: 1,
    },
    Candidate {
        number: 4,
        source: "BD-W064",
        label: "Hache de combat",
        note: "Manche horizontal / fer aligné",
        grip: [0.25, 0.86],
        support: [0., 0.],
        kind: 2,
    },
    Candidate {
        number: 5,
        source: "BD-W098",
        label: "Étude : contondante à 2 mains",
        note: "Pose seule / aucun changement de règle",
        grip: [0.14, 0.87],
        support: [0., 0.],
        kind: 3,
    },
    Candidate {
        number: 6,
        source: "BD-W003",
        label: "Fusil ordinaire : témoin",
        note: "Rendu conservé à l’identique",
        grip: [0., 0.],
        support: [0., 0.],
        kind: 0,
    },
];
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn load(path: impl AsRef<Path>) -> Image {
    Image::from_file_with_format(&std::fs::read(root().join(path)).unwrap(), None).unwrap()
}
fn blank() -> Image {
    Image::gen_image_color(128, 128, BLANK)
}
fn crop(source: &Image, x: usize, y: usize, width: usize, height: usize) -> Image {
    let mut dest = Image::gen_image_color(width as u16, height as u16, BLANK);
    for row in 0..height {
        let a = ((y + row) * usize::from(source.width) + x) * 4;
        dest.bytes[row * width * 4..(row + 1) * width * 4]
            .copy_from_slice(&source.bytes[a..a + width * 4]);
    }
    dest
}
fn cell(source: &Image, index: usize) -> Image {
    crop(source, index % 16 * SIDE, index / 16 * SIDE, SIDE, SIDE)
}
fn bounds(source: &Image) -> [usize; 4] {
    let mut b = [usize::from(source.width), usize::from(source.height), 0, 0];
    for y in 0..usize::from(source.height) {
        for x in 0..usize::from(source.width) {
            if source.bytes[(y * usize::from(source.width) + x) * 4 + 3] >= 96 {
                b[0] = b[0].min(x);
                b[1] = b[1].min(y);
                b[2] = b[2].max(x + 1);
                b[3] = b[3].max(y + 1);
            }
        }
    }
    b
}
fn over(dest: &mut Image, source: &Image) {
    for (d, s) in dest
        .bytes
        .chunks_exact_mut(4)
        .zip(source.bytes.chunks_exact(4))
    {
        let a = u32::from(s[3]);
        let old = u32::from(d[3]) * (255 - a);
        let total = a * 255 + old;
        if total == 0 {
            continue;
        }
        for c in 0..3 {
            d[c] = ((u32::from(s[c]) * a * 255 + u32::from(d[c]) * old + total / 2) / total) as u8;
        }
        d[3] = ((total + 127) / 255) as u8;
    }
}
// Premultiplied bilinear sampling keeps translucent edges free of black fringes.
fn transformed(source: &Image, pivot: [f32; 2], target: [f32; 2], scale: f32, angle: f32) -> Image {
    let (sin, cos) = angle.sin_cos();
    affine(
        source,
        pivot,
        target,
        [cos * scale, -sin * scale, sin * scale, cos * scale],
    )
}
fn affine(source: &Image, pivot: [f32; 2], target: [f32; 2], matrix: [f32; 4]) -> Image {
    let mut dest = blank();
    let [a, b, c, d] = matrix;
    let det = a * d - b * c;
    for y in 0..SIDE {
        for x in 0..SIDE {
            let dx = x as f32 + 0.5 - target[0];
            let dy = y as f32 + 0.5 - target[1];
            let sx = pivot[0] + (d * dx - b * dy) / det - 0.5;
            let sy = pivot[1] + (-c * dx + a * dy) / det - 0.5;
            let ix = sx.floor() as i32;
            let iy = sy.floor() as i32;
            let fx = sx - ix as f32;
            let fy = sy - iy as f32;
            let mut sum = [0.; 4];
            for (xx, yy, w) in [
                (ix, iy, (1. - fx) * (1. - fy)),
                (ix + 1, iy, fx * (1. - fy)),
                (ix, iy + 1, (1. - fx) * fy),
                (ix + 1, iy + 1, fx * fy),
            ] {
                if xx < 0
                    || yy < 0
                    || xx >= i32::from(source.width)
                    || yy >= i32::from(source.height)
                {
                    continue;
                }
                let i = (yy as usize * usize::from(source.width) + xx as usize) * 4;
                let a = source.bytes[i + 3] as f32 * w;
                sum[3] += a;
                for c in 0..3 {
                    sum[c] += source.bytes[i + c] as f32 * a;
                }
            }
            let i = (y * SIDE + x) * 4;
            if sum[3] > 0. {
                for c in 0..3 {
                    dest.bytes[i + c] = (sum[c] / sum[3]).round() as u8;
                }
            }
            dest.bytes[i + 3] = sum[3].round() as u8;
        }
    }
    dest
}
fn inside(point: [f32; 2], original: &[[f32; 2]]) -> bool {
    let p = [
        625. + (point[0] / 2. - 32.) * 1100. / 29.,
        1190. + (point[1] / 2. - 47.) * 1100. / 29.,
    ];
    let mut found = false;
    for i in 0..original.len() {
        let a = original[i];
        let b = original[(i + 1) % original.len()];
        if (a[1] > p[1]) != (b[1] > p[1])
            && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            found = !found;
        }
    }
    found
}
fn selected(source: &Image, polygon: &[[f32; 2]], keep_inside: bool) -> Image {
    let mut dest = source.clone();
    for y in 0..SIDE {
        for x in 0..SIDE {
            if inside([x as f32 + 0.5, y as f32 + 0.5], polygon) != keep_inside {
                dest.bytes[(y * SIDE + x) * 4 + 3] = 0;
            }
        }
    }
    dest
}
fn posed_arm(source: &Image, old: [f32; 2], new: [f32; 2], shoulder: [f32; 2]) -> Image {
    let height = old[1] - shoulder[1];
    affine(
        source,
        shoulder,
        shoulder,
        [
            1.,
            (new[0] - old[0]) / height,
            0.,
            (new[1] - shoulder[1]) / height,
        ],
    )
}
struct Bank {
    bodies: Image,
    fingers: Image,
    extra_bodies: Image,
    extra_fingers: Image,
    weapons: Image,
    manifest: Value,
}
impl Bank {
    fn new() -> Self {
        Self {
            bodies: load("assets/sprites/flat-agents/bodies-128.png"),
            fingers: load("assets/sprites/flat-agents/fingers-128.png"),
            extra_bodies: load("assets/sprites/equipment-native64/bodies-128.png"),
            extra_fingers: load("assets/sprites/equipment-native64/fingers-128.png"),
            weapons: load("assets/sprites/flat-agents/weapons-128.png"),
            manifest: serde_json::from_slice(
                &std::fs::read(root().join("assets/sprites/flat-agents/manifest.json")).unwrap(),
            )
            .unwrap(),
        }
    }
    fn body(&self, outfit: usize, pose: usize) -> Image {
        cell(
            if outfit < 12 {
                &self.bodies
            } else {
                &self.extra_bodies
            },
            ((if outfit < 12 { outfit } else { outfit - 12 }) * 3) * 4 + pose,
        )
    }
    fn fingers(&self, outfit: usize, pose: usize, side: usize) -> Image {
        cell(
            if outfit < 12 {
                &self.fingers
            } else {
                &self.extra_fingers
            },
            (((if outfit < 12 { outfit } else { outfit - 12 }) * 3) * 4 + pose) * 2 + side,
        )
    }
    fn binding(&self, id: &str) -> &Value {
        self.manifest["weapons"]
            .as_array()
            .unwrap()
            .iter()
            .find(|w| w["review_id"] == id && w["side"] != 0)
            .unwrap()
    }
    fn current(&self, outfit: usize, id: &str) -> Image {
        let b = self.binding(id);
        let pose = b["pose"].as_u64().unwrap() as usize;
        let mut image = self.body(outfit, pose);
        over(
            &mut image,
            &cell(&self.weapons, b["layer"].as_u64().unwrap() as usize),
        );
        for side in 0..2 {
            if pose != 0 || side == 1 {
                over(&mut image, &self.fingers(outfit, pose, side));
            }
        }
        image
    }
    fn proposed(&self, outfit: usize, c: &Candidate) -> Image {
        if c.kind == 0 {
            return self.current(outfit, c.source);
        }
        let raw = load(format!(
            "assets/source-art/refonte-agent-2026-10-06/comic-catalogue-review-01/sources/{}.png",
            c.source
        ));
        let b = bounds(&raw);
        let raw = crop(&raw, b[0], b[1], b[2] - b[0], b[3] - b[1]);
        let grip = [
            c.grip[0] * f32::from(raw.width),
            c.grip[1] * f32::from(raw.height),
        ];
        let (scale, angle) = if c.kind == 1 {
            (
                self.binding(c.source)["transform"]["scale_native"]
                    .as_f64()
                    .unwrap() as f32
                    * 2.
                    * 1.55,
                0.,
            )
        } else if c.kind == 2 {
            (48. / f32::from(raw.height), std::f32::consts::FRAC_PI_2)
        } else {
            (
                52. / f32::from(raw.width).hypot(f32::from(raw.height)),
                std::f32::consts::FRAC_PI_4,
            )
        };
        let mut target = HEAVY_HANDS[0];
        if c.kind == 1 {
            target[1] = target[1].max(49. + grip[1] * scale);
        }
        let (sin, cos) = angle.sin_cos();
        let support = if c.kind == 1 {
            let v = [
                c.support[0] * f32::from(raw.width) - grip[0],
                c.support[1] * f32::from(raw.height) - grip[1],
            ];
            [
                target[0] + (cos * v[0] - sin * v[1]) * scale,
                target[1] + (sin * v[0] + cos * v[1]) * scale,
            ]
        } else {
            HEAVY_HANDS[1]
        };
        let hands = [target, support];
        let mut image = self.body(outfit, 0);
        for poly in INDEPENDENT {
            image = selected(&image, poly, false);
        }
        for side in 0..2 {
            let arm = selected(&self.body(outfit, 2), HEAVY[side], true);
            over(
                &mut image,
                &posed_arm(&arm, HEAVY_HANDS[side], hands[side], SHOULDERS[side]),
            );
        }
        let weapon = transformed(&raw, grip, target, scale, angle);
        let box_weapon = bounds(&weapon);
        assert!(
            box_weapon[0] > 0 && box_weapon[1] > 0 && box_weapon[2] < 128 && box_weapon[3] < 128,
            "Weapon clipped: {}",
            c.source
        );
        over(&mut image, &weapon);
        for side in 0..2 {
            let hand = if c.kind == 1 {
                transformed(
                    &self.fingers(outfit, 2, side),
                    HEAVY_HANDS[side],
                    hands[side],
                    1.,
                    0.,
                )
            } else {
                transformed(
                    &self.fingers(outfit, 3, side),
                    POLE_HANDS[side],
                    hands[side],
                    1.,
                    0.694,
                )
            };
            over(&mut image, &hand);
        }
        // The proposal never modifies the head; all changes stay below shoulder.
        let reference = self.body(outfit, 0);
        let head_changes: Vec<_> = image.bytes[..48 * SIDE * 4]
            .chunks_exact(4)
            .zip(reference.bytes.chunks_exact(4))
            .enumerate()
            .filter(|(_, (a, b))| a[3] != b[3] || (a[3] > 0 && a != b))
            .map(|(index, _)| (index % SIDE, index / SIDE))
            .collect();
        assert!(
            head_changes.is_empty(),
            "{} outfit {outfit}: head changes {:?}",
            c.source,
            &head_changes[..head_changes.len().min(8)]
        );
        image
    }
}
fn label(font: &Font, text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size as u16,
            color,
            ..Default::default()
        },
    );
}
fn conf() -> Conf {
    Conf {
        window_title: "Project RL — revue prises 64 px".into(),
        window_width: 1280,
        window_height: HEIGHT,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}
fn draw_sprite(tex: &Texture2D, x: f32, y: f32) {
    draw_rectangle(
        x + 32.,
        y + 32.,
        64.,
        64.,
        Color::from_rgba(43, 47, 45, 255),
    );
    draw_rectangle_lines(
        x + 32.,
        y + 32.,
        64.,
        64.,
        1.,
        Color::from_rgba(92, 105, 101, 255),
    );
    draw_texture(tex, x, y, WHITE);
}
#[macroquad::main(conf)]
async fn main() {
    let bank = Bank::new();
    if std::env::args().any(|arg| arg == "--freeze-approved") {
        let mut atlas = Image::gen_image_color(2048, 128, BLANK);
        for (weapon, candidate) in CANDIDATES[..4].iter().enumerate() {
            for (outfit_index, outfit) in [12, 3, 13].into_iter().enumerate() {
                let image = bank.proposed(outfit, candidate);
                let column = weapon * 3 + outfit_index;
                for row in 0..128 {
                    let i = (row * 2048 + column * 128) * 4;
                    atlas.bytes[i..i + 512]
                        .copy_from_slice(&image.bytes[row * 512..(row + 1) * 512]);
                }
            }
        }
        let folder =
            root().join("assets/source-art/refonte-agent-2026-10-06/held-weapon-review-01");
        std::fs::create_dir_all(&folder).unwrap();
        let original = atlas.bytes.clone();
        for y in 0..128 {
            let row = 2048 * 4;
            atlas.bytes[y * row..(y + 1) * row]
                .copy_from_slice(&original[(127 - y) * row..(128 - y) * row]);
        }
        atlas.export_png(folder.join("approved-128.png").to_str().unwrap());
        println!("12 exact approved compositions frozen with the original review compositor.");
        return;
    }
    let font = load_ttf_font_from_bytes(include_bytes!(
        "../assets/fonts/AtkinsonHyperlegible-Regular.ttf"
    ))
    .unwrap();
    let outfits = [(12, "Cerbère"), (3, "Arcaniste"), (13, "Chrysalide")];
    let textures: Vec<_> = CANDIDATES
        .iter()
        .map(|c| {
            outfits
                .iter()
                .map(|&(n, _)| {
                    let before = Texture2D::from_image(&bank.current(n, c.source));
                    before.set_filter(FilterMode::Nearest);
                    let after = Texture2D::from_image(&bank.proposed(n, c));
                    after.set_filter(FilterMode::Nearest);
                    (before, after)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    for _ in 0..3 {
        next_frame().await;
    }
    for frame in 0..2 {
        clear_background(Color::from_rgba(16, 23, 27, 255));
        label(
            &font,
            "PRISES 64 PX — PROPOSITIONS À VALIDER",
            22.,
            32.,
            25.,
            WHITE,
        );
        label(
            &font,
            "À gauche : actuel. À droite : proposition. Case : 64 px ; dessin affiché sans agrandissement.",
            22.,
            57.,
            17.,
            LIGHTGRAY,
        );
        for (column, &(_, name)) in outfits.iter().enumerate() {
            let x = 270. + column as f32 * 330.;
            label(&font, name, x + 64., 85., 19., WHITE);
        }
        for (row, c) in CANDIDATES.iter().enumerate() {
            let y = 99. + row as f32 * 130.;
            label(
                &font,
                &format!("PRISES64-{:02}", c.number),
                20.,
                y + 36.,
                18.,
                Color::from_rgba(239, 183, 119, 255),
            );
            label(&font, c.label, 20., y + 62., 16., WHITE);
            label(&font, c.note, 20., y + 83., 13., LIGHTGRAY);
            for (column, (before, after)) in textures[row].iter().enumerate() {
                let x = 255. + column as f32 * 330.;
                draw_sprite(before, x, y);
                draw_sprite(after, x + 148., y);
            }
        }
        label(
            &font,
            "Étude 05 : pose uniquement ; les armes à une main conservent leurs règles. Rien n’est intégré au jeu.",
            22.,
            927.,
            16.,
            LIGHTGRAY,
        );
        if frame == 1 {
            let mut capture = get_screen_data();
            for p in capture.bytes.chunks_exact_mut(4) {
                p[3] = 255;
            }
            capture.export_png(
                root()
                    .join("artifacts/apercu-prises64.png")
                    .to_str()
                    .unwrap(),
            );
        }
        next_frame().await;
    }
    println!(
        "PRISES64-01: 6 numbered comparisons, 3 outfits; native 64px; active assets unchanged."
    );
}
