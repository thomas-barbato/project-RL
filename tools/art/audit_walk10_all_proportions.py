"""Read-only native64 audit: actual player compositor, campaign references, extras.

Numeric alerts are review signals. A projected silhouette is not a 3D body-volume
measurement: wings, overlapping limbs, crouching and foreshortening need review.
"""
from pathlib import Path
from statistics import median
import csv, hashlib, json, math, re
from PIL import Image, ImageDraw, ImageOps

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-10/full-audit'
DOC=ROOT/'docs/reviews/player-walk-10/full-audit'
BG='#253136'
FRAMES=[8,0,1,2,3,4,5]
CREATURE_OVERRIDE_MANIFEST=None

def read_export(path):return ImageOps.flip(Image.open(path).convert('RGBA'))
def alpha_hash(im):
    a=im.getchannel('A').point(lambda x:255 if x>=96 else 0)
    return hashlib.sha256(str(im.size).encode()+a.tobytes()).hexdigest()
def metrics(im):
    a=im.getchannel('A').point(lambda x:255 if x>=96 else 0);b=a.getbbox()
    if not b:raise ValueError('Empty body')
    x0,y0,x1,y1=b;h=y1-y0
    top=y0+round(h*.27);end=y0+round(h*.55)
    area=sum(a.histogram()[1:]);band=sum(a.crop((0,top,im.width,end)).histogram()[1:])
    return dict(bounds=list(b),height=h,width=x1-x0,opaque_area=area,upper_body_mean_occupied_width=band/(end-top))
def evaluate(front,poses,human=False):
    h=front['height'];heights=[p['geometry']['height'] for p in poses]
    widths=[p['geometry']['upper_body_mean_occupied_width']/front['upper_body_mean_occupied_width'] for p in poses]
    corrected=[p['geometry']['opaque_area']/front['opaque_area']/(p['geometry']['height']/h)**2 for p in poses]
    directional=[median(p['geometry']['height'] for p in poses if p['facing']==s) for s in [-1,1]]
    idle=[p['geometry'] for p in poses if p['frame']==8]
    signals=[]
    if max(heights)-h>2 or h-min(heights)>2:signals.append('front_height_difference')
    if max(heights)-min(heights)>2:signals.append('within_cycle_height_variation')
    if abs(directional[0]-directional[1])>2:signals.append('left_right_height_difference')
    if median(corrected)>(1.2 if human else 1.35) or median(corrected)<(.65 if human else .55):signals.append('projected_bulk_difference')
    if median(widths)/(median(heights)/h)>(1.15 if human else 1.45):signals.append('upper_silhouette_wider')
    return dict(front_height=h,walk_height_range=[min(heights),max(heights)],walk_height_by_direction=directional,
        idle_height_range=[min(p['height'] for p in idle),max(p['height'] for p in idle)],
        idle_area_ratio_range=[round(min(p['opaque_area']/front['opaque_area'] for p in idle),3),round(max(p['opaque_area']/front['opaque_area'] for p in idle),3)],
        median_area_ratio_height_normalized=round(median(corrected),3),upper_width_ratio_range=[round(min(widths),3),round(max(widths),3)],
        signals=signals,review_needed=bool(signals))

def paste(canvas,image,at):canvas.paste(image,at,image)
def project_front(image,side,foot,offset=0):
    # Campaign destination uses side native pixels, regardless of PNG dimensions.
    if image.size!=(side,side):image=image.resize((side,side),Image.Resampling.NEAREST)
    tile=Image.new('RGBA',(128,144));paste(tile,image,(64-foot[0],120-foot[1]+offset));return tile
def project_cycle(image,offset=0):
    tile=Image.new('RGBA',(128,144));paste(tile,image,(0,8+offset));return tile

