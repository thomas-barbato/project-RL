"""U01 r06: complete architectural contours, separate props, varied conduits."""
from PIL import Image, ImageDraw, ImageFont
import build_street_refined_review as scene
import show_street_scenery_library as library

PREVIOUS=scene.ART
ART=PREVIOUS.parent/'u01-expansion-r06'
REPLACEMENTS=[
    ('warehouse',(651,403),'Entrepôt en L · extrémité sud fermée'),
    ('house-a-body',(527,339),'Logement · intérieur'),
    ('house-b-body',(495,347),'Boutique · intérieur'),
    ('west-garage',(326,309),'Garage · conduits rouillés'),
    ('east-long',(265,563),'Bâtiment long · conduits acier'),
    ('crate',(46,40),'Caisse entière'),
    ('cabinet',(40,48),'Armoire extérieure entière'),
    ('barrel',(24,32),'Fût entier')]


def contour_board():
    rows=[('warehouse','KIT-U01-009 · Entrepôt'),
          ('house-a-body','KIT-U01-018 · Logement'),
          ('house-b-body','KIT-U01-019 · Boutique')]
    images=[Image.open(ART/(key+'-native.png')).convert('RGBA') for key,title in rows]
    height=12+sum(im.height+75 for im in images)
    out=Image.new('RGBA',(730,height),(17,26,31,255));d=ImageDraw.Draw(out)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',20);y=12
    for (_,title),im in zip(rows,images):
        d.text((20,y),title,font=font,fill='#dec898');y+=40
        assert 28+im.width<out.width and y+im.height+8<out.height
        d.rectangle((12,y-4,718,y+im.height+8),fill='#303b42',outline='#566970')
        out.alpha_composite(im,(28,y));y+=im.height+35
    out.convert('RGB').save(scene.OUT/'contours-corriges-native.png')


def build():
    ART.mkdir(exist_ok=True)
    rows=scene.read(PREVIOUS/'scenery-library.json')
    replacements={}
    for ident,size,title in REPLACEMENTS:
        source=ART/(ident+'-source.png');native=ART/(ident+'-native.png')
        scene.isolated(source,size,bottom=ident!='warehouse').save(native)
        replacements[ident]=dict(id=ident,title=title,file=scene.rel(native),source=scene.rel(source))
    for index,row in enumerate(rows):
        if row['id'] in replacements:rows[index]=replacements.pop(row['id'])
    rows.extend(replacements.values())
    exports={r['id']:scene.ROOT/r['file'] for r in rows}
    exports['ROOF-U01-02A-body']=exports['house-a-body']
    exports['ROOF-U01-02B-body']=exports['house-b-body']
    # Complete outside objects are independent placements, not part of an
    # architectural silhouette. Leave the shutter and the walkway unobstructed.
    props=[dict(id='warehouse-'+key,file=scene.rel(exports[key]),at=at)
           for key,at in [('crate',[630,488]),('cabinet',[714,480]),('barrel',[774,496])]]
    scene.ART=ART
    scene.build(exports=exports,revision='r06',extra_props=props)
    library.ART=ART
    library.build(rows=rows,revision='r06')
    contour_board()
    scene.write(scene.DOC/'contour-checks-r06.json',dict(
        review='CONTOURS-U01-06',status='visual_review_pending',
        architectural_replacements=['warehouse','ROOF-U01-02A-body','ROOF-U01-02B-body'],
        conduit_palettes={'warehouse':'aged copper','west-garage':'rust brown','east-long':'blue grey steel'},
        new_separate_props=props,render_only_candidate=True,
        warehouse_south_wall='MUR-SUD-03: full-height south facade matches the garage facade; footprint retained; terminal corner pillars aligned; useless rubble removed',
        warehouse_native_size=[651,403],
        previous_sources_preserved=True,unchanged_sources_reused=True))


if __name__=='__main__':build()
