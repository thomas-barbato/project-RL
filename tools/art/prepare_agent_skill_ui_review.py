"""Build SKILLS-UI01 from reviewed skills, without activating new game art."""
from pathlib import Path
import hashlib
import json
import re
import shutil

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
OUT = BASE / 'skill-window-review-01'
OUT.mkdir(exist_ok=True)
(OUT / 'icons').mkdir(exist_ok=True)
curves_path = BASE / 'skill-rank-curves-provisional-02.json'
curves = json.loads(curves_path.read_text('utf-8'))
catalogue = json.loads((BASE / 'review-primary-catalogue-273.json').read_text('utf-8'))

# Bold silhouettes and two material tones. These are interface drawings, never
# alternate weapons, bodies or gameplay effects.
SHAPES = {
 'blade': 'M29 46L29 28L39 9L43 13L35 31L35 46ZM24 44H40V49H24ZM29 49V57H35V49',
 'gun': 'M10 22H47L54 27V33H34L29 46H20L23 32H10ZM43 25H55V29H43',
 'heavy': 'M6 23H43L50 28V39H34L30 50H19L22 39H10ZM43 26H59V33H43ZM10 39H18V47H10ZM13 17H35V23H13',
 'shield': 'M15 13H49V34L43 47L32 55L21 47L15 34ZM22 20H42V32L37 42L32 46L27 42L22 32Z',
 'heart': 'M32 51L12 32Q6 16 19 14Q27 11 32 20Q39 10 47 15Q59 20 51 33Z',
 'battery': 'M22 12H42V18H47V52H17V18H22ZM24 23H40V29H24ZM24 35H40V45H24',
 'flame': 'M32 8Q42 25 39 32L48 21Q59 40 45 51Q29 61 17 48Q9 33 24 20Q22 34 28 32Q35 24 32 8Z',
 'ice': 'M32 7L44 22L48 43L32 57L16 43L20 22ZM20 22H44M16 43H48M32 7V57M20 22L32 43L44 22',
 'drop': 'M32 8L49 32Q59 53 32 56Q6 53 15 32ZM23 38Q19 48 28 49',
 'eye': 'M7 32Q32 7 57 32Q32 56 7 32ZM23 32A9 9 0 1 0 41 32A9 9 0 1 0 23 32',
 'skull': 'M16 36Q7 11 30 10Q57 9 49 36L42 41V53H22V41ZM21 25H28V32H21ZM36 25H43V32H36ZM28 42V51M36 42V51',
 'paw': 'M19 47Q19 37 28 32Q34 28 43 38Q51 51 40 53Q34 49 28 53Q17 55 19 47ZM12 19Q22 12 23 26Q17 35 12 19ZM29 10Q39 7 38 24Q31 32 29 10ZM46 17Q58 14 50 31Q41 34 46 17',
 'head': 'M18 46L13 29L20 9L30 19L43 9L51 29L46 46L32 55ZM20 29L28 33M44 29L36 33M27 42L32 46L37 42',
 'drone': 'M17 24L24 16H40L47 24V42H17ZM9 23H17V40H9ZM47 23H55V40H47ZM23 28H41V35H23ZM25 42V49H39V42',
 'gear': 'M26 8H38L40 16L46 13L54 21L50 28L58 31V40L50 42L53 49L44 55L38 50L35 58H26L24 50L16 53L10 44L16 38L8 35V26L16 23L13 16L22 10L26 16ZM25 33A8 8 0 1 0 41 33A8 8 0 1 0 25 33',
 'mine': 'M12 38L19 28H45L52 38V47H12ZM26 22H38V28H26ZM24 15H40M32 10V22',
 'grenade': 'M22 22H42L46 31V47L38 55H26L18 47V31ZM26 13H38V22H26ZM35 12L48 17V28M24 33H40M24 43H40',
 'hex': 'M32 8L53 20V44L32 56L11 44V20ZM32 15L48 42H16ZM20 22L32 50L44 22Z',
 'boot': 'M21 11H38L35 39L50 43V52H12V44L21 35ZM22 22H36M20 32H35',
 'mask': 'M14 16L32 9L50 16V37L40 51H24L14 37ZM19 25H29V31H19ZM35 25H45V31H35ZM23 41H41',
 'wing': 'M9 17L30 30L32 52L21 41L14 29ZM55 17L34 30L32 52L43 41L50 29Z',
 'ring': 'M12 32A20 20 0 1 0 52 32A20 20 0 1 0 12 32M23 32A9 9 0 1 1 41 32A9 9 0 1 1 23 32',
 'wall': 'M10 20H54V52H10ZM10 31H54M10 42H54M23 20V31M40 20V31M31 31V42M23 42V52M40 42V52',
 'beam': 'M10 27H43L56 32L43 37H10ZM10 17L18 23M10 47L18 41',
 'claw': 'M17 9L12 48L22 37L28 14ZM33 8L27 49L37 39L43 13ZM48 10L42 50L52 40L56 19',
 'node': 'M11 11H23V23H11ZM42 11H54V23H42ZM26 42H38V54H26ZM23 18H42M17 23L32 42L48 23',
 'fang': 'M10 17Q32 8 54 17L50 28H43L40 43L33 28H30L23 43L20 28H14ZM14 43Q32 53 50 43',
 'target': 'M15 32A17 17 0 1 0 49 32A17 17 0 1 0 15 32M8 32H22M42 32H56M32 8V22M32 42V56',
 'cloak': 'M32 9L19 20L10 51Q32 61 54 51L45 20ZM25 23H39L36 35H28ZM19 41L16 49M45 41L48 49',
 'chain': 'M10 24L21 13Q34 6 36 20L26 31Q12 41 10 24ZM28 40L39 29Q53 22 54 38L42 49Q28 58 28 40ZM25 29L39 35',
 'burst': 'M32 6L37 22L53 13L45 29L59 34L43 38L50 54L35 45L29 59L25 43L9 50L18 36L5 28L22 26L16 10L29 21Z',
}
ACCENTS = {
 'plus':'M44 40H52V45H57V53H52V58H44V53H39V45H44Z',
 'cross':'M13 12L51 52M51 12L13 52',
 'arrow':'M6 50H23V42L36 53L23 61V56H6Z',
 'up':'M40 19L49 8L58 19H53V32H45V19Z',
 'down':'M40 43H45V30H53V43H58L49 55Z',
 'spark':'M45 6L47 13L54 15L47 18L45 25L42 18L35 15L42 13Z',
 'clock':'M40 46A10 10 0 1 0 60 46A10 10 0 1 0 40 46M50 40V47L54 50',
 'slash':'M8 53L54 9M14 56L58 16',
 'sound':'M37 15Q57 32 37 49M44 9Q66 32 44 55',
 'dots':'M10 48H16V54H10ZM25 48H31V54H25ZM40 48H46V54H40',
 'wave':'M4 44Q11 36 18 44T32 44T46 44T60 44',
 'orbit':'M4 35Q23 7 58 29Q60 47 7 49',
 'mark':'M25 25H39V39H25Z',
}
RECIPES = {
 'B01':('blade','burst'), 'ME02':('blade','orbit'), 'ME03':('blade','slash'), 'ME04':('shield','arrow'), 'ME05':('blade','boot'), 'ME06':('blade','ring'),
 'B03':('heart','plus'), 'B02':('shield','hex'), 'SU03':('shield','claw'), 'SU04':('heart','clock'), 'SU05':('drop','cross'), 'SU06':('heart','spark'), 'BD06':('battery','skull'),
 'D56':('drone','eye'), 'D57':('drone','battery'), 'D58':('drone','claw'), 'D59':('drone','gun'), 'D66':('drone','shield'), 'DP01':('drone','clock'), 'DP02':('drone','up'), 'O03':('node','target'), 'DR01':('drone','plus'),
 'O02':('node','cross'), 'AF02':('shield','drop'), 'AF03':('mask','cross'), 'AF04':('heart','cross'), 'AF05':('eye','orbit'), 'AF06':('skull','arrow'),
 'DI01':('gun','beam'), 'DI02':('gun','dots'), 'DI03':('gun','burst'), 'DI04':('gun','target'), 'DI05':('gun','chain'), 'DI07':('gun','fang'),
 'T02':('boot','hex'), 'ES03':('mask','dots'), 'ES05':('boot','wave'), 'ES06':('mask','shield'),
 'FE01':('flame','beam'), 'FE02':('fang','flame'), 'FE03':('wall','flame'), 'FE04':('flame','up'), 'FE05':('flame','burst'), 'FE06':('flame','skull'),
 'FR01':('ice','beam'), 'FR02':('ice','ring'), 'FR03':('wall','ice'), 'FR04':('ice','wave'), 'FR05':('ice','dots'), 'FR06':('ice','burst'),
 'A01':('hex','beam'), 'A02':('hex','burst'), 'A03':('hex','battery'), 'OC04':('ring','drop'), 'OC05':('beam','fang'), 'OC07':('skull','clock'), 'HY05':('hex','drop'),
 'M01':('head','claw'), 'TR61':('mask','burst'), 'TR63':('shield','head'), 'TR64':('head','hex'), 'TR65':('wing','hex'), 'M03':('paw','up'),
 'M02':('claw','beam'), 'HY07':('drone','burst'), 'HY04':('node','arrow'), 'HY06':('head','chain'),
 'BM01':('blade','flame'), 'BM02':('blade','fang'), 'BM03':('blade','clock'), 'BM05':('blade','hex'),
 'T01':('target','mark'), 'T03':('gun','up'), 'BD04':('target','clock'),
 'HL01':('heavy','wall'), 'HL02':('heavy','spark'), 'HL03':('heavy','wave'), 'HL04':('heavy','gear'), 'HL05':('grenade','burst'), 'HL06':('grenade','fang'),
 'SF00':('cloak','eye'), 'SF01':('boot','battery'), 'SF02':('cloak','down'),
 'SM00':('mine','spark'), 'SM01':('mine','node'), 'SM02':('mine','drop'), 'SM03':('mine','ice'), 'SM04':('mine','ring'), 'SM05':('mine','wave'),
 'SG00':('grenade','arrow'), 'SG01':('grenade','eye'), 'SG02':('grenade','sound'), 'SG03':('grenade','up'), 'SG04':('grenade','plus'),
}
COLORS = {
 'Mêlée':('#c48d68','#fae0ad'), 'Survie':('#3b9991','#b3eed5'), 'Drones':('#547e99','#b8ebf1'),
 'Affaiblissements':('#755e9f','#cfc0ee'), 'Attaques à distance':('#507bad','#bbdcff'), 'Esquive':('#6486aa','#c9e1f1'),
 'Feu':('#d75327','#ffd383'), 'Froid':('#4b94b2','#dbfbff'), 'Énergie occulte':('#9c3549','#e9a6a0'),
 'Transformations':('#70834c','#dce89a'), 'Pouvoirs hybrides':('#7960ad','#d4bef8'), 'Buffs de mêlée':('#a8704e','#fbd0a3'),
 'Buffs à distance':('#618aad','#c4e3f3'), 'Armes lourdes':('#8c7372','#e4d0bc'),
 'Furtivité':('#3b8d80','#b4e4d1'), 'Mines':('#9a8049','#ebdba2'), 'Grenades':('#838452','#e8e8b5'),
}