def contact_pages(cases,folder,title,provider):
    folder.mkdir(exist_ok=True)
    records=[]
    for page,start in enumerate(range(0,len(cases),8)):
        chunk=cases[start:start+8];board=Image.new('RGB',(2040,38+len(chunk)*162),BG);d=ImageDraw.Draw(board)
        d.text((8,7),title+' | FACE puis 7 GAUCHE / 7 DROITE | taille native64',fill='white')
        for row,c in enumerate(chunk):
            yy=34+row*162
            d.text((5,yy+3),c['id'],fill='white')
            signal=' / '.join(c['evaluation']['signals']) or 'mesures stables'
            d.text((5,yy+18),f"{c['evaluation']['front_height']}px",fill='#bbc9cf')
            for col,im in enumerate(provider(c)):
                xx=120+col*128
                d.rectangle((xx+32,yy+56,xx+95,yy+119),outline='#46545a')
                paste(board,im,(xx,yy))
            d.text((126,yy+145),signal,fill='#f3bf76' if c['evaluation']['signals'] else '#a4c7af')
        name=f'{page:02}.png';board.save(folder/name)
        records.append({'page':str((folder/name).relative_to(DOC)).replace('\\','/'),'cases':[c['id'] for c in chunk]})
    return records

def players():
    raw=json.loads((ART/'players/geometry.json').read_text());body=[];weapons=[]
    for source,target in [(raw['body_cases'],body),(raw['weapon_cases'],weapons)]:
        for r in source:
            c=dict(r);c['id']=r['id']+f"-H{r['helmet']}"+(f"-W{r['weapon']:03}" if r['weapon'] is not None else '')
            c['evaluation']=evaluate(r['front'],r['poses'],True);target.append(c)
    base=[r for r in body if r['helmet']==0 and r['weapon'] is None]
    def images(c):
        sheet=read_export(ART/'players'/c['atlas'])
        # Player canvases use feet y96; the common contact-board feet are y120.
        return [project_cycle(sheet.crop((col*128,0,(col+1)*128,128)),16) for col in range(15)]
    pages=contact_pages(base,DOC/'players','PERSONNAGES - corps sans arme',images)
    armed=[r for r in body if r['weapon']==67]
    pages+=contact_pages(armed,DOC/'players-axe','PERSONNAGES - hache equipee / mesure corps sans arme',images)
    helmets=sorted([r for r in body if r['helmet']>0],key=lambda r:(len(r['evaluation']['signals']),r['evaluation']['median_area_ratio_height_normalized']),reverse=True)
    pages+=contact_pages(helmets[:24],DOC/'players-helmet-alerts','CASQUES - principaux signaux / pas une validation artistique',images)
    return dict(body_cases=body,weapon_cases=weapons,base_cases=base,pages=pages,
        full_body_pose_measurements=sum(len(c['poses']) for c in body),weapon_pose_measurements=sum(len(c['poses']) for c in weapons),weapon_sampling=raw['weapon_sampling'],not_tested=raw['not_tested'])

def campaigns():
    references=json.loads((ART/'campaign/references.json').read_text())['references']
    manifest={e['number']:e for e in json.loads((ROOT/'assets/animations/walk-07/manifest.json').read_text())['entries']}
    if CREATURE_OVERRIDE_MANIFEST:
        manifest.update({e['number']:e for e in json.loads(CREATURE_OVERRIDE_MANIFEST.read_text())})
    cycles={};poses={};cases=[];geometry_groups={};images={}
    for number,e in manifest.items():
        atlas=Image.open(ROOT/e['file']).convert('RGBA');p=[]
        for facing,row in [(-1,0),(1,1)]:
            for col,frame in enumerate(FRAMES):
                im=atlas.crop((col*128,row*128,(col+1)*128,(row+1)*128));m=metrics(im)
                p.append(dict(facing=facing,frame=frame,geometry=m));images[number,facing,frame]=im
        cycles[number]=e;poses[number]=p
    for r in references:
        number=r['number'];e=cycles[number]
        front=read_export(ART/'campaign'/r['file']);scale=r['side']/front.height
        if scale!=1:front=front.resize((r['side'],r['side']),Image.Resampling.NEAREST)
        fm=metrics(front);key=(number,alpha_hash(front));id=f"MON-{number:03}-{r['variant']}"
        c=dict(r,id=id,front=fm,poses=poses[number],evaluation=evaluate(fm,poses[number]),
            front_alpha_sha256=key[1],front_vertical_offset=e['front_vertical_offset'],cycle_file=e['file'])
        c['geometry_equivalent_to']=geometry_groups.setdefault(key,id);cases.append(c)
    representative=[c for c in cases if c['id']==c['geometry_equivalent_to']]
    def view(c):
        e=cycles[c['number']];front=read_export(ART/'campaign'/c['file'])
        return [project_front(front,c['side'],c['foot'],e['front_vertical_offset'])]+[project_cycle(images[c['number'],facing,frame],e['vertical_offset']) for facing in [-1,1] for frame in FRAMES]
    pages=contact_pages(representative,DOC/'campaign','CAMPAGNE - bases et geometries distinctes; couleurs regroupees',view)
    return dict(cases=cases,distinct_front_geometries=len(representative),pages=pages,
        base_cycles=129,base_pose_measurements=129*14,all_pair_pose_comparisons=len(cases)*14,
        normal_animated_pairs=sum(c['normal_cycle_active'] for c in cases),normal_static_pairs=sum(not c['normal_cycle_active'] for c in cases)),cycles,poses,images

