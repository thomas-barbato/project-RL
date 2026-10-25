"""Compare production compositions with frozen approvals and real GPU captures."""
from pathlib import Path
import argparse
import hashlib
import json

from PIL import Image, ImageDraw, ImageFont

from prepare_weapon_readability_review_01 import ROOT, OUT, NAMES, read, write, load, sha, light_actor

DEST = OUT/'integration'
DOC = ROOT/'docs/reviews/weapon-readability-02'
PAGES = {'WR-AX01':'two-handed-axe','WR-RF01':'rifle','WR-MG02':'heavy-machinegun',
         'WR-FT02':'heavy-flamethrower','WR-GL02':'heavy-grenade','WR-RL02':'heavy-rocket'}


def sheet(folder, index):
    im = load(folder/f'outfit-{index:02}.png')
    if read(folder/'capture.json')['exported_pngs_need_vertical_flip']:
        im = im.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
    return im


def cell(im, column):
    return im.crop((column%16*128, column//16*128, column%16*128+128, column//16*128+128))


def cpu():
    before = ROOT/'artifacts/actor-readability-integration-02/compositor'
    after = DEST/'compositor'
    baseline = read(DOC/'integration-02-baseline-hashes.json')
    labels = baseline['configurations']
    assert labels == read(after/'capture.json')['configurations']
    manifest = read(ROOT/'assets/sprites/weapon-readability-03/manifest.json')
    entries = {e['model']:e for e in manifest['entries']}
    expected = {f'weapon-{model:03}' for model in entries}
    melee_cleanup = {f'weapon-{model:03}' for model in [*range(61,68),*range(88,94)]} - expected
    unchanged = changed = 0
    max_color = max_alpha = max_premult = 0
    approved_cases = 0
    cleanup_changes = 0
    for index in range(25):
        new = sheet(after,index)
        for column,label in enumerate(labels):
            b = cell(new,column)
            same = hashlib.sha256(b.tobytes()).hexdigest() == baseline['rgba_sha256_by_outfit'][index][column]
            if label not in expected | melee_cleanup:
                assert same, (index,label,'unexpected composition change')
            if label in expected:
                assert not same, (index,label,'approved drawing missing')
            unchanged += same
            changed += not same
            cleanup_changes += label in melee_cleanup and not same
            if label not in expected:
                continue
            model = int(label.split('-')[1])
            e = entries[model]
            actual = light_actor(b).crop((16,16,112,112))
            approved = load(OUT/'cells'/f'{e["id"]}-{index:02}-final.png')
            for p,q in zip(actual.get_flattened_data(),approved.get_flattened_data()):
                max_alpha = max(max_alpha,abs(p[3]-q[3]))
                if min(p[3],q[3]) >= 128:
                    max_color = max(max_color,*(abs(p[c]-q[c]) for c in range(3)))
                max_premult = max(max_premult,*(abs(p[c]*p[3]-q[c]*q[3])/255 for c in range(3)))
            approved_cases += 1
    assert changed+unchanged == 3100 and approved_cases == 150
    assert changed == approved_cases+cleanup_changes and cleanup_changes > 0
    assert max_alpha <= 1 and max_color <= 2 and max_premult <= 2, (max_alpha,max_color,max_premult)
    for e in manifest['entries']:
        native = load(ROOT/e['native'])
        origin = e['origin']
        runtime = load(ROOT/e['runtime'])
        assert runtime.crop((origin[0],origin[1],origin[0]+native.width,origin[1]+native.height)).tobytes() == native.tobytes()
        assert sha(ROOT/e['runtime']) == e['runtime_sha256']
    record = dict(compositions=changed+unchanged, changed_compositions=changed,
                  approved_drawing_changes=approved_cases,melee_arm_cleanup_changes=cleanup_changes,
                  unrelated_compositions_byte_identical=unchanged, approved_outfit_cases=approved_cases,
                  maximum_review_rgb_delta=max_color,maximum_review_alpha_delta=max_alpha,
                  maximum_premultiplied_review_delta=max_premult,source_native_pixels_preserved=True)
    write(DOC/'integration-03-cpu-checks.json',record)
    # The native before/after boards are retained as review evidence. Old raw
    # baseline sheets can be removed once their RGBA hashes have been frozen.
    if (before/'capture.json').exists():
        melee_boards(before,after,labels)
    print(json.dumps(record))


def melee_boards(before,after,labels):
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',11)
    for model in [63,64,88]:
        im = Image.new('RGB',(584,454),'#182126')
        draw = ImageDraw.Draw(im)
        draw.text((6,4),f'Mêlée à deux mains · modèle {model} · 64 px natifs',font=font,fill='white')
        for rank in range(5):
            draw.text((140+rank*96,24),f'{rank+1:02}',font=font,fill='#b9cdd2')
        for cls,class_index in enumerate([0,3]):
            for row,(folder,label) in enumerate([(before,'Avant'),(after,'Corrigé')]):
                y=43+(cls*2+row)*100
                draw.text((4,y+36),NAMES[class_index],font=font,fill='white')
                draw.text((4,y+51),label,font=font,fill='#b9cdd2')
                for rank in range(5):
                    source=cell(sheet(folder,class_index*5+rank),labels.index(f'weapon-{model:03}'))
                    actor=light_actor(source).crop((16,16,112,112))
                    im.paste(actor,(100+rank*96,y),actor)
        im.save(DEST/f'melee-{model}-native.png',optimize=True)


def gpu():
    manifest = read(ROOT/'assets/sprites/weapon-readability-03/manifest.json')
    labels = read(DEST/'compositor/capture.json')['configurations']
    cases = []
    visual_cases = [(e['id'],e['model'],PAGES[e['id']]) for e in manifest['entries']]
    visual_cases += [('MELEE-CUFF63',63,'two-handed-axe-combat'),('MELEE-CUFF88',88,'two-handed-sword')]
    for ident,model,page in visual_cases:
        board = load(DEST/'gpu'/f'native-{page}.png')
        for index in range(25):
            actual = cell(sheet(DEST/'compositor',index),labels.index(f'weapon-{model:03}'))
            expected = light_actor(actual)
            origin = [141+index%5*150,72+index//5*150]
            rendered = board.crop((origin[0],origin[1],origin[0]+128,origin[1]+128))
            deltas = []
            for p,q in zip(expected.get_flattened_data(),rendered.get_flattened_data()):
                if p[3] == 255:
                    deltas.extend(abs(p[c]-q[c]) for c in range(3))
            mean = sum(deltas)/len(deltas)
            assert len(deltas)>100 and mean<.8, (ident,index,mean,max(deltas))
            cases.append(dict(id=ident,outfit=index,opaque_rgb_mean_error=mean,max_error=max(deltas)))
    for page,model in [('rifle-gray',2),('heavy-machinegun-gray',31)]:
        board = load(DEST/'gpu'/f'native-{page}.png')
        for index in range(25):
            source = cell(sheet(DEST/'compositor',index),labels.index(f'weapon-{model:03}'))
            origin = [141+index%5*150,72+index//5*150]
            rendered = board.crop((origin[0],origin[1],origin[0]+128,origin[1]+128))
            for p,q in zip(source.get_flattened_data(),rendered.get_flattened_data()):
                if p[3]==255:
                    assert max(q[:3])-min(q[:3])<=1, (page,index,q)
    write(DOC/'integration-03-gpu-checks.json',dict(cases=cases,
        actual_item_bindings=True,opaque_actor_pixels_match_cpu=True,grayscale_outfit_cases=50,
        gpu_capture=read(DEST/'gpu/gpu-checks.json')))
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',11)
    for name,ids in [('heavy',['WR-MG02','WR-FT02','WR-GL02','WR-RL02']),('axe-rifle',['WR-AX01','WR-RF01'])]:
        im = Image.new('RGB',(80+len(ids)*88,548),'#182126')
        draw = ImageDraw.Draw(im)
        draw.text((5,4),'Rendu du moteur · 64 px natifs · armures 05',font=font,fill='white')
        for col,ident in enumerate(ids):
            draw.text((84+col*88,24),ident.split('-')[1],font=font,fill='#b9cdd2')
            image = load(DEST/'gpu'/f'native-{PAGES[ident]}.png')
            for cls,label in enumerate(NAMES):
                if col == 0:
                    draw.text((2,80+cls*100),label,font=font,fill='white')
                # Keep native pixels; trim only the transparent-canvas margin.
                x,y = 141+4*150,72+cls*150
                sprite = image.crop((x+20,y+16,x+108,y+112))
                im.paste(sprite,(80+col*88,43+cls*100))
        im.save(DEST/f'{name}-engine-native.png',optimize=True)
    print(json.dumps(dict(gpu_matched_approved_outfits=150,gpu_matched_melee_cleanup_outfits=50,grayscale_outfits=50)))


if __name__ == '__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--gpu',action='store_true')
    args=parser.parse_args()
    if args.gpu:
        gpu()
    else:
        cpu()