def icon(sid, branch):
    color, light = COLORS[branch]
    main, secondary = RECIPES[sid]
    primary = f'<path d="{SHAPES[main]}" fill="{color}" stroke="#090c11" stroke-width="3.8" stroke-linejoin="round"/>'
    if secondary == 'clock':
        detail = f'<circle cx="50" cy="46" r="10" fill="{light}" stroke="#090c11" stroke-width="2.4"/><path d="M50 40V47L54 50" fill="none" stroke="#090c11" stroke-width="2.4" stroke-linecap="round"/>'
    elif secondary in SHAPES:
        detail = f'<g transform="translate(25 26) scale(.55)"><path d="{SHAPES[secondary]}" fill="{light}" stroke="#090c11" stroke-width="5" stroke-linejoin="round"/></g>'
    else:
        detail = f'<path d="{ACCENTS[secondary]}" fill="{light if secondary not in ["cross","slash","sound","wave","orbit"] else "none"}" stroke="{light}" stroke-width="3" stroke-linejoin="round" stroke-linecap="round"/>'
    return f'<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 64 64"><rect x="1.5" y="1.5" width="61" height="61" rx="7" fill="#151b25" stroke="#46505d" stroke-width="2"/><path d="M9 5H45L59 19V53H9Z" fill="#1e2833"/>{primary}{detail}</svg>'

