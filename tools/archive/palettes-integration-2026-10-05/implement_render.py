from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def edit(path,pairs):
 p=ROOT/path;s=p.read_text(encoding='utf-8')
 for a,b in pairs:
  assert a in s,(path,a[:120]);s=s.replace(a,b,1)
 p.write_text(s,encoding='utf-8')

edit('src/game_art.rs',[
 ('use project_rl::world::GridPos;', 'use project_rl::world::GridPos;\nuse project_rl::entity::EquipmentPalette;\n#[path = "game_art_palette.rs"]\nmod palettes;'),
 ('        body: u8,','        body: u8,\n        palettes: [EquipmentPalette; 3],'),
 ('    Item(ItemArt),','    Item(ItemArt),\n    ColoredItem(ItemArt, EquipmentPalette),'),
 ('        body,\n        weapon,','        body,\n        palettes: [EquipmentPalette::Original; 3],\n        weapon,'),
 ('            body,\n            weapon,','            body,\n            palettes,\n            weapon,'),
 ('player_layers::draw_player(rect, body, equipment, weapon, hands, 32, opacity);','player_layers::draw_player(rect, body, equipment, weapon, hands, palettes, 32, opacity);'),
 ('        Visual::Item(item) => draw_cell(&art.ground, rect, u16::from(item.icon), 32, opacity),','        Visual::Item(item) => palettes::draw_item(rect, item, EquipmentPalette::Original, 32, opacity),\n        Visual::ColoredItem(item, palette) => palettes::draw_item(rect,item,palette,32,opacity),'),
 ('            body,\n            weapon,','            body,\n            palettes,\n            weapon,'),
 ('player_layers::draw_player(rect, body, equipment, weapon, hands, 128, 1.0);','player_layers::draw_player(rect, body, equipment, weapon, hands, palettes, 128, 1.0);'),
 ('        Visual::Item(item) => draw_cell(&art.items, rect, item.illustration, 128, 1.0),','        Visual::Item(item) => palettes::draw_item(rect, item, EquipmentPalette::Original, 128, 1.0),\n        Visual::ColoredItem(item, palette) => palettes::draw_item(rect,item,palette,128,1.0),'),
 ('pub(crate) fn draw_item(rect: Rect, id: &str) -> bool {','pub(crate) fn draw_item_variant(rect: Rect, id: &str, palette: EquipmentPalette) -> bool {\n    let Some(art) = item(id) else { return false; };\n    draw_portrait(rect, Visual::ColoredItem(art,palette));\n    true\n}\n\npub(crate) fn draw_item(rect: Rect, id: &str) -> bool {'),
])
edit('src/game_art_player.rs',[
 ('    hands: Option<[Option<WeaponHold>; 2]>,','    hands: Option<[Option<WeaponHold>; 2]>,\n    palettes: [EquipmentPalette; 3],'),
 ('worn::draw(rect, body, equipment, weapon, hands, side, opacity);','worn::draw(rect, body, equipment, weapon, hands, palettes, side, opacity);'),
])
edit('src/game_art_worn.rs',[
 ('    portrait: Texture2D,','    portrait: Texture2D,\n    world_source: Image,\n    portrait_source: Image,'),
 ('        portrait: texture(portrait, FilterMode::Nearest),','        portrait: texture(portrait, FilterMode::Nearest),\n        world_source: palettes::decoded(world),\n        portrait_source: palettes::decoded(portrait),'),
 ('    hands: Option<[Option<WeaponHold>; 2]>,\n    side: u16,','    hands: Option<[Option<WeaponHold>; 2]>,\n    palettes: [EquipmentPalette; 3],\n    side: u16,'),
 ('    draw_pose(rect, body, equipment, pose, side, opacity, flip);','    draw_pose_variant(rect, body, equipment, pose, side, opacity, flip, palettes);'),
 ('fn draw_pose(\n','fn draw_pose_variant(\n'),
 ('    flip: bool,\n) {','    flip: bool,\n    mut colors: [EquipmentPalette;3],\n) {'),
 ('        draw_texture_ex(\n            sprite,','''        if flip { colors.swap(1,2); }
        if colors != [EquipmentPalette::Original;3] {
            COLORED.with(|cache| {
                let mut cache=cache.borrow_mut();
                let key=(pose,index,side,colors);
                if !cache.contains_key(&key) {
                    let source=if side==128 { &art[pose].portrait_source } else { &art[pose].world_source };
                    let image=palettes::figure(source,index,pose,colors);
                    let texture=Texture2D::from_image(&image);
                    texture.set_filter(FilterMode::Nearest);
                    if cache.len()>=192 { cache.clear(); }
                    cache.insert(key,texture);
                }
                draw_texture_ex(&cache[&key],destination.x,destination.y,Color::new(1.0,1.0,1.0,opacity),DrawTextureParams {
                    dest_size:Some(destination.size()),flip_x:flip,..Default::default()
                });
            });
            return;
        }
        draw_texture_ex(
            sprite,'''),
])
with (ROOT/'src/game_art_worn.rs').open('a',encoding='utf-8') as f:
 f.write('''
type FigureColorKey = (usize,u16,u16,[EquipmentPalette;3]);
thread_local! { static COLORED: std::cell::RefCell<std::collections::BTreeMap<FigureColorKey,Texture2D>> = std::cell::RefCell::new(std::collections::BTreeMap::new()); }
fn draw_pose(rect: Rect,body:u8,equipment:[Option<u16>;4],pose:usize,side:u16,opacity:f32,flip:bool) {
    draw_pose_variant(rect,body,equipment,pose,side,opacity,flip,[EquipmentPalette::Original;3]);
}
''')

