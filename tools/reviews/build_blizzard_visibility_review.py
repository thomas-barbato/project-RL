"""BLIZZARD-VIS-01: verify real FR05 captures and append them to the current gallery."""
import copy
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageStat
import build_animation_shape_review as shapes

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/blizzard-visibility-01'
DOC = ROOT / 'docs/reviews/blizzard-visibility-01'


def main():
    data = json.loads((OUT/'capture.json').read_text(encoding='utf-8'))
    assert data['review'] == 'BLIZZARD-VIS-01' and data['real_FR05_cast'] and not data['normal_game_activation']
    size = (data['frame_width']*data['columns'], data['frame_height']*((data['frames']+data['columns']-1)//data['columns']))
    checks = []
    for row in data['cases']:
        for light in ('sombre','clair'):
            paths = [OUT/f'{row["number"]:02d}-{light}-{i}.png' for i in range(2)]
            for path in paths:
                raw=path.with_suffix('.rgba')
                if raw.exists():
                    payload=raw.read_bytes()
                    assert len(payload)==size[0]*size[1]*4
                    with Image.frombytes('RGBA',size,payload).transpose(Image.Transpose.FLIP_TOP_BOTTOM) as sheet:
                        sheet.save(path,compress_level=1)
            with Image.open(paths[0]) as before,Image.open(paths[1]) as after,Image.open(OUT/f'{row["number"]:02d}-{light}-clean.png') as clean:
                assert before.size==after.size==size
                clean=clean.convert('RGB')
                changed=overhang=0
                old_light=new_light=0.
                area=Image.new('L',clean.size,255);draw=ImageDraw.Draw(area)
                for x,y in row['affected_cells']:draw.rectangle((x*64,y*64,x*64+63,y*64+63),fill=0)
                first=shapes.frame(after,data,0)
                assert ImageChops.difference(first,shapes.frame(after,data,20)).getbbox(), 'Frozen persistent animation'
                for index in range(data['frames']):
                    a,b=shapes.frame(before,data,index),shapes.frame(after,data,index)
                    changed+=ImageChops.difference(a,b).getbbox() is not None
                    effect=ImageChops.difference(b,clean)
                    old_light+=sum(ImageStat.Stat(ImageChops.difference(a,clean)).sum)
                    new_light+=sum(ImageStat.Stat(effect).sum)
                    bands=effect.split();mask=ImageChops.lighter(ImageChops.lighter(bands[0],bands[1]),bands[2]).point(lambda v:255 if v else 0)
                    overhang+=ImageChops.multiply(mask,area).histogram()[255]
                    for x,y in row['excluded_visual_cells']:
                        box=(x*64,y*64,x*64+64,y*64+64)
                        assert ImageChops.difference(b.crop(box),clean.crop(box)).getbbox() is None,(row['title'],light,index,x,y)
                assert changed and overhang and new_light>old_light,(row['title'],light,new_light,old_light)
                checks.append({'number':row['number'],'light':light,'changed_phases':changed,'visual_overhang_pixel_samples':overhang,'excluded_cell_leaks':0,'effect_contrast_ratio':round(new_light/old_light,3),'persistent_animation_varies':True})
    combined=copy.deepcopy(json.loads((shapes.OUT/'capture.json').read_text(encoding='utf-8')))
    existing=len(combined['cases'])
    for row in data['cases']:
        item=copy.deepcopy(row);item['file_number']=row['number'];item['number']=existing+row['number'];item['folder']='blizzard-visibility-01';combined['cases'].append(item)
    page=shapes.html(combined)
    page=page.replace('Formes naturelles · ANIM-FLOW-01','Animations · formes et Blizzard')
    page=page.replace('ANIM-FLOW-01 · NOUVELLE PROPOSITION','ANIM-FLOW-01 + BLIZZARD-VIS-01 · PROPOSITIONS')
    page=page.replace('Des effets qui respirent hors des cases','Des effets plus naturels, un Blizzard plus lisible')
    page=page.replace('Les cases de jeu restent de <strong>64 px</strong>.','Les cases de jeu restent de <strong>64 px</strong>. Les exemples 05 à 07 comparent aussi la lisibilité de Blizzard, pendant son avertissement puis sa tempête active.')
    page=page.replace('Trois effets et un exemple contre une cloison, à taille native sur deux éclairages de revue.','Trois effets et un exemple contre une cloison, complétés par trois vues de Blizzard, à taille native sur deux éclairages de revue.')
    page=page.replace('Même phase et même intensité dans les deux versions pour comparer la forme.','Les deux versions sont synchronisées. Pour Blizzard, le contraste est renforcé dans la proposition.')
    # Start at the active Blizzard while keeping the earlier examples available.
    page=page.replace('load();requestAnimationFrame(tick);',"el('case').value='6';load();requestAnimationFrame(tick);")
    shapes.CURRENT.write_text(page,encoding='utf-8')
    DOC.mkdir(parents=True,exist_ok=True)
    (DOC/'proposal.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (OUT/'pixel-checks.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (OUT/'controller-check.js').write_text(page.split('<script>')[1].split('</script>')[0],encoding='utf-8')
    for row in data['cases']:
        for light in ('sombre','clair'):
            for panel in range(2):
                raw=OUT/f'{row["number"]:02d}-{light}-{panel}.rgba'
                if raw.exists():raw.unlink()
    print(shapes.CURRENT)


if __name__=='__main__':
    main()
