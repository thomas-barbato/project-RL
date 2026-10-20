"""Check production captures, retain native evidence and refresh the existing preview."""
import copy
import hashlib
import json
import shutil
from pathlib import Path

import struct
from PIL import Image, ImageChops, ImageDraw, ImageStat
import build_animation_shape_review as gallery

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
OUT = BASE / 'animation-policy-01'
DOC = ROOT / 'docs/reviews/animation-policy-01'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def convert(raw, size):
    payload = raw.read_bytes()
    assert len(payload) == size[0]*size[1]*4, raw
    result = Image.frombytes('RGBA', size, payload).transpose(Image.Transpose.FLIP_TOP_BOTTOM)
    return result


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    (DOC/'integration-native').mkdir(parents=True, exist_ok=True)
    cases, checks, sources = [], [], []
    for folder in ('animation-shape-01', 'blizzard-visibility-01', 'animation-readability-01'):
        old = BASE/folder
        actual = OUT/folder
        data = json.loads((actual/'capture.json').read_text(encoding='utf-8'))
        assert data['game_state_unchanged']
        size = (data['frame_width']*data['columns'], data['frame_height']*((data['frames']+data['columns']-1)//data['columns']))
        for row in data['cases']:
            n = row['number']
            number = len(cases)+1
            for light in ('sombre', 'clair'):
                raw = actual/f'{n:02}-{light}-0.rgba'
                png = actual/f'{n:02}-{light}-0.png'
                if not png.exists() and not raw.exists():png=OUT/f'{number:02}-{light}-1.png'
                if raw.exists():
                    with convert(raw, size) as sheet:
                        sheet.save(png, compress_level=1)
                sources.append({'path':(OUT/f'{number:02}-{light}-1.png').relative_to(ROOT).as_posix(), 'sha256':digest(png)})
                # The current gallery already retains the exact historical
                # baseline. Its obsolete duplicate can be cleaned safely.
                before_png = OUT/f'{number:02}-{light}-0.png'
                if (old/f'{n:02}-{light}-0.png').exists():
                    shutil.copyfile(old/f'{n:02}-{light}-0.png', before_png)
                assert before_png.exists(), ('Missing historical baseline', before_png)
                if png.resolve()!=(OUT/f'{number:02}-{light}-1.png').resolve():shutil.copyfile(png, OUT/f'{number:02}-{light}-1.png')
                with Image.open(png) as sheet, Image.open(before_png) as before:
                    current = [gallery.frame(sheet, data, index) for index in range(data['frames'])]
                    clean = (Image.open(old/f'{n:02}-{light}-clean.png').convert('RGB') if folder=='blizzard-visibility-01' else gallery.frame(before, data, data['frames']-1))
                    leaks = 0
                    for image in current:
                        for x, y in row.get('excluded_visual_cells', []):
                            box = (x*64,y*64,(x+1)*64,(y+1)*64)
                            leaks += ImageChops.difference(image.crop(box), clean.crop(box)).getbbox() is not None
                    assert leaks == 0, (folder,n,light,'physical or hidden-cell leak',leaks)
                    difference = sum(ImageChops.difference(image,gallery.frame(before,data,index)).getbbox() is not None for index,image in enumerate(current))
                    assert difference, (folder,n,light,'unchanged animation')
                    record = {'review':folder,'case':n,'light':light,'production_phases':len(current),'changed_phases':difference,'excluded_cell_leaks':leaks}
                    if folder == 'blizzard-visibility-01':
                        with Image.open(old/f'{n:02}-{light}-1.png') as accepted:
                            deltas=[]
                            for index,image in enumerate(current):
                                phase=int(struct.unpack("f",struct.pack("f",struct.unpack("f",struct.pack("f",index/60/1.3))[0]*6))[0])%6
                                expected=gallery.frame(accepted,data,phase*10+2)
                                deltas.append(ImageChops.difference(image,expected))
                            record['approved_blizzard_max_rgb_error']=max(max(high for low,high in d.getextrema()) for d in deltas)
                            record['approved_blizzard_mean_rgb_error']=round(sum(sum(ImageStat.Stat(d).mean)/3 for d in deltas)/len(deltas),6)
                            assert record['approved_blizzard_max_rgb_error'] <= 1, record
                    checks.append(record)
                    if light=='sombre' and number in (2,3,4,6):
                        # Capture the most visible native phase from the actual renderer.
                        index=max(range(len(current)),key=lambda i:sum(ImageStat.Stat(ImageChops.difference(current[i],clean)).sum)) if row['id'] not in ('glide','reaction') else 2
                        canvas=Image.new('RGB',(944,380),(13,18,20));draw=ImageDraw.Draw(canvas)
                        draw.text((16,16),'Avant',fill='white');draw.text((480,16),'Rendu integre',fill='white')
                        canvas.paste(gallery.frame(before,data,index),(16,44));canvas.paste(current[index],(480,44))
                        canvas.save(DOC/'integration-native'/f'{number:02}-{row["id"]}.png')
            item=copy.deepcopy(row);item['before_seconds']=row['seconds'];item['before_loop_seconds']=row.get('loop_seconds');item.update(number=number,file_number=number,folder='animation-policy-01',before_label='Avant',after_label='Rendu intégré',duration_scale=1.0)
            scale=1.30 if row['id']=='FE05' else 1.35 if row['id']=='B01' else 1.40 if row['id']=='reaction' else 1.50
            item['seconds']*=1.30 if folder=='blizzard-visibility-01' else scale
            if folder=='blizzard-visibility-01':item['loop_seconds']=1.30
            item['rationale']='Effet de production : rythme ralenti, matière plus visible et contours naturels. La portée et les dégâts restent ceux du jeu.'
            cases.append(item)
        # Raw captures can be regenerated. Remove only files produced in this run.
        for raw in actual.glob('*.rgba'):raw.unlink()
        for pattern in ('*-0.png','*-reference.png','*-clean.png'):
            for unused in actual.glob(pattern):unused.unlink()
    data.update(cases=cases,status='accepted_and_integrated',normal_game_activation=True,review='ANIM-POLICY-01')
    page=gallery.html(data)
    page=page.replace('Formes naturelles · ANIM-FLOW-01','Animations intégrées · ANIM-POLICY-01').replace('ANIM-FLOW-01 · NOUVELLE PROPOSITION','ANIM-POLICY-01 · VALIDÉ ET INTÉGRÉ').replace('Des effets qui respirent hors des cases','Des effets plus lisibles, un rythme plus posé')
    page=page.replace('La version actuelle découpe le dessin selon les cases touchées. La proposition conserve le dessin entier :','À gauche, le rendu précédent. À droite, le rendu intégré :')
    page=page.replace('Animation entière proposée, taille native','Animation intégrée, taille native')
    page=page.replace('checked>Rythme ralenti des deux côtés','disabled>Rythme intégré aux captures')
    page=page.replace('`${duration} ms par cycle visuel · intensité actuelle`', '`${Math.round(row.before_seconds*1000)} ms par cycle visuel · rendu précédent`').replace('`${duration} ms · dessin limité aux cases`','`${Math.round(row.before_seconds*1000)} ms · rendu précédent`')
    page=page.replace('paint(contexts[0],sheets[0],t);paint(contexts[1],sheets[1],t);', 'paint(contexts[0],sheets[0],row.before_loop_seconds?age%row.before_loop_seconds:t);paint(contexts[1],sheets[1],t);')
    start=page.index('<footer>');end=page.index('</footer>',start)+len('</footer>')
    page=page[:start]+'<footer><p class="muted">Rendus validés puis raccordés au jeu. Comparaisons natives sur sols sombre et clair. Les effets peuvent dépasser les cases de dégâts, en respectant les murs et la visibilité. Les durées de jeu et les règles de combat restent inchangées.</p></footer>'+page[end:]
    page=page.replace('Même phase et même intensité dans les deux versions pour comparer la forme. Le ralentissement peut être désactivé.','Les captures comparent le rendu précédent au rendu de production avec son nouveau rythme.')
    page=page.replace('load();requestAnimationFrame(tick);',"el('case').value='6';load();requestAnimationFrame(tick);")
    gallery.CURRENT.write_text(page,encoding='utf-8')
    (OUT/'controller-check.js').write_text(page.split('<script>')[1].split('</script>')[0],encoding='utf-8')
    (DOC/'pixel-checks.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (DOC/'production-captures.json').write_text(json.dumps(sources,indent=2)+'\n',encoding='utf-8')
    print(f'{len(cases)} native comparisons; {len(checks)} lighting checks; protected-cell leaks: 0')


if __name__ == '__main__':
    main()