for path in ['src/inventory_terminal.rs','src/ascii_app.rs']:
 p=ROOT/path;s=p.read_text(encoding='utf-8')
 s=s.replace('crate::game_art::draw_item(image, entry.item().as_str())','crate::game_art::draw_item_variant(image, entry.item().as_str(), entry.palette())')
 s=s.replace('crate::game_art::draw_item(glyph, entry.item().as_str())','crate::game_art::draw_item_variant(glyph, entry.item().as_str(), entry.palette())')
 p.write_text(s,encoding='utf-8')
edit('src/ascii_app.rs',[
 ('ground_art.entry(stack.position()).or_insert(art);','ground_art.entry(stack.position()).or_insert(crate::game_art::Visual::ColoredItem(art,stack.palette()));'),
 ('                                    .copied()\n                                    .map(crate::game_art::Visual::Item),','                                    .copied(),'),
 ('        visual\n    }\n\n    fn front_actor_visual','''        if let crate::game_art::Visual::Player { palettes, .. } = &mut visual {
            let inventory=self.game.player_inventory();
            let equipment=self.game.player_equipment();
            let entry_for_slot=|slot: &project_rl::content::ContentId| equipment.get(slot).and_then(|instance| inventory.get(instance));
            palettes[0]=entry_for_slot(&"core:body_armor".parse().unwrap()).map(|entry|entry.palette()).unwrap_or_default();
            let weapon_color=|slot: usize| self.game.rules().player_weapon_slots.get(slot).and_then(entry_for_slot).map(|entry|entry.palette()).unwrap_or_default();
            if self.game.rules().uses_weapon_hands() && self.active_weapon_slot < 2 {
                palettes[1]=weapon_color(1); palettes[2]=weapon_color(0);
            } else {
                palettes[1]=weapon_color(self.active_weapon_slot as usize); palettes[2]=palettes[1];
            }
        }
        visual
    }

    fn front_actor_visual'''),
])
edit('src/terminal_view.rs',[
 ('            .and_then(|stack| crate::game_art::item(stack.item().as_str()))\n            .map(crate::game_art::Visual::Item),','            .and_then(|stack| crate::game_art::item(stack.item().as_str()).map(|art|crate::game_art::Visual::ColoredItem(art,stack.palette()))),'),
])
edit('src/combat_visuals_capture.rs',[
 ('                    body: (index % 3) as u8,','                    body: (index % 3) as u8,\n                    palettes: Default::default(),'),
 ('                    body: (index % 3) as u8,\n                    weapon: crate','                    body: (index % 3) as u8,\n                    palettes: Default::default(),\n                    weapon: crate'),
 ('                    body: (index % 3) as u8,\n                    weapon,\n                    hands,','                    body: (index % 3) as u8,\n                    palettes: Default::default(),\n                    weapon,\n                    hands,'),
])
