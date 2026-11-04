"""Next three humanoid silhouettes on the accepted player gait; keep MON-144 frozen."""
from pathlib import Path
import hashlib
import json
from PIL import Image
from prepare_class_lateral_r02 import center

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/biped-lateral-02/r01'


def main():
    references=json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-01/r01/references.json').read_text(encoding='utf-8'))
    profiles=[]
    for number in [5,14,156]:
        r=next(r for r in references if r['number']==number)
        atlas=Image.new('RGBA',(1152,256));records=[];sources=[]
        for side,name in enumerate(['left','right']):
            path=OUT/f'MON-{number:03}-{name}-source.png';raw=Image.open(path).convert('RGBA')
            assert raw.getchannel('A').getextrema()[0]==0
            sources.append(path.relative_to(ROOT).as_posix())
            for phase in range(9):
                x,y=phase%3,phase//3
                tile=raw.crop((round(x*raw.width/3),round(y*raw.height/3),round((x+1)*raw.width/3),round((y+1)*raw.height/3)))
                box=tile.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox();assert box
                figure=tile.crop(box);scale=r['reference_height']/figure.height
                native=figure.resize((round(figure.width*scale),r['reference_height']),Image.Resampling.NEAREST)
                xx=round(64-center(figure)*scale);yy=112-r['reference_height']
                assert 0<=xx and xx+native.width<=128
                pose=Image.new('RGBA',(128,128));pose.alpha_composite(native,(xx,yy));atlas.alpha_composite(pose,(phase*128,side*128))
                records.append({'side':side,'phase':phase,'whole_figure_scale':scale,'bounds':pose.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()})
        for side in range(2):
            assert len({atlas.crop((p*128,side*128,(p+1)*128,(side+1)*128)).tobytes() for p in range(8)})==8
        path=OUT/f'MON-{number:03}-atlas.png';atlas.save(path)
        atlas.resize((2304,512),Image.Resampling.NEAREST).save(OUT/f'MON-{number:03}-poses-zoom.png')
        profiles.append({k:v for k,v in r.items() if k!='source'}|{'name':{5:'Possédé',14:'Cultiste',156:'Démon massif'}[number],
            'atlas':path.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'sources':sources,'bounds':records,'accepted':False})
    accepted=json.loads((ROOT/'docs/reviews/animation-polish-01/BIPED-LATERAL01-ACCEPTANCE-r03.json').read_text(encoding='utf-8'))
    for asset in accepted['assets']:
        assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']
    p=json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-01/r03/manifest.json').read_text(encoding='utf-8'))['profiles'][0]
    profiles.append({k:v for k,v in p.items() if k!='source'}|{'name':'Démon ailé validé','accepted':True})
    (OUT/'manifest.json').write_text(json.dumps({'review':'BIPED-LATERAL02-r01','native_cell_px':64,'normal_game_enabled':False,
        'step_ms':400,'cycle_ms':800,'phase_weights':[180,180,170,120,180,180,170,120],
        'phase_ms_integer_allocation':[111,111,104,74,111,111,104,74],'frame':[128,128],'anchor':[64,112],
        'reference':'assets/source-art/animation-polish-01/class-lateral-02/r02/bastion-atlas.png',
        'accepted_reference':'docs/reviews/animation-polish-01/BIPED-LATERAL01-ACCEPTANCE-r03.json','profiles':profiles},indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Prepared three new cycles at400 ms; frozen MON-144 reused without changes')


if __name__=='__main__':main()
