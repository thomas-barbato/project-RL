from pathlib import Path

root=Path(__file__).resolve().parents[2]
def edit(name,old,new):
    p=root/name
    s=p.read_text(encoding='utf-8')
    assert old in s, (name,old[:80])
    p.write_text(s.replace(old,new),encoding='utf-8')

edit('examples/surface_preview/editor.rs', 'text.to_lowercase()\n        .chars()', 'text.to_lowercase().replace("œ", "oe").replace("æ", "ae")\n        .chars()')
p=root/'examples/textured_surface_preview.rs'
s=p.read_text(encoding='utf-8'); a=s.index('fn sprite('); b=s.index('fn origin(',a)
p.write_text(s[:a]+s[b:],encoding='utf-8')

edit('examples/surface_preview/workbench.rs', 'p.h - 214.0', 'p.h - 246.0')
edit('examples/surface_preview/workbench.rs', '("Décalage Y (pixels)", details.offset_y),', '("Décalage Y (pixels)", details.offset_y),\n        ("Taille du dessin (%)", details.scale as i16),')
edit('examples/surface_preview/workbench.rs', 'p.y + 338.0', 'p.y + 372.0')
edit('examples/surface_preview/workbench.rs', 'p.y + 364.0', 'p.y + 398.0')
edit('examples/surface_preview/workbench.rs', 'pub fn draw_world(app: &App, assets: &super::assets::Assets, font: &Font)', 'pub fn draw_world(app: &App, assets: &super::assets::Assets, background: &[Texture2D], font: &Font)')
edit('examples/surface_preview/workbench.rs', '''        let accessible = if app.workbench.overlay == 3 {
            Some(author::reachable(&app.scene.document))
        } else {
            None
        };''', '''        let accessible = &app.workbench.accessible;''')
edit('examples/surface_preview/workbench.rs', 'accessible.as_ref().is_some_and(|set| !set.contains(&pos))', '!accessible.contains(&pos)')
p=root/'examples/surface_preview/workbench.rs'; s=p.read_text(encoding='utf-8')
a=s.index('    for p in &app.workbench.preview {',s.index('pub fn draw_world'))
b=s.index('    if app.editor.layers.visible[4]',a)
s=s[:a]+'''    let preview_cells: Vec<_> = if app.workbench.pasting {
        app.workbench.clipboard.as_ref().zip(app.hover()).map(|(stamp,pos)|
            Region{x:pos.x,y:pos.y,w:stamp.width,h:stamp.height}.cells().collect()).unwrap_or_default()
    } else { app.workbench.preview.clone() };
    if let Some(proposal)=&app.workbench.preview_scene {
        assets.paint.borrow_mut().sync(&proposal.document.paint);
        for pos in &preview_cells {
            if pos.x<0 || pos.y<0 || pos.x>=proposal.document.width || pos.y>=proposal.document.height { continue; }
            if app.editor.layers.visible[0] {
                super::rendering::draw_floor(proposal,assets,background,*pos,point(*pos),cell,app.editor.layers.visible[1]);
            }
        }
        for part in &proposal.document.structures {
            if app.editor.layers.visible[1] && preview_cells.iter().any(|p| (p.x-part.pos.x).abs()+(p.y-part.pos.y).abs()<=1) {
                super::rendering::draw_structure_tinted(proposal,assets,part,point(part.pos),cell,Color::new(1.,1.,1.,0.75));
            }
        }
        for prop in &proposal.document.props {
            if app.editor.layers.visible[author::prop_layer(prop)] && preview_cells.iter().any(|p| prop.contains(*p)) {
                super::rendering::draw_prop_tinted(assets,prop,point(prop.pos),cell,Color::new(1.,1.,1.,0.75));
            }
        }
        assets.paint.borrow_mut().sync(&app.scene.document.paint);
    }
    for p in &preview_cells {
        let at = point(*p);
        let color=if app.workbench.preview_valid { Color::new(0.2,0.8,0.65,0.22) } else { Color::new(0.95,0.2,0.2,0.35) };
        draw_rectangle(at.x,at.y,cell,cell,color);
        draw_rectangle_lines(at.x,at.y,cell,cell,1.,if app.workbench.preview_valid { GREEN } else { RED });
    }
''' + s[b:]
a=s.index('    if app.workbench.pasting {',s.index('    if app.workbench.overlay != 0',s.index('pub fn draw_world')))
b=s.index('\n}\n\nfn catalogue_grid',a)
s=s[:a]+s[b:]
needle='    btn(font, ctl(p, p.w - 116.0, p.h - 42.0, 92.0), "Fermer", false);'
assert needle in s
s=s.replace(needle,'''    let status: String=app.message.chars().take((p.w/8.).floor() as usize).collect();
    text(font,&status,p.x+24.,p.y+p.h-54.,14);
'''+needle)
p.write_text(s,encoding='utf-8')

edit('examples/map_editor.rs', 'workbench::draw_world(app, assets, font);', 'workbench::draw_world(app, assets, background, font);')
edit('examples/map_editor.rs', 'rendering::draw_ground(&app.scene, assets, background, GridPos::new(x, y), p, cell);', 'rendering::draw_floor(&app.scene, assets, background, GridPos::new(x, y), p, cell, app.testing || app.editor.layers.visible[1]);')
edit('examples/map_editor.rs', 'app.hover().filter(|_| app.pointer_preview)', 'app.hover().filter(|_| app.pointer_preview && app.workbench.panel.is_none() && app.workbench.tool == author::Tool::Pencil && !app.workbench.pasting)')
edit('examples/map_editor.rs', '''                    if let Some(texture) = app.editor.texture(app.editor.brush, assets) {''', '''                    if let Brush::Object(sprite,furniture) = app.editor.brush {
                        let prop=scene::Prop{pos,sprite,furniture,rotation:app.editor.rotation,blocking:app.editor.blocking,details:app.editor.details.clone()};
                        let valid=prop.cells().all(|q| q.x>=0 && q.y>=0 && q.x<doc.width && q.y<doc.height
                            && (!app.scene.is_structure(q) || prop.details.decoration)
                            && !doc.props.iter().any(|old| old.contains(q) && !old.details.decoration && !prop.details.decoration && !old.contains(pos)));
                        rendering::draw_prop_tinted(assets,&prop,p,cell,Color::new(1.,1.,1.,0.55));
                        let (w,h)=prop.size();
                        draw_rectangle_lines(p.x,p.y,w as f32*cell,h as f32*cell,2.,if valid { GREEN } else { RED });
                    } else if let Some(texture) = app.editor.texture(app.editor.brush, assets) {''')
edit('examples/map_editor.rs', '    if let Some(index) = args.iter().position(|arg| arg == "--capture-sf") {', '    app.saved = Some(app.scene.document.clone());\n    if let Some(index) = args.iter().position(|arg| arg == "--capture-sf") {')
