//! Composition operations shared by gestures, prefabs and validation.
use super::scene::{Document, Marker, MarkerKind, Prop, PropDetails, Structure};
use project_rl::world::{DoorState, GridPos, Terrain};
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brush {
    Floor(Option<usize>),
    Wall,
    Corner(u8),
    Door(bool),
    Object(usize, bool),
    Spawn,
}

pub const LAYER_NAMES: [&str; 5] = ["Sols", "Murs", "Mobilier", "Décorations", "Marqueurs"];
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Layers {
    pub visible: [bool; 5],
    pub locked: [bool; 5],
}
impl Default for Layers {
    fn default() -> Self {
        Self {
            visible: [true; 5],
            locked: [false; 5],
        }
    }
}
impl Layers {
    pub fn editable(&self, layer: usize) -> bool {
        self.visible[layer] && !self.locked[layer]
    }
}
pub fn prop_layer(prop: &Prop) -> usize {
    if prop.details.decoration { 3 } else { 2 }
}
pub fn brush_layer(brush: Brush, details: &PropDetails) -> usize {
    match brush {
        Brush::Floor(_) => 0,
        Brush::Object(..) => {
            if details.decoration {
                3
            } else {
                2
            }
        }
        Brush::Spawn => 4,
        _ => 1,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Pencil,
    Line,
    Rectangle,
    Fill,
    Select,
    Move,
    Enemies,
    Cave,
    Exit,
}
pub const TOOLS: [(Tool, &str); 9] = [
    (Tool::Pencil, "Pinceau"),
    (Tool::Line, "Ligne"),
    (Tool::Rectangle, "Rectangle"),
    (Tool::Fill, "Remplir"),
    (Tool::Select, "Sélection"),
    (Tool::Move, "Déplacer"),
    (Tool::Enemies, "Ennemis"),
    (Tool::Cave, "Cave"),
    (Tool::Exit, "Sortie"),
];
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Region {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Region {
    pub fn between(a: GridPos, b: GridPos) -> Self {
        Self {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
            w: (a.x - b.x).abs() + 1,
            h: (a.y - b.y).abs() + 1,
        }
    }
    pub fn contains(self, p: GridPos) -> bool {
        (self.x..self.x + self.w).contains(&p.x) && (self.y..self.y + self.h).contains(&p.y)
    }
    pub fn cells(self) -> impl Iterator<Item = GridPos> {
        (self.y..self.y + self.h)
            .flat_map(move |y| (self.x..self.x + self.w).map(move |x| GridPos::new(x, y)))
    }
}
pub fn line(a: GridPos, b: GridPos) -> Vec<GridPos> {
    let (mut x, mut y) = (a.x, a.y);
    let dx = (b.x - a.x).abs();
    let dy = -(b.y - a.y).abs();
    let sx = if a.x < b.x { 1 } else { -1 };
    let sy = if a.y < b.y { 1 } else { -1 };
    let mut error = dx + dy;
    let mut out = vec![];
    loop {
        out.push(GridPos::new(x, y));
        if (x, y) == (b.x, b.y) {
            break;
        }
        let next = 2 * error;
        if next >= dy {
            error += dy;
            x += sx
        }
        if next <= dx {
            error += dx;
            y += sy
        }
    }
    out
}
pub fn shape(tool: Tool, a: GridPos, b: GridPos, brush: Brush) -> Vec<GridPos> {
    if tool == Tool::Line {
        return line(a, b);
    }
    let r = Region::between(a, b);
    r.cells()
        .filter(|p| {
            !matches!(brush, Brush::Wall | Brush::Corner(_) | Brush::Door(_))
                || p.x == r.x
                || p.x == r.x + r.w - 1
                || p.y == r.y
                || p.y == r.y + r.h - 1
        })
        .collect()
}
pub fn fill_cells(doc: &Document, start: GridPos) -> Vec<GridPos> {
    let target = doc.floors[(start.y * doc.width + start.x) as usize];
    let walls: BTreeSet<_> = doc.structures.iter().map(|p| p.pos).collect();
    let mut queue = VecDeque::from([start]);
    let mut seen = BTreeSet::new();
    let mut result = vec![];
    while let Some(p) = queue.pop_front() {
        if p.x < 0
            || p.y < 0
            || p.x >= doc.width
            || p.y >= doc.height
            || !seen.insert(p)
            || walls.contains(&p)
            || doc.floors[(p.y * doc.width + p.x) as usize] != target
        {
            continue;
        }
        result.push(p);
        queue.extend([
            GridPos::new(p.x - 1, p.y),
            GridPos::new(p.x + 1, p.y),
            GridPos::new(p.x, p.y - 1),
            GridPos::new(p.x, p.y + 1),
        ]);
    }
    result
}
pub struct Placement<'a> {
    pub brush: Brush,
    pub rotation: usize,
    pub style: usize,
    pub fixed: Option<u8>,
    pub blocking: bool,
    pub details: &'a PropDetails,
    pub layers: &'a Layers,
}
pub fn place(
    doc: &mut Document,
    points: &[GridPos],
    placement: &Placement<'_>,
) -> Result<(), String> {
    let layer = brush_layer(placement.brush, placement.details);
    if !placement.layers.editable(layer) {
        return Err("Ce calque est masqué ou verrouillé.".into());
    }
    if matches!(placement.brush, Brush::Floor(_)) && doc.paint.iter().any(|s| s.material.is_some())
    {
        let mut rows: std::collections::BTreeMap<i32, BTreeSet<i32>> = Default::default();
        for p in points {
            rows.entry(p.y).or_default().insert(p.x);
        }
        for (y, xs) in rows {
            let xs: Vec<_> = xs.into_iter().collect();
            let mut i = 0;
            while i < xs.len() {
                let start = xs[i];
                let mut end = start;
                i += 1;
                while i < xs.len() && xs[i] == end + 1 {
                    end = xs[i];
                    i += 1;
                }
                doc.paint.push(super::paint::Stroke {
                    material: None,
                    diameter: 128,
                    points: vec![
                        super::paint::Point::new(start * 64 + 32, y * 64 + 32),
                        super::paint::Point::new(end * 64 + 32, y * 64 + 32),
                    ],
                    clip: Some(super::paint::Clip {
                        left: start * 64,
                        top: y * 64,
                        right: (end + 1) * 64,
                        bottom: (y + 1) * 64,
                        offset_x: 0,
                        offset_y: 0,
                        turns: 0,
                    }),
                });
            }
        }
    }
    for &pos in points {
        if pos.x < 0 || pos.y < 0 || pos.x >= doc.width || pos.y >= doc.height {
            return Err("Le placement dépasse la carte.".into());
        }
        match placement.brush {
            Brush::Floor(index) => {
                doc.floors[(pos.y * doc.width + pos.x) as usize] = index;
                doc.background_tiles
                    .remove(&((pos.y * doc.width + pos.x) as usize));
            }
            Brush::Spawn => doc.spawn = pos,
            Brush::Object(sprite, furniture) => {
                let wanted = Prop {
                    pos,
                    sprite,
                    furniture,
                    rotation: placement.rotation,
                    blocking: placement.blocking,
                    details: placement.details.clone(),
                };
                if doc.props.iter().any(|p| *p == wanted) {
                    continue;
                }
                if !wanted.details.decoration {
                    doc.props.retain(|p| {
                        !p.contains(pos)
                            || p.details.decoration
                            || !placement.layers.editable(prop_layer(p))
                    });
                    if doc.structures.iter().any(|p| p.pos == pos) {
                        return Err("Un mur occupe cet emplacement.".into());
                    }
                }
                doc.props.push(wanted);
            }
            brush => {
                if doc
                    .props
                    .iter()
                    .any(|p| p.contains(pos) && !p.details.decoration)
                {
                    return Err("Un meuble occupe cet emplacement.".into());
                }
                doc.structures.retain(|p| p.pos != pos);
                doc.structures.push(Structure {
                    pos,
                    door: match brush {
                        Brush::Door(open) => Some(if open {
                            DoorState::Open
                        } else {
                            DoorState::Closed
                        }),
                        _ => None,
                    },
                    rotation: if matches!(brush, Brush::Corner(_)) {
                        0
                    } else {
                        placement.rotation
                    },
                    style: if matches!(brush, Brush::Door(_)) {
                        0
                    } else {
                        placement.style
                    },
                    fixed_connections: match brush {
                        Brush::Corner(mask) => Some(mask),
                        Brush::Door(_) => None,
                        _ => placement.fixed,
                    },
                });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Stamp {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub floors: Option<Vec<Option<usize>>>,
    pub backgrounds: std::collections::BTreeMap<usize, GridPos>,
    pub paint: Vec<super::paint::Stroke>,
    pub structures: Vec<Structure>,
    pub props: Vec<Prop>,
    pub markers: Vec<Marker>,
    pub spawn: Option<GridPos>,
}
impl Stamp {
    pub fn take(doc: &Document, r: Region, layers: &Layers) -> Self {
        let shift = |p: GridPos| GridPos::new(p.x - r.x, p.y - r.y);
        let floors = layers.editable(0).then(|| {
            r.cells()
                .map(|p| doc.floors[(p.y * doc.width + p.x) as usize])
                .collect()
        });
        let backgrounds = if floors.is_some() {
            r.cells()
                .filter(|p| doc.floors[(p.y * doc.width + p.x) as usize].is_none())
                .map(|p| {
                    let local = shift(p);
                    (
                        (local.y * r.w + local.x) as usize,
                        doc.background_position(p),
                    )
                })
                .collect()
        } else {
            Default::default()
        };
        Self {
            name: "Ensemble".into(),
            width: r.w,
            height: r.h,
            floors,
            backgrounds,
            paint: if layers.editable(0) {
                doc.paint
                    .iter()
                    .filter_map(|p| p.resized(doc.width, doc.height, r.w, r.h, -r.x, -r.y))
                    .collect()
            } else {
                vec![]
            },
            structures: doc
                .structures
                .iter()
                .filter(|p| layers.editable(1) && r.contains(p.pos))
                .cloned()
                .map(|mut p| {
                    p.pos = shift(p.pos);
                    p
                })
                .collect(),
            props: doc
                .props
                .iter()
                .filter(|p| layers.editable(prop_layer(p)) && p.cells().all(|pos| r.contains(pos)))
                .cloned()
                .map(|mut p| {
                    p.pos = shift(p.pos);
                    p
                })
                .collect(),
            markers: doc
                .markers
                .iter()
                .filter(|p| {
                    layers.editable(4)
                        && r.contains(p.pos)
                        && r.contains(GridPos::new(p.pos.x + p.width - 1, p.pos.y + p.height - 1))
                })
                .cloned()
                .map(|mut p| {
                    p.pos = shift(p.pos);
                    p
                })
                .collect(),
            spawn: (layers.editable(4) && r.contains(doc.spawn)).then(|| shift(doc.spawn)),
        }
    }
    pub fn remove(doc: &mut Document, r: Region, layers: &Layers) {
        if layers.editable(0) {
            for p in r.cells() {
                doc.floors[(p.y * doc.width + p.x) as usize] = Some(6);
                doc.background_tiles
                    .remove(&((p.y * doc.width + p.x) as usize));
            }
            // An eraser is clipped to the rectangle, so retained curved paint is exact.
            doc.paint.push(super::paint::Stroke {
                material: None,
                diameter: super::paint::MAX_DIAMETER,
                points: vec![
                    super::paint::Point::new(r.x * 64, r.y * 64),
                    super::paint::Point::new((r.x + r.w) * 64, (r.y + r.h) * 64),
                ],
                clip: Some(super::paint::Clip {
                    left: r.x * 64,
                    top: r.y * 64,
                    right: (r.x + r.w) * 64,
                    bottom: (r.y + r.h) * 64,
                    offset_x: 0,
                    offset_y: 0,
                    turns: 0,
                }),
            });
            // Parallel wide erasers cover even a large selection, including its corners.
            for y in (r.y * 64..(r.y + r.h) * 64).step_by(128) {
                doc.paint.push(super::paint::Stroke {
                    material: None,
                    diameter: 384,
                    points: vec![
                        super::paint::Point::new(r.x * 64, y),
                        super::paint::Point::new((r.x + r.w) * 64, y),
                    ],
                    clip: Some(super::paint::Clip {
                        left: r.x * 64,
                        top: r.y * 64,
                        right: (r.x + r.w) * 64,
                        bottom: (r.y + r.h) * 64,
                        offset_x: 0,
                        offset_y: 0,
                        turns: 0,
                    }),
                });
            }
        }
        doc.structures
            .retain(|p| !layers.editable(1) || !r.contains(p.pos));
        doc.props
            .retain(|p| !layers.editable(prop_layer(p)) || !p.cells().all(|pos| r.contains(pos)));
        doc.markers.retain(|p| {
            !layers.editable(4)
                || !r.contains(p.pos)
                || !r.contains(GridPos::new(p.pos.x + p.width - 1, p.pos.y + p.height - 1))
        });
    }
    pub fn paste(
        &self,
        doc: &mut Document,
        at: GridPos,
        layers: &Layers,
        include_spawn: bool,
    ) -> Result<(), String> {
        if at.x < 0 || at.y < 0 || at.x + self.width > doc.width || at.y + self.height > doc.height
        {
            return Err("L'ensemble dépasse la carte.".into());
        }
        let shift = |p: GridPos| GridPos::new(p.x + at.x, p.y + at.y);
        if layers.editable(0) {
            if let Some(floors) = &self.floors {
                let ground_only = Layers {
                    visible: [true; 5],
                    locked: [false, true, true, true, true],
                };
                Self::remove(
                    doc,
                    Region {
                        x: at.x,
                        y: at.y,
                        w: self.width,
                        h: self.height,
                    },
                    &ground_only,
                );
                for y in 0..self.height {
                    for x in 0..self.width {
                        let source = (y * self.width + x) as usize;
                        let destination = ((y + at.y) * doc.width + x + at.x) as usize;
                        doc.floors[destination] = floors[source];
                        doc.background_tiles.remove(&destination);
                        if let Some(bg) = self.backgrounds.get(&source) {
                            doc.background_tiles.insert(destination, *bg);
                        }
                    }
                }
                doc.paint.extend(self.paint.iter().filter_map(|p| {
                    p.resized(self.width, self.height, doc.width, doc.height, at.x, at.y)
                }));
            }
        }
        if layers.editable(1) {
            doc.structures
                .extend(self.structures.iter().cloned().map(|mut p| {
                    p.pos = shift(p.pos);
                    p
                }));
        }
        doc.props.extend(
            self.props
                .iter()
                .filter(|p| layers.editable(prop_layer(p)))
                .cloned()
                .map(|mut p| {
                    p.pos = shift(p.pos);
                    p
                }),
        );
        if layers.editable(4) {
            doc.markers
                .extend(self.markers.iter().cloned().map(|mut p| {
                    p.pos = shift(p.pos);
                    p
                }));
            if include_spawn {
                if let Some(p) = self.spawn {
                    doc.spawn = shift(p)
                }
            }
        }
        Ok(())
    }
    pub fn rotate(&mut self) {
        let (w, h) = (self.width, self.height);
        let cell = |p: GridPos| GridPos::new(h - 1 - p.y, p.x);
        if let Some(floors) = &mut self.floors {
            let old = floors.clone();
            for y in 0..h {
                for x in 0..w {
                    let p = cell(GridPos::new(x, y));
                    floors[(p.y * h + p.x) as usize] = old[(y * w + x) as usize];
                }
            }
        }
        self.backgrounds = self
            .backgrounds
            .iter()
            .map(|(i, p)| {
                let next = cell(GridPos::new(*i as i32 % w, *i as i32 / w));
                ((next.y * h + next.x) as usize, *p)
            })
            .collect();
        for part in &mut self.structures {
            part.pos = cell(part.pos);
            part.rotation = (part.rotation + 1) % 4;
        }
        for prop in &mut self.props {
            let (_, ph) = prop.size();
            prop.pos = GridPos::new(h - prop.pos.y - ph, prop.pos.x);
            prop.rotation = (prop.rotation + 1) % 4;
            let (dx, dy) = (prop.details.offset_x, prop.details.offset_y);
            prop.details.offset_x = -dy;
            prop.details.offset_y = dx;
        }
        for marker in &mut self.markers {
            marker.pos = GridPos::new(h - marker.pos.y - marker.height, marker.pos.x);
            std::mem::swap(&mut marker.width, &mut marker.height);
        }
        self.spawn = self.spawn.map(cell);
        for stroke in &mut self.paint {
            for point in &mut stroke.points {
                let (x, y) = (point.x, point.y);
                point.x = h * 64 - y;
                point.y = x;
            }
            if let Some(c) = &mut stroke.clip {
                let (l, t, r, b) = (c.left, c.top, c.right, c.bottom);
                c.left = h * 64 - b;
                c.top = l;
                c.right = h * 64 - t;
                c.bottom = r;
                let (x, y) = (c.offset_x, c.offset_y);
                c.offset_x = h * 64 - y;
                c.offset_y = x;
                c.turns = (c.turns + 1) % 4;
            }
        }
        self.width = h;
        self.height = w;
    }
}

#[derive(Clone, Debug)]
pub struct Issue {
    pub pos: GridPos,
    pub message: String,
}
fn blocked_cells(doc: &Document) -> BTreeSet<GridPos> {
    doc.structures
        .iter()
        .filter(|p| {
            p.door.is_none() || matches!(p.door, Some(DoorState::Locked | DoorState::Unpowered))
        })
        .map(|p| p.pos)
        .chain(
            doc.props
                .iter()
                .filter(|p| p.blocking)
                .flat_map(|p| p.cells()),
        )
        .collect()
}
pub fn reachable(doc: &Document) -> BTreeSet<GridPos> {
    let blocked = blocked_cells(doc);
    let mut queue = VecDeque::from([doc.spawn]);
    let mut seen = BTreeSet::new();
    while let Some(p) = queue.pop_front() {
        if p.x < 0
            || p.y < 0
            || p.x >= doc.width
            || p.y >= doc.height
            || blocked.contains(&p)
            || doc.ground_terrain(p) == Terrain::DeepWater
            || !seen.insert(p)
        {
            continue;
        }
        queue.extend([
            GridPos::new(p.x - 1, p.y),
            GridPos::new(p.x + 1, p.y),
            GridPos::new(p.x, p.y - 1),
            GridPos::new(p.x, p.y + 1),
        ]);
    }
    seen
}
pub fn validate(doc: &Document) -> Vec<Issue> {
    let access = reachable(doc);
    let mut out = vec![];
    let blocked = blocked_cells(doc);
    let mut disconnected: BTreeSet<_> = Region {
        x: 0,
        y: 0,
        w: doc.width,
        h: doc.height,
    }
    .cells()
    .filter(|p| {
        !access.contains(p) && !blocked.contains(p) && doc.ground_terrain(*p) != Terrain::DeepWater
    })
    .collect();
    while let Some(start) = disconnected.first().copied() {
        let mut queue = VecDeque::from([start]);
        let mut count = 0;
        while let Some(p) = queue.pop_front() {
            if !disconnected.remove(&p) {
                continue;
            }
            count += 1;
            queue.extend([
                GridPos::new(p.x - 1, p.y),
                GridPos::new(p.x + 1, p.y),
                GridPos::new(p.x, p.y - 1),
                GridPos::new(p.x, p.y + 1),
            ]);
        }
        if out.len() < 256 {
            out.push(Issue {
                pos: start,
                message: format!("Zone de {count} cases inaccessible depuis le départ."),
            });
        }
    }
    for part in doc.structures.iter().filter(|p| p.door.is_some()) {
        let adjacent = [
            GridPos::new(part.pos.x - 1, part.pos.y),
            GridPos::new(part.pos.x + 1, part.pos.y),
            GridPos::new(part.pos.x, part.pos.y - 1),
            GridPos::new(part.pos.x, part.pos.y + 1),
        ];
        if !adjacent.iter().any(|p| access.contains(p)) {
            out.push(Issue {
                pos: part.pos,
                message: "Porte inaccessible depuis le départ.".into(),
            })
        }
    }
    for marker in &doc.markers {
        let candidates = Region {
            x: marker.pos.x,
            y: marker.pos.y,
            w: marker.width,
            h: marker.height,
        }
        .cells()
        .filter(|p| access.contains(p) && *p != doc.spawn)
        .count();
        if candidates == 0 {
            out.push(Issue {
                pos: marker.pos,
                message: "Marqueur sans case accessible.".into(),
            })
        } else if marker.kind == MarkerKind::Enemies && candidates < (marker.count as usize) {
            out.push(Issue {
                pos: marker.pos,
                message: "Zone trop petite pour le nombre d'ennemis.".into(),
            })
        }
        if marker.kind != MarkerKind::Enemies && marker.target.is_empty() {
            out.push(Issue {
                pos: marker.pos,
                message: "Destination de l'accès à renseigner.".into(),
            })
        }
    }
    out
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RandomPreview {
    pub enemies: Vec<GridPos>,
    pub cave: Option<GridPos>,
    pub exit: Option<GridPos>,
}
pub fn random_preview(doc: &Document, seed: u64) -> RandomPreview {
    let access = reachable(doc);
    let mut state = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut pick = |length: usize| {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        (state.wrapping_mul(2685821657736338717) % length as u64) as usize
    };
    let mut used = BTreeSet::from([doc.spawn]);
    let mut out = RandomPreview {
        enemies: vec![],
        cave: None,
        exit: None,
    };
    for kind in [MarkerKind::Cave, MarkerKind::Exit, MarkerKind::Enemies] {
        let zones: Vec<_> = doc.markers.iter().filter(|m| m.kind == kind).collect();
        let selected: Vec<_> = if kind == MarkerKind::Enemies {
            zones
        } else if zones.is_empty() {
            vec![]
        } else {
            vec![zones[pick(zones.len())]]
        };
        for m in selected {
            let mut candidates: Vec<_> = Region {
                x: m.pos.x,
                y: m.pos.y,
                w: m.width,
                h: m.height,
            }
            .cells()
            .filter(|p| access.contains(p) && !used.contains(p))
            .collect();
            for _ in 0..if kind == MarkerKind::Enemies {
                m.count
            } else {
                1
            } {
                if candidates.is_empty() {
                    break;
                }
                let p = candidates.swap_remove(pick(candidates.len()));
                used.insert(p);
                match kind {
                    MarkerKind::Enemies => out.enemies.push(p),
                    MarkerKind::Cave => out.cave = Some(p),
                    MarkerKind::Exit => out.exit = Some(p),
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::{
        paint::{Point, Stroke},
        scene::Scene,
    };
    use super::*;
    #[test]
    fn new_floor_clears_only_selected_paint_and_validation_detects_a_sealed_room() {
        let mut doc = Scene::empty(10, 10).unwrap().document;
        doc.paint.push(Stroke {
            clip: None,
            material: Some(5),
            diameter: 96,
            points: vec![Point::new(150, 150)],
        });
        let textures: Vec<_> = (0..super::super::catalog::MATERIAL_SLOTS)
            .map(|i| super::super::assets::Raster {
                bytes: [i as u8, 0, 0, 255].repeat(4096),
            })
            .collect();
        let adjacent = super::super::paint::rasterize(GridPos::new(1, 2), &doc.paint, &textures);
        place(
            &mut doc,
            &[GridPos::new(2, 2)],
            &Placement {
                brush: Brush::Floor(Some(4)),
                rotation: 0,
                style: 0,
                fixed: None,
                blocking: false,
                details: &Default::default(),
                layers: &Default::default(),
            },
        )
        .unwrap();
        assert!(
            super::super::paint::rasterize(GridPos::new(2, 2), &doc.paint, &textures)
                .chunks_exact(4)
                .all(|p| p[3] == 0)
        );
        assert_eq!(
            adjacent,
            super::super::paint::rasterize(GridPos::new(1, 2), &doc.paint, &textures)
        );
        place(
            &mut doc,
            &shape(
                Tool::Rectangle,
                GridPos::new(3, 3),
                GridPos::new(7, 7),
                Brush::Wall,
            ),
            &Placement {
                brush: Brush::Wall,
                rotation: 0,
                style: 0,
                fixed: None,
                blocking: true,
                details: &Default::default(),
                layers: &Default::default(),
            },
        )
        .unwrap();
        assert!(
            validate(&doc)
                .iter()
                .any(|i| i.message.contains("9 cases inaccessible"))
        );
    }
    #[test]
    fn room_fill_stops_at_walls_and_shape_does_not_change_the_source() {
        let scene = Scene::empty(12, 10).unwrap();
        let original = scene.document.clone();
        let mut doc = original.clone();
        let points = shape(
            Tool::Rectangle,
            GridPos::new(2, 2),
            GridPos::new(7, 7),
            Brush::Wall,
        );
        place(
            &mut doc,
            &points,
            &Placement {
                brush: Brush::Wall,
                rotation: 0,
                style: 4,
                fixed: None,
                blocking: true,
                details: &Default::default(),
                layers: &Default::default(),
            },
        )
        .unwrap();
        let constructed = Scene::from_document(doc).unwrap();
        assert_eq!(
            fill_cells(&constructed.document, GridPos::new(3, 3)).len(),
            16
        );
        assert_eq!(scene.document, original);
    }
    #[test]
    fn group_rotation_moves_footprints_markers_and_exact_painted_pixels() {
        let mut doc = Scene::empty(5, 4).unwrap().document;
        doc.props.push(Prop {
            pos: GridPos::new(1, 1),
            sprite: 72,
            furniture: true,
            rotation: 0,
            blocking: true,
            details: PropDetails {
                width: 1,
                height: 2,
                opaque: Some(false),
                ..Default::default()
            },
        });
        doc.markers.push(Marker {
            pos: GridPos::new(3, 1),
            width: 1,
            height: 2,
            kind: MarkerKind::Enemies,
            name: "Patrouille".into(),
            target: String::new(),
            count: 1,
        });
        doc.paint.push(Stroke {
            clip: None,
            material: Some(5),
            diameter: 96,
            points: vec![Point::new(10, 20), Point::new(200, 180)],
        });
        let mut stamp = Stamp::take(
            &doc,
            Region {
                x: 0,
                y: 0,
                w: 5,
                h: 4,
            },
            &Default::default(),
        );
        let original = stamp.clone();
        let textures: Vec<_> = (0..super::super::catalog::MATERIAL_SLOTS)
            .map(|i| super::super::assets::Raster {
                bytes: (0..4096)
                    .flat_map(|p| [i as u8 * 5, (p % 64) as u8, (p / 64) as u8, 255])
                    .collect(),
            })
            .collect();
        stamp.rotate();
        assert_eq!((stamp.width, stamp.height), (4, 5));
        assert_eq!(stamp.props[0].pos, GridPos::new(1, 1));
        assert_eq!(stamp.props[0].size(), (2, 1));
        for y in 0..4 {
            for x in 0..5 {
                let before = super::super::assets::Raster {
                    bytes: super::super::paint::rasterize(
                        GridPos::new(x, y),
                        &original.paint,
                        &textures,
                    ),
                };
                let after =
                    super::super::paint::rasterize(GridPos::new(3 - y, x), &stamp.paint, &textures);
                assert_eq!(after, before.rotated(1).bytes, "tile {x},{y}");
            }
        }
        for _ in 0..3 {
            stamp.rotate();
        }
        assert_eq!(stamp, original);
    }
    #[test]
    fn decoration_overlays_and_independent_sight_survive_old_json_and_reload() {
        let mut doc = Scene::empty(6, 6).unwrap().document;
        let mut prop = Prop {
            pos: GridPos::new(2, 2),
            sprite: 6,
            furniture: true,
            rotation: 0,
            blocking: true,
            details: PropDetails {
                opaque: Some(false),
                ..Default::default()
            },
        };
        doc.props.push(prop.clone());
        prop.sprite = 103;
        prop.blocking = false;
        prop.details.decoration = true;
        doc.props.push(prop);
        let scene = Scene::from_document(doc.clone()).unwrap();
        assert!(!scene.game.map().is_walkable(GridPos::new(2, 2)));
        assert!(!scene.game.map().blocks_vision(GridPos::new(2, 2)));
        assert_eq!(
            Scene::from_document(
                serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap()
            )
            .unwrap()
            .document,
            doc
        );
        let mut value = serde_json::to_value(&doc).unwrap();
        value["props"][0].as_object_mut().unwrap().remove("details");
        let old: Document = serde_json::from_value(value).unwrap();
        assert!(
            Scene::from_document(old)
                .unwrap()
                .game
                .map()
                .blocks_vision(GridPos::new(2, 2))
        );
    }
    #[test]
    fn random_placements_are_repeatable_accessible_and_never_overlap() {
        let mut doc = Scene::empty(10, 10).unwrap().document;
        for kind in [MarkerKind::Enemies, MarkerKind::Cave, MarkerKind::Exit] {
            doc.markers.push(Marker {
                pos: GridPos::new(1, 1),
                width: 8,
                height: 8,
                kind,
                name: "Zone".into(),
                target: "autre-carte".into(),
                count: 5,
            });
        }
        let first = random_preview(&doc, 42);
        assert_eq!(first, random_preview(&doc, 42));
        assert_ne!(first, random_preview(&doc, 43));
        let positions: Vec<_> = first
            .enemies
            .iter()
            .copied()
            .chain(first.cave)
            .chain(first.exit)
            .collect();
        assert_eq!(positions.len(), 7);
        assert_eq!(positions.iter().collect::<BTreeSet<_>>().len(), 7);
        assert!(
            positions
                .iter()
                .all(|p| reachable(&doc).contains(p) && *p != doc.spawn)
        );
    }
}