def extras(cycles,poses,images):
    jobs={j['id']:j for j in json.loads((ROOT/'docs/reviews/player-walk-08/extra-generation.json').read_text(encoding='utf-8-sig'))}
    entries=json.loads((ROOT/'assets/animations/walk-08/extras/manifest.json').read_text());cases=[];views={}
    for e in entries:
        id=e['id'];j=jobs[id];front=Image.open(j['reference']).convert('RGBA');atlas=Image.open(ROOT/e['file']).convert('RGBA')
        pp=[];body=[]
        for facing in [-1,1]:
            row=e.get('left_row',0) if facing<0 else 1-e.get('left_row',0)
            for col,frame in enumerate(FRAMES):
                im=atlas.crop((col*128,row*128,(col+1)*128,(row+1)*128));pp.append(dict(facing=facing,frame=frame,geometry=metrics(im)));body.append(project_cycle(im,e['vertical_offset']))
        # Forms fill64; the three non-gunner drones draw their128 canvas at native size.
        front_view=Image.new('RGBA',(128,144));at=(32,56) if front.width==64 else (0,24)
        paste(front_view,front,at);views[id]=[front_view,*body]
        cases.append(dict(id=id,reference=str(Path(j['reference']).relative_to(ROOT)).replace('\\','/'),front=metrics(front),poses=pp,
            evaluation=evaluate(metrics(front),pp),normal_cycle_active=False,cycle_status='proposal_only'))
    gunner=Image.open(ROOT/'assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png').convert('RGBA')
    e=cycles[0]
    front_view=Image.new('RGBA',(128,144));paste(front_view,gunner,(32,56+e['front_vertical_offset']))
    views['DRONE-GUNNER']=[front_view]+[project_cycle(images[0,facing,frame],e['vertical_offset']) for facing in [-1,1] for frame in FRAMES]
    cases.append(dict(id='DRONE-GUNNER',front=metrics(gunner),poses=poses[0],evaluation=evaluate(metrics(gunner),poses[0]),normal_cycle_active=True,cycle_status='accepted_active'))
    pages=contact_pages(cases,DOC/'forms-drones','FORMES / DRONES - canonnier actif, autres cycles en revue',lambda c:views[c['id']])
    return dict(cases=cases,pages=pages,pose_measurements=140)

