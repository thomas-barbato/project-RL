"""Read-only art audit and direct-resolution exports for visual review.

No active atlas, content ID, pose, attachment or accepted source is changed.
The 64px candidates come from existing large compositions, never from 32px.
"""
from pathlib import Path
import hashlib
import html
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
OUT = BASE / 'actor-resolution-review-01'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def bounds(image, threshold=128):
    return image.getchannel('A').point(lambda a: 255 if a >= threshold else 0).getbbox()


def cell(sheet, index, side):
    x, y = index % 16 * side, index // 16 * side
    return sheet.crop((x, y, x + side, y + side))


def write_json(name, value):
    (OUT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2), 'utf-8')


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for folder in ('native64', 'npc32', 'npc64-current'):
        (OUT / folder).mkdir(exist_ok=True)
    protected = list((ROOT / 'assets/sprites/flat-agents').glob('*'))
    protected += list((ROOT / 'src').glob('game_art*.rs'))
    protected += [ROOT / 'src/graphics.rs', ROOT / 'assets/sprites/cyberpunk/actors-32.png',
                  ROOT / 'assets/sprites/cyberpunk/actors-128.png',
                  ROOT / 'assets/sprites/cyberpunk/crawl-bodies-32.png']
    protected = {str(p.relative_to(ROOT)).replace('\\', '/'): digest(p) for p in protected if p.is_file()}

    source = json.loads((BASE / 'armor-flat-review-05/review.json').read_text('utf-8'))
    active = json.loads((ROOT / 'assets/sprites/flat-agents/manifest.json').read_text('utf-8'))
    active_ids = {e['review_id'] for e in active['outfits']}
    outfits = []
    changed = 0
    files = {}
    for entry in source['entries']:
        if entry['kind'] != 'outfit':
            continue
        e = {k: entry.get(k) for k in ('id', 'name', 'content_id', 'review_state', 'origin_directory')}
        e['active_bank'] = entry['id'] in active_ids
        e['views'] = {}
        for key, view in entry['views'].items():
            old32_path = (BASE / 'armor-flat-review-05' / view['native32']).resolve()
            master_path = (BASE / 'armor-flat-review-05' / view['normal']).resolve()
            old32 = Image.open(old32_path).convert('RGBA')
            master = Image.open(master_path).convert('RGBA')
            assert old32.size == (64, 64) and min(master.size) >= 128
            # Canvas covers two tiles, retaining the accepted visual overflow.
            direct = master.resize((128, 128), Image.Resampling.LANCZOS)
            filename = f"{entry['id']}-{key}.png"
            target = OUT / 'native64' / filename
            direct.save(target)
            different = direct.tobytes() != old32.resize((128, 128), Image.Resampling.NEAREST).tobytes()
            assert different, filename
            changed += 1
            e['views'][key] = {'current32': str(old32_path.relative_to(OUT.parent)).replace('\\', '/'),
                               'current64': str((BASE / 'armor-flat-review-05' / view['display64']).resolve().relative_to(OUT.parent)).replace('\\', '/'),
                               'direct64': f'native64/{filename}'}
            files[str(master_path.relative_to(ROOT)).replace('\\', '/')] = digest(master_path)
            if key == '0-00':
                box = bounds(old32)
                box64 = bounds(direct)
                e['body32_bounds_in_two_tile_canvas'] = box
                e['body32_height'] = box[3] - box[1]
                e['body64_height'] = box64[3] - box64[1]
        outfits.append(e)

    actors = Image.open(ROOT / 'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
    portraits = Image.open(ROOT / 'assets/sprites/cyberpunk/actors-128.png').convert('RGBA')
    bodies = Image.open(ROOT / 'assets/sprites/cyberpunk/crawl-bodies-32.png').convert('RGBA')
    bank = json.loads((ROOT / 'assets/sprites/bestiary/manifest.json').read_text('utf-8'))
    npcs = []
    role_names = {23: 'Clinique / soigneur', 24: 'Marchand, résident, quête / porteur', 25: 'Artisan, technicien / ferrailleur'}
    for index in range(20, 26):
        entry = next(e for e in bank['entries'] if e['legacy_actor_index'] == index)
        image = cell(actors, index, 32)
        portrait = cell(portraits, index, 128)
        fixed = cell(bodies, index - 3, 32)
        image.save(OUT / f'npc32/NPC-{index}.png')
        image.resize((64, 64), Image.Resampling.NEAREST).save(OUT / f'npc64-current/NPC-{index}.png')
        box = bounds(image)
        npcs.append({'id': f'NPC-{index}', 'name': entry['name'], 'role': role_names.get(index, entry['name']),
                     'actor_index': index, 'body32_bounds': box, 'body32_height': box[3] - box[1],
                     'portrait_is_32_x4': portrait.tobytes() == image.resize((128, 128), Image.Resampling.NEAREST).tobytes(),
                     'animated_body_matches_static': fixed.tobytes() == image.tobytes(),
                     'authored_native64_available': False,
                     'current32': f'npc32/NPC-{index}.png', 'current64': f'npc64-current/NPC-{index}.png'})
    assert all(n['body32_height'] == 30 for n in npcs)

    audit = {'review_id': 'ACTOR-RES01', 'date': '2026-10-11', 'status': 'awaiting_visual_review',
             'scope': 'Existing artwork export comparison and current NPC size audit; no active integration',
             'icons_work': 'temporarily_on_hold_by_user', 'requested_display_sizes': [32, 64],
             'logical_tile': 32, 'outfit_canvas32': 64, 'outfit_canvas64': 128,
             'native64_method': 'Direct export from existing large accepted/pending compositions; no redraw claimed',
             'poses_and_attachments': 'unchanged', 'outfits': outfits, 'npcs': npcs,
             'configurations': source['configurations'], 'common_rig': source['common_rig'],
             'active_files_before': protected, 'large_source_hashes': files,
             'findings': ['World outfit path produces a 32px body and enlarges it at 64px.',
                          'NPC quest/service humanoids are 30px tall on native 32px tiles.',
                          'Their 128px portraits are exact enlargements of 32px drawings.',
                          'The bestiary 64px atlas contains four larger bosses, not resolution variants of regular NPCs.']}
    write_json('review.json', audit)
    (OUT / 'review-data.js').write_text('window.actorResolution=' + json.dumps(audit, ensure_ascii=False) + ';', 'utf-8')
    make_html()
    make_board(outfits, npcs)
    assert all(digest(ROOT / name) == before for name, before in protected.items())
    write_json('validation.json', {'review_id': 'ACTOR-RES01', 'outfits': len(outfits),
                                  'active_outfits': len(active_ids), 'direct_native64_exports': changed,
                                  'npc_models': len(npcs), 'npc_native32_height': 30,
                                  'source_and_runtime_files_unchanged': len(protected),
                                  'new_art_accepted': False, 'active_integration': False})
    print(json.dumps({'outfits': len(outfits), 'exports64': changed, 'npcs': npcs,
                      'active_outfit_heights': [(e['id'], e['body32_height'], e['body64_height']) for e in outfits if e['active_bank']]}))


def make_board(outfits, npcs):
    visible = [e for e in outfits if e['active_bank']]
    canvas = Image.new('RGB', (1120, 1130), '#101820')
    draw = ImageDraw.Draw(canvas)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 16)
    title = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 24)
    draw.text((20, 12), 'ACTOR-RES01 · Résolutions réelles et PNJ existants', fill='#eef1f3', font=title)
    draw.text((20, 46), 'Chaque tenue : 32 px actuel | 64 px agrandi | export direct 64 px. Vues à taille réelle.', fill='#a9bccc', font=font)
    for i, e in enumerate(visible):
        x, y = 20 + i % 3 * 365, 90 + i // 3 * 180
        draw.text((x, y), e['id'] + ' · ' + e['name'].split(' · ')[0], fill='#e6ebf0', font=font)
        view = e['views']['0-00']
        for j, key in enumerate(('current32', 'current64', 'direct64')):
            path = OUT / view[key] if key == 'direct64' else BASE / view[key]
            sprite = Image.open(path).convert('RGBA')
            stage = Image.new('RGBA', (128, 128), '#25313b')
            stage.alpha_composite(sprite, ((128 - sprite.width) // 2, (128 - sprite.height) // 2))
            # Keep the literal 32px canvas 64 wide; 64px canvases 128 wide.
            if j == 0:
                stage = stage.crop((32, 0, 96, 128))
            xx = x if j == 0 else x + 72 + (j - 1) * 132
            canvas.paste(stage.convert('RGB'), (xx, y + 30))
            draw.text((xx + 4, y + 158), ('32 px', '64 zoom', '64 direct')[j], fill='#a9bccc', font=font)
    y = 845
    draw.text((20, y), 'PNJ actuels · 30 px de haut en 32 px · aucune version 64 px indépendante', fill='#eef1f3', font=title)
    for i, e in enumerate(npcs):
        x = 20 + i * 183
        draw.text((x, y + 42), e['id'], fill='#e6ebf0', font=font)
        small = Image.open(OUT / e['current32']).convert('RGBA')
        large = Image.open(OUT / e['current64']).convert('RGBA')
        canvas.paste(small, (x + 15, y + 80), small)
        canvas.paste(large, (x + 65, y + 64), large)
        draw.text((x, y + 139), e['name'].replace('Humain · ', ''), fill='#a9bccc', font=font)
    canvas.save(OUT / 'comparison-board.png')


def make_html():
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ACTOR-RES01 · Personnages et PNJ</title>
<style>*{box-sizing:border-box}body{margin:0;background:#0d141c;color:#e7edf3;font:15px system-ui}header{padding:22px 28px;border-bottom:1px solid #314153}h1{font-size:25px;margin:0 0 8px}p{color:#b1c2d0;max-width:1050px;line-height:1.5}.controls{display:flex;gap:18px;flex-wrap:wrap;align-items:center;margin-top:20px}label{display:flex;gap:7px;align-items:center}select{padding:8px;background:#1b2937;color:#e7edf3;border:1px solid #415365;border-radius:5px}main{padding:25px;display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,440px),1fr));gap:20px}.card{background:#15202b;border:1px solid #344252;border-radius:7px;padding:16px;min-width:0}.card h2{font-size:17px;margin:0 0 8px}.tag{font-size:12px;color:#92aecc}.views{display:flex;gap:6px;align-items:end;margin-top:18px}.view{text-align:center;flex:1;font-size:12px;color:#b1c2d0;min-width:0}.stage{position:relative;width:128px;height:128px;max-width:100%;margin:0 auto 8px;display:flex;align-items:center;justify-content:center;overflow:visible;background:#25313b}.stage img{flex:none;image-rendering:pixelated}.stage.light{background:#b5bec3}.stage.textured{background:#41413e url('../armor-flat-review-05/floor32.png');background-size:64px}.npc .stage{width:96px;height:96px}.note{font-size:13px;color:#b1c2d0;margin:14px 0 0}.scene{grid-column:1/-1}.scene-strip{display:flex;align-items:end;gap:0;min-height:160px;padding:32px;overflow:auto;background:#41413e url('../armor-flat-review-05/floor32.png');background-size:64px}.scene-cell{width:64px;height:64px;flex:none;position:relative}.scene-cell img{position:absolute;image-rendering:pixelated}.scene-cell img.outfit{width:128px;height:128px;left:-32px;top:-32px}.scene-cell img.npc{width:64px;height:64px;left:0;top:0}</style>
<header><h1>ACTOR-RES01 · Personnages et PNJ</h1><p>Comparaison des dessins actuels avec des exports directs des grandes sources en 64 px. Les poses, les couleurs et les équipements restent les mêmes. Ces exports sont à examiner ; ils ne sont pas encore intégrés au jeu.</p><p>Les six modèles de PNJ humains mesurent tous 30 px de haut dans une case de 32 px. En 64 px, ils sont actuellement agrandis. Leur portrait agrandi ne fournit pas de dessin supplémentaire.</p><div class="controls"><label>Afficher<select id="group"><option value="active">Les 12 tenues de la banque active</option><option value="all">Toutes les 28 tenues du catalogue</option><option value="npc">Les PNJ existants</option></select></label><label>Configuration<select id="config"></select></label><label>Palette<select id="palette"><option value="0">Référence</option><option value="1">Variante 1</option><option value="2">Variante 2</option></select></label><label>Fond<select id="floor"><option value="dark">Sombre</option><option value="light">Clair</option><option value="textured">Sol du jeu</option></select></label></div></header><main id="cards"></main><script src="review-data.js"></script><script>
const d=window.actorResolution, cards=document.querySelector('#cards'), esc=s=>String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('"','&quot;');
const controls=['group','config','palette','floor'].map(id=>document.querySelector('#'+id));
controls[1].innerHTML=d.configurations.map(c=>'<option value="'+c.number+'">'+esc(c.label||c.number)+'</option>').join('');
function view(path,label,npc=false){return '<div class="view"><div class="stage '+controls[3].value+'"><img alt="'+esc(label)+'" src="'+path+'"></div>'+esc(label)+'</div>'}
function render(){const [group,config,palette]=controls.map(e=>e.value), background=controls[3].value;
if(group==='npc'){cards.innerHTML=d.npcs.map(e=>'<article class="card npc"><h2>'+esc(e.id+' · '+e.name)+'</h2><div class="tag">'+esc(e.role)+'</div><div class="views">'+view(e.current32,'32 px actuel',true)+view(e.current64,'64 px agrandi',true)+'</div><p class="note">Hauteur native : 30 px. Version 64 px indépendante à produire.</p></article>').join('');return;}
const entries=d.outfits.filter(e=>group==='all'||e.active_bank);
let scene='<article class="card scene"><h2>Comparaison en situation · cases de 64 px</h2><div class="scene-strip">';
for(const e of entries.slice(0,5)){scene+='<div class="scene-cell"><img class="outfit" alt="'+esc(e.name)+'" src="'+e.views[palette+'-'+config].direct64+'"></div>';}
for(const e of d.npcs.slice(3)){scene+='<div class="scene-cell"><img class="npc" alt="'+esc(e.name)+'" src="'+e.current64+'"></div>';}
scene+='</div><p class="note">Tenues : exports directs 64 px. PNJ : rendus actuels agrandis.</p></article>';
cards.innerHTML=scene+entries.map(e=>{const v=e.views[palette+'-'+config];return '<article class="card"><h2>'+esc(e.id+' · '+e.name)+'</h2><div class="tag">'+(e.active_bank?'Banque active':'Catalogue de revue')+' · source '+esc(e.review_state||'pending')+'</div><div class="views">'+view('../'+v.current32,'32 px actuel')+view('../'+v.current64,'64 px agrandi')+view(v.direct64,'64 px direct')+'</div><p class="note">Corps sans arme : '+e.body32_height+' px en 32 ; '+e.body64_height+' px dans l’export 64.</p></article>'}).join('');}
controls.forEach(e=>e.addEventListener('change',render));render();</script></html>'''
    (OUT / 'index.html').write_text(page, 'utf-8')


if __name__ == '__main__':
    main()

