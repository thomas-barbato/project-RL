"""Build numerical GPU lighting data; retain every original drawing unchanged.

Correction is an inverse illumination field sampled by a background shader.
Lightmap is a wall-occluded RGB irradiance field, not a painted scene.
"""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'assets/prototypes/handcrafted-camp-01/r08'
OUT = ROOT / 'assets/prototypes/handcrafted-camp-01/r09'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def make_light(at, origin, box, radius=230, power=.6, facing=(0, 50)):
    return dict(at=at, origin=origin, box=box, radius=radius, power=power,
                bake_center=[at[0]+facing[0], at[1]+facing[1]])


def visible_field(rows, light, tx, ty, wall_surfaces=True):
    """DDA: illuminate the first wall surface, never the receiver behind it."""
    floor = np.array([[tile == '.' for tile in row] for row in rows])
    cx, cy = light['origin']
    assert floor[cy, cx]
    sx = np.clip(light['at'][0], cx * 64 + 1, (cx + 1) * 64 - 1)
    sy = np.clip(light['at'][1], cy * 64 + 1, (cy + 1) * 64 - 1)
    target_x = np.clip((tx // 64).astype(int), 0, floor.shape[1]-1)
    target_y = np.clip((ty // 64).astype(int), 0, floor.shape[0]-1)
    # At most 1/8 tile per ray step, including diagonal corner crossings.
    steps = int(np.ceil(light['radius'] / 8)) + 2
    visible = np.ones(tx.shape, dtype=bool)
    for i in range(1, steps + 1):
        x = np.clip(((sx + (tx - sx) * i / steps) // 64).astype(int), 0, floor.shape[1]-1)
        y = np.clip(((sy + (ty - sy) * i / steps) // 64).astype(int), 0, floor.shape[0]-1)
        visible &= floor[y, x] | ((x == target_x) & (y == target_y))
    if wall_surfaces:
        # A painted wall/floor margin can extend inside a conservative wall
        # cell. Light its face from a visible adjacent floor, rather than
        # showing a square spotlight aligned with that collision cell.
        gy, gx = np.mgrid[0:floor.shape[0],0:floor.shape[1]].astype(np.float32)
        lit_floor = visible_field(rows,light,gx*64+32,gy*64+32,False)&floor
        faces = np.zeros(floor.shape,dtype=bool)
        faces[1:] |= lit_floor[:-1]
        faces[:-1] |= lit_floor[1:]
        faces[:,1:] |= lit_floor[:,:-1]
        faces[:,:-1] |= lit_floor[:,1:]
        visible = np.where(floor[target_y,target_x],visible,faces[target_y,target_x])
    return visible


def main():
    c = json.loads((BASE/'camp.json').read_text(encoding='utf-8'))
    width, height = c['width']*64, c['height']*64
    rows = (ROOT/c['layout']).read_text(encoding='utf-8').splitlines()
    lights = [
        make_light([1125,122],[17,2],[1100,112,1150,130]),
        make_light([1950,122],[30,2],[1924,112,1976,130]),
        make_light([233,290],[3,5],[211,278,256,298],220,.62),
        make_light([233,761],[3,10],[211,744,256,770],220,.62,(0,-50)),
        make_light([35,522],[1,8],[27,496,42,548],195,.42,(50,0)),
        make_light([512,484],[8,8],[495,473,525,500],155,.4),
        make_light([783,435],[12,7],[760,424,808,442],285,.56),
        make_light([1536,435],[24,7],[1508,424,1558,442],285,.56),
        make_light([2291,435],[35,7],[2266,424,2318,442],285,.56),
        make_light([694,490],[10,8],[681,482,708,498],170,.3),
        make_light([2377,490],[37,8],[2363,482,2389,499],170,.3),
        make_light([1200,750],[18,12],[1182,740,1216,771],230,.62),
        make_light([2030,750],[31,12],[2012,740,2048,775],230,.62),
    ]
    for light in lights:
        light['power'] *= 1.35
    # Irradiance is low resolution and interpolated separately from the crisp
    # nearest-filtered artwork. No lighting work is repeated per game turn.
    scale = 4
    ty, tx = np.mgrid[0:height:scale, 0:width:scale].astype(np.float32)
    tx += scale/2; ty += scale/2
    ambient = np.array([.86,.88,.91], dtype=np.float32)
    irradiance = np.broadcast_to(ambient, (*tx.shape,3)).copy()
    checks = []
    for light in lights:
        d2 = (tx-light['at'][0])**2+(ty-light['at'][1])**2
        visible = visible_field(rows, light, tx, ty)
        falloff = np.maximum(0., (np.exp(-3*d2/light['radius']**2)-np.exp(-3))/(1-np.exp(-3)))
        w = falloff*visible*light['power']
        irradiance += w[...,None]*np.array([1.,.67,.28],dtype=np.float32)
        checks.append(dict(at=light['at'], lit_samples=int((w>.002).sum()),
                           blocked_samples_in_range=int(((~visible)&(d2<light['radius']**2)).sum())))
    # A wall immediately above the infirmary separates this floor receiver
    # from its lamp even though it lies inside the light's radius.
    probe = lights[11]
    assert not visible_field(rows, probe, np.array([[1184.]],np.float32), np.array([[608.]],np.float32))[0,0]
    assert visible_field(rows, probe, np.array([[1184.]],np.float32), np.array([[808.]],np.float32))[0,0]

    # Compensate broad painted warm pools mathematically in the renderer;
    # no original character, furnishing or architecture image is rewritten.
    py, px = np.mgrid[0:height,0:width].astype(np.float32)
    warmth = np.zeros((height,width),dtype=np.float32)
    centers = [l['bake_center'] for l in lights] + [[1125,800],[1952,800]]
    for bx, by in centers:
        warmth = np.maximum(warmth, np.exp(-((px-bx)/105)**2-((py-by)/95)**2))
    factors = np.stack((1/(1+.55*warmth),1/(1+.32*warmth),1/(1-.06*warmth)),axis=-1)
    correction = np.empty((height,width,4),dtype=np.uint8)
    correction[:,:,:3] = np.clip(np.rint(factors*127.5),0,255).astype(np.uint8)
    correction[:,:,3] = 0
    source = Image.open(ROOT/c['floor']).convert('RGBA')
    source.alpha_composite(Image.open(ROOT/c['structure']).convert('RGBA'))
    rgb = np.asarray(source)[:,:,:3].astype(np.float32)/255
    # Warm baked pools primarily add red/green. Use their less affected blue
    # channel to estimate a neutral diffuse surface; retain native shading.
    chroma = np.clip((rgb[:,:,0]-rgb[:,:,2])/.12,0,1)*warmth*.9
    neutral = rgb*factors
    diffuse = rgb[:,:,2,None]*np.array([1.03,1.05,1.04],dtype=np.float32)
    neutral = neutral*(1-chroma[:,:,None])+diffuse*chroma[:,:,None]
    factors = np.where(rgb>1/255,neutral/np.maximum(rgb,1/255),1.)
    correction[:,:,:3] = np.clip(np.rint(factors*127.5),0,255).astype(np.uint8)
    for light in lights:
        x0,y0,x1,y1 = light['box']
        ink = rgb[y0:y1,x0:x1]
        core = (ink[:,:,0]>.86)&(ink[:,:,1]>.78)&(ink[:,:,2]>.30)&(ink[:,:,0]>ink[:,:,2]*1.1)
        correction[y0:y1,x0:x1,3] = core.astype(np.uint8)*255
    OUT.mkdir(parents=True,exist_ok=True)
    Image.fromarray(correction).save(OUT/'illumination-correction.png',optimize=True)
    Image.fromarray(np.clip(np.rint(irradiance*127.5),0,255).astype(np.uint8)).save(OUT/'irradiance.png',optimize=True)
    c.update(revision='r09',atmosphere=dict(
        correction='assets/prototypes/handcrafted-camp-01/r09/illumination-correction.png',
        irradiance='assets/prototypes/handcrafted-camp-01/r09/irradiance.png',
        ambient=ambient.tolist(),lights=lights,lightmap_scale=scale,
        shadows='Static ray occlusion by the actual logical wall grid; first wall surface receives light.',
        baked_compensation='Inverse broad illumination field in GPU background shader; source artwork remains intact.',
        animations=['ANIM-CAMP-01','ANIM-CAMP-02','ANIM-CAMP-03']))
    c['new_lighting_system_implemented'] = True
    c['new_ambient_animations_implemented'] = True
    # Carry approval only when both the complete scene and its source files
    # still match the exact version reviewed by the player.
    acceptance = REVIEW/'acceptance-r09b.json'
    if acceptance.exists():
        accepted = json.loads(acceptance.read_text(encoding='utf-8'))
        scene = {k:v for k,v in c.items() if k not in ('art_approved','layout_approved','visual_acceptance')}
        scene_sha = hashlib.sha256(json.dumps(scene,sort_keys=True,ensure_ascii=False).encode('utf-8')).hexdigest()
        sources_match = all((ROOT/p).is_file() and hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h
                            for p,h in accepted['source_sha256'].items())
        if scene_sha==accepted['scene_sha256'] and sources_match:
            c.update(art_approved=True,layout_approved=True,
                     visual_acceptance='docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/acceptance-r09b.json')
    text = json.dumps(c,ensure_ascii=False,indent=2)+'\n'
    (OUT/'camp.json').write_text(text,encoding='utf-8')
    (REVIEW/'full-camp-r09.json').write_text(text,encoding='utf-8')
    report=dict(lights=checks,wall_probe_blocked=True,same_room_probe_lit=True,
                lightmap_size=[width//scale,height//scale],source_art_modified=False,
                maps=[dict(file=p.relative_to(ROOT).as_posix(),sha256=hashlib.sha256(p.read_bytes()).hexdigest())
                      for p in sorted(OUT.glob('*.png'))])
    (REVIEW/'lighting-data-r09.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(lights=len(lights),wall_occlusion_verified=True,source_art_unchanged=True)))


if __name__=='__main__':main()