def static_inventory():
    result=[];bank=Image.open(ROOT/'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
    for index in range(27):
        raw=bank.crop((index%16*32,index//16*32,index%16*32+32,index//16*32+32)).resize((64,64),Image.Resampling.NEAREST)
        entry=dict(id=f'ACTOR-{index:02}',category='legacy_actor_index',normal_cycle_active=False,
            status='no_distinct_walk_cycle',note='same static source during grid glide')
        if 20<=index<=25:
            entry.update(raw_fallback_geometry=metrics(raw),front=None,note='Context-dependent modular service NPC; no distinct walk cycle. Raw fallback geometry is not a measurement of the modular outfit.')
        else:entry['front']=metrics(raw)
        result.append(entry)
    catalog=(ROOT/'assets/sprites/bestiary/catalog.rs').read_text()
    for n,side,index in re.findall(r'BankEntry\s*\{\s*number:\s*(\d+),\s*side:\s*(\d+),\s*index:\s*(\d+)',catalog):
        n,side,index=map(int,(n,side,index));sheet=Image.open(ROOT/f'assets/sprites/bestiary/living-{side}.png').convert('RGBA')
        # Portrait sheet bank uses16 columns; world draws side/32 cells.
        x=index%16*side;y=index//16*side
        raw=sheet.crop((x,y,x+side,y+side)).resize((side*2,side*2),Image.Resampling.NEAREST)
        if n==57:raw=Image.open(ROOT/'assets/sprites/art-finish-01/FIN-11.png').convert('RGBA')
        result.append(dict(id=f'BANK-{n:03}',category='legacy_bestiary',normal_cycle_active=False,front=metrics(raw),status='no_distinct_walk_cycle',note='accepted FIN-11 replacement' if n==57 else 'same static source during grid glide'))
    scene=json.loads((ROOT/'assets/maps/cryogenic-camp-01/camp.json').read_text())
    for a in scene['actors'][1:]:
        im=Image.open(ROOT/a['file']).convert('RGBA')
        result.append(dict(id=a['id'],category='camp_npc',normal_cycle_active=False,front=metrics(im),status='no_distinct_walk_cycle',note='idle breathing is separate from walking'))
    return result

def main():
    DOC.mkdir(exist_ok=True)
    player=players();campaign,cycles,poses,images=campaigns();extra=extras(cycles,poses,images);static=static_inventory()
    report=dict(review_id='WALK10-ALL-PROPORTIONS-AUDIT01',runtime_assets_modified=False,normal_release_modified=False,
        method='native64, alpha>=96; player body excludes held objects; all14 poses per full case. Signals require visual review and are not proof of anatomical scale changes.',
        thresholds={'height_pixels':2,'human_area_ratio_height_normalized':[.65,1.2],'other_area_ratio_height_normalized':[.55,1.35],'human_upper_width_height_normalized_max':1.15,'other_upper_width_height_normalized_max':1.45},
        players=player,campaign=campaign,extras=extra,static_inventory=static,
        derived_visuals={'AgentHologram':'inherits the existing player/form body with a color/opacity shader; no independent walking binding'},
        exclusions=['dead bodies','ground items','terrain','spell particles'],
        limits=['Not a full armor x helmet x weapon x pose Cartesian product.','Silhouette area changes can reflect pose overlap, wings or foreshortening.','Static banks are inventoried separately; absence of a gait does not count as a successful walk-proportion review.'])
    (DOC/'audit.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    rows=[]
    for group,values in [('player_body',player['body_cases']),('player_weapon_sample',player['weapon_cases']),('campaign',campaign['cases']),('extra',extra['cases'])]:
        for c in values:
            e=c['evaluation'];rows.append(dict(group=group,id=c['id'],front_height=e['front_height'],walk_min=e['walk_height_range'][0],walk_max=e['walk_height_range'][1],area_ratio=e['median_area_ratio_height_normalized'],signals=';'.join(e['signals']),normal_cycle_active=c.get('normal_cycle_active',True)))
    with (DOC/'cases.csv').open('w',newline='',encoding='utf-8-sig') as file:
        writer=csv.DictWriter(file,fieldnames=rows[0].keys());writer.writeheader();writer.writerows(rows)
    summary=dict(player_body_cases=len(player['body_cases']),player_full_pose_measurements=player['full_body_pose_measurements'],player_weapon_cases=len(player['weapon_cases']),weapon_sample_pose_measurements=player['weapon_pose_measurements'],
        base_player_outfits_with_signals=sum(c['evaluation']['review_needed'] for c in player['base_cases']),campaign_pairs=len(campaign['cases']),campaign_distinct_front_geometries=campaign['distinct_front_geometries'],campaign_base_pose_measurements=campaign['base_pose_measurements'],campaign_pair_pose_comparisons=campaign['all_pair_pose_comparisons'],
        campaign_bases_with_signals=sum(c['variant']==0 and c['evaluation']['review_needed'] for c in campaign['cases']),campaign_pairs_with_signals=sum(c['evaluation']['review_needed'] for c in campaign['cases']),normal_campaign_static_pairs=campaign['normal_static_pairs'],extras=len(extra['cases']),extras_with_signals=sum(c['evaluation']['review_needed'] for c in extra['cases']),static_indices=len(static),
        pages=player['pages']+campaign['pages']+extra['pages'])
    (DOC/'summary.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({k:v for k,v in summary.items() if k!='pages'},indent=2))

if __name__=='__main__':main()
