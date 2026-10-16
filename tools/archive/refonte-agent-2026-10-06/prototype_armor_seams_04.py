"""Layer-mask prototype: replace in target coordinates without alpha cut seams."""
import diagnose_armor_seams_04 as diag
from PIL import Image, ImageDraw, ImageFont, ImageChops, ImageFilter

art=diag.art
POLYS={
 'independent':[
  [[105,435],[482,435],[483,473],[463,519],[425,556],[390,600],[340,660],[100,660]],
  [[770,435],[1150,435],[1160,660],[910,660],[865,600],[825,556],[792,519],[770,473]]],
 'rifle':[
  [[335,435],[472,435],[480,472],[495,480],[660,463],[672,590],[585,605],[505,605],[335,600]],
  [[770,435],[889,435],[917,486],[1160,486],[1160,642],[855,642],[770,552]]],
 'heavy':[
  [[340,435],[477,435],[485,500],[550,537],[550,674],[426,674],[284,608],[283,526]],
  [[778,435],[919,435],[960,526],[966,608],[894,674],[742,674],[742,537],[779,500]]],
 'pole':[
  [[340,435],[474,435],[479,476],[565,502],[673,538],[700,659],[598,683],[431,656],[337,568]],
  [[760,403],[822,392],[866,414],[896,467],[956,493],[956,627],[845,627],[799,550],[704,531],[699,410]]],
}
def replacement_body(ident,pose):
    ref=art.rig.normalize(art.source(ident,'independent'))
    src=art.source(ident,pose)
    target_old=art.rig.mask((art.MS,art.MS),[[art.rig.master(p) for p in poly] for poly in POLYS['independent']])
    target_new=art.rig.mask((art.MS,art.MS),[[art.rig.master(p) for p in poly] for poly in POLYS[pose]])
    result=ref.copy()
    result.putalpha(ImageChops.multiply(ref.getchannel('A'),ImageChops.invert(target_old)))
    for side in (0,1):
        mask=art.rig.mask((art.MS,art.MS),[[art.rig.master(p) for p in POLYS[pose][side]]])
        mask=mask.filter(ImageFilter.MaxFilter(17))
        layer=art.registered(src,ident,pose,side)
        layer.putalpha(ImageChops.multiply(layer.getchannel('A'),mask))
        result.alpha_composite(layer)
    return result

ids=['AR-N14','AR-08','AR-10','AR-11','AR-12']
font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',20)
board=Image.new('RGB',(2000,len(ids)*400),'#111c25');d=ImageDraw.Draw(board)
for row,ident in enumerate(ids):
    for col,pose in enumerate(art.POSES):
        im=replacement_body(ident,pose)
        bg=Image.new('RGBA',im.size,'#afb9bd');bg.alpha_composite(im)
        view=bg.crop((15*art.F,17*art.F,49*art.F,43*art.F)).resize((470,360),Image.Resampling.LANCZOS)
        board.paste(view,(col*500+15,row*400+32))
        d.text((col*500+10,row*400+5),ident+' · '+pose,fill='white',font=font)
board.save(diag.out/'target-mask-prototype.png')
