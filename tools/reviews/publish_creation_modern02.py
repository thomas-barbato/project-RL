"""Publish lossless native UI animation and a GIF from the engine review capture."""
from pathlib import Path
import hashlib,json,shutil
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/character-creation-review-02/current'
DOC=ROOT/'docs/reviews/character-creation-ui-02'

def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def verify_timeline(path,frames,durations,expected):
    # Animation encoders coalesce identical held frames; compare by time.
    with Image.open(path) as movie:
        encoded=movie.n_frames;index=0;movie.seek(index);movie.load()
        end=movie.info['duration'];now=0
        for n,source in enumerate(frames):
            while now>=end:
                index+=1;assert index<encoded
                movie.seek(index);movie.load();end+=movie.info['duration']
            assert movie.convert('RGB').tobytes()==expected(source).convert('RGB').tobytes(),n
            now+=durations[n]
        assert end==now
        return encoded
def main():
    if (DOC/'acceptance-02B.json').exists():raise SystemExit('02-B accepted: publish further changes as a new revision.')
    capture=OUT/'modern-readable-animation'
    checks=json.loads((capture/'animation-checks.json').read_text('utf8'))
    frames=sorted((capture/'frames').glob('*.png'))
    assert checks['frames']==len(frames)==145 and checks['game_fingerprint_unchanged'] and checks['turn_and_rng_unchanged']
    assert all(0<=r['class_progress']<=1 and 0<=r['detail_progress']<=1 and r['cached_scalars']<=128 for r in checks['observations'])
    def rgb(path):return Image.open(path).convert('RGB')
    for path in frames:
        reader=rgb(path).crop((890,202,1214,660))
        assert sum(max(pixel)>100 for pixel in reader.get_flattened_data())>100,path.name
    first=rgb(frames[0]);webp=OUT/'modern-02B.webp'
    webp_durations=[33,33,34]*48+[33]
    first.save(webp,save_all=True,append_images=(rgb(p) for p in frames[1:]),duration=webp_durations,loop=0,lossless=True,quality=100,method=4)
    # Keep a byte-exact native animation; GIF is only a compatibility preview.
    encoded_webp=verify_timeline(webp,frames,webp_durations,rgb)
    sheet=Image.new('RGB',(first.width,first.height*5))
    for n,key in enumerate([20,48,75,100,144]):sheet.paste(rgb(frames[key]),(0,n*first.height))
    palette=sheet.quantize(colors=256,method=Image.Quantize.MEDIANCUT)
    def indexed(path):return rgb(path).quantize(palette=palette,dither=Image.Dither.NONE)
    gif=OUT/'modern-02B.gif'
    gif_durations=[30,30,40]*48+[30]
    indexed(frames[0]).save(gif,save_all=True,append_images=(indexed(p) for p in frames[1:]),duration=gif_durations,loop=0,optimize=True,disposal=1)
    encoded_gif=verify_timeline(gif,frames,gif_durations,indexed)
    shutil.copy2(OUT/'modern-readable/cold-start.png',OUT/'modern-02B.png')
    hashes={p.relative_to(ROOT).as_posix():digest(p) for p in frames}
    (DOC/'animation-frames-02B.json').write_text(json.dumps({'review':'CREATION-UI02-B','native_frames':145,'lossless_animation_frame_comparisons':145,'sha256':hashes},indent=2)+'\n','utf8')
    report=json.loads((DOC/'PROPOSITION-02B.json').read_text('utf8'))
    report['motion']['maximum_reveal_veil_opacity']=0.30
    report.update(status='native_prototype_awaiting_visual_approval',animation_checks=checks,reader_never_blank_samples=145,encoded_animation_frames={'webp':encoded_webp,'gif':encoded_gif,'source_time_samples_verified':145},media={p.name:{'path':p.relative_to(ROOT).as_posix(),'bytes':p.stat().st_size,'sha256':digest(p)} for p in [webp,gif,OUT/'modern-02B.png']})
    (DOC/'PROPOSITION-02B.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'review':'CREATION-UI02-B','lossless_frames_verified':145,'webp_bytes':webp.stat().st_size,'gif_bytes':gif.stat().st_size,'normal_game_enabled':False}))

if __name__=='__main__':main()
