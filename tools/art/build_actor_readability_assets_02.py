"""Freeze the three selected native paintings and preserve existing palette IDs."""
from pathlib import Path
import hashlib,json,re,shutil
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/sprites/actor-readability-02'
DOC=ROOT/'docs/reviews/actor-readability-01'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def lum(rgb):return sum(v*w for v,w in zip(rgb,(.2126,.7152,.0722)))
def rgb(v):return [(v>>16)&255,(v>>8)&255,v&255]

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    selection=json.loads((DOC/'selection-02.json').read_text('utf-8'))
    entries=[]
    for ref in selection['selected_pixel_references']:
        source=ROOT/ref['candidate_native'];assert sha(source)==ref['candidate_native_sha256']
        target=OUT/f"{ref['number']:03}-0.png";shutil.copyfile(source,target)
        entries.append({'number':ref['number'],'variant':0,'path':target.relative_to(ROOT).as_posix(),'sha256':sha(target),'selected_reference':ref})
    base=Image.open(OUT/'008-0.png').convert('RGBA')
    mask=Image.new('RGBA',base.size)
    for y in range(128):
        for x in range(128):
            r,g,b,a=base.getpixel((x,y))
            # The cream carapace, crest and claws are the material. Dark flesh,
            # cavities and saturated eyes keep their painted pixels.
            if a and r>78 and r>=g>=b and g>=r*.63 and r-b>3:
                mask.putpixel((x,y),(255,0,0,a))
    mask.save(OUT/'008-material.png')
    maps='\n'.join((ROOT/f'assets/sprites/campaign-monsters/{f}').read_text('utf-8') for f in ['material-palettes.rs','blue-material-palettes.rs'])
    for variant,body in re.findall(r'MaterialPalette \{ number: 8, variant: (\d+), colors: &\[(.*?)\] \}',maps):
        stops=sorted([(rgb(int(a,16)),rgb(int(b,16))) for a,b in re.findall(r'\(0x([\da-f]+), 0x([\da-f]+)\)',body)],key=lambda p:lum(p[0]))
        image=base.copy()
        for y in range(128):
            for x in range(128):
                if not mask.getpixel((x,y))[3]:continue
                pixel=base.getpixel((x,y));l=lum(pixel[:3]);lo=hi=stops[0]
                if l>=lum(stops[-1][0]):lo=hi=stops[-1]
                else:
                    for a,b in zip(stops,stops[1:]):
                        if lum(a[0])<=l<=lum(b[0]):lo,hi=a,b;break
                t=0 if lo==hi else (l-lum(lo[0]))/(lum(hi[0])-lum(lo[0]))
                delta=[lo[1][c]-lo[0][c]+t*((hi[1][c]-hi[0][c])-(lo[1][c]-lo[0][c])) for c in range(3)]
                image.putpixel((x,y),tuple(max(0,min(255,round(pixel[c]+delta[c]))) for c in range(3))+(pixel[3],))
        target=OUT/f'008-{variant}.png';image.save(target)
        assert image.getchannel('A').tobytes()==base.getchannel('A').tobytes()
        assert all(a==b for a,b,m in zip(image.get_flattened_data(),base.get_flattened_data(),mask.get_flattened_data()) if m[3]==0)
        entries.append({'number':8,'variant':int(variant),'path':target.relative_to(ROOT).as_posix(),'sha256':sha(target),'palette':'existing campaign material palette, interpolated on the new cream plates','mask':'assets/sprites/actor-readability-02/008-material.png'})
    manifest={'review_id':'ACTOR-READABILITY01-INTEGRATION02','native_cell':64,'canvas':128,'entries':entries,'existing_ids_and_stats_preserved':True}
    (OUT/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    code='// Selected ACTOR-READABILITY01 paintings and existing material variants.\nfn readability_bytes(name: &str) -> Option<&\'static [u8]> { Some(match name {\n'
    code+=''.join(f'    "VIS-{e["number"]:03}-{e["variant"]}" => include_bytes!("../{e["path"]}"),\n' for e in entries)
    code+='    _ => return None,\n}) }\n'
    (ROOT/'src/game_art_readability_catalog.rs').write_text(code,encoding='utf-8')
    print(json.dumps({'selected_models':3,'native_appearances':len(entries)}))

if __name__=='__main__':main()