rows = {c['id']: (b['name'], c['role']) for b in catalogue['branches'] for c in b['candidates']}
skills = []
sources = []
for s in curves['skills']:
    sid = s['skill_id']
    if sid.startswith('SF'): branch, kind = 'Furtivité', 'active' if sid == 'SF00' else 'passive'
    elif sid.startswith('SM'): branch, kind = 'Mines', 'active' if sid == 'SM00' else 'passive'
    elif sid.startswith('SG'): branch, kind = 'Grenades', 'active' if sid == 'SG00' else 'passive'
    elif sid == 'DR01': branch, kind = 'Drones', 'active'
    else: branch, kind = rows[sid]
    card_path = ROOT / s['source_card']['reference']
    card = json.loads(card_path.read_text('utf-8'))
    # RP is retained verbatim. Later presentation rules discard prose duplicated
    # after that paragraph, rank breakdowns and obsolete formula sections.
    copy = card.get('player_copy_markdown', card.get('shown_copy_markdown', ''))
    match = re.search(r'^\*([^*\n]+)\*\s*$', copy, re.M)
    rp = card.get('retained_roleplay', '') or (match.group(1) if match else '')
    if not rp:
        files = sorted(BASE.glob(f'review-individual-description-{sid}-*.json'))
        for p in reversed(files):
            c = json.loads(p.read_text('utf-8'))
            m = re.search(r'^\*([^*\n]+)\*\s*$', c.get('player_copy_markdown', ''), re.M)
            if m: rp = m.group(1); break
    if not rp:
        raise ValueError(f'Missing reviewed RP for {sid}')
    title = s['title'].split(' — ')[0]
    requires = curves['global_constraints']['secondary_dependencies'].get(sid)
    if sid not in RECIPES: raise ValueError(f'Missing unique icon recipe {sid}')
    (OUT / 'icons' / f'{sid}.svg').write_text(icon(sid, branch), 'utf-8')
    skill = dict(id=sid, title=title, branch=branch, kind=kind, roleplay=rp,
                 learned_cap=s['learned_rank_cap'], requires=requires,
                 parameters=s['parameters'], source=s['source_card'])
    skills.append(skill)
    sources.append(dict(id=sid, source=s['source_card'], rp_sha256=hashlib.sha256(rp.encode()).hexdigest(),
                        icon_sha256=hashlib.sha256((OUT / 'icons' / f'{sid}.svg').read_bytes()).hexdigest()))
assert len(skills) == len(RECIPES) == 94
assert len({s['icon_sha256'] for s in sources}) == 94
data = dict(review_id='SKILLS-UI01', visual_status='proposed_not_integrated',
            character_profile='illustrative_level1_attributes5_before_equipment', skills=skills)
(OUT / 'skills.json').write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', 'utf-8')
(OUT / 'skills-data.js').write_text('window.SKILL_REVIEW='+json.dumps(data,ensure_ascii=False).replace('</','<\\/')+';\n', 'utf-8')
(OUT / 'manifest.json').write_text(json.dumps(dict(review_id='SKILLS-UI01', status='awaiting_visual_review',
    count=94, icon_native_sizes=[64,32,24], source_curves_sha256=hashlib.sha256(curves_path.read_bytes()).hexdigest(),
    sources=sources),ensure_ascii=False,indent=2)+'\n','utf-8')
for name in ['index.html','style.css','review.js']:
    shutil.copyfile(Path(__file__).with_name('agent_skill_ui_review') / name, OUT / name)
print(json.dumps({'review_id':'SKILLS-UI01','skills':94,'distinct_icons':94,'output':str(OUT)}))
