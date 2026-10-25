"""Native comparison boards from ordinary compositor exports."""
import json
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,font

OUT=ROOT/'artifacts/class-armor-integration-01'
DOC=ROOT/'docs/reviews/class-armor-integration-01/pose-fixes-03'


def tile(index,label):
    data=json.loads((OUT/'compositor/capture.json').read_text('utf-8'))
    cell=data['configurations'].index(label)
    im=Image.open(OUT/f'compositor/outfit-{index:02}.png').convert('RGBA')
    return im.crop((cell%16*128,cell//16*128,cell%16*128+128,cell//16*128+128))


def main():
    path=OUT/'pose-fixes-03';path.mkdir(exist_ok=True)
    board=Image.new('RGB',(620,470),'#172126');d=ImageDraw.Draw(board)
    for rank in range(5):d.text((120+rank*100,8),f'{rank+1:02}',font=font(14),fill='white')
    for row,(start,label,caption) in enumerate([
        (0,'weapon-101','Bastion\n1 pistolet'),(0,'dual-pistols','Bastion\n2 pistolets'),
        (15,'weapon-101','Arcaniste\n1 pistolet'),(15,'dual-pistols','Arcaniste\n2 pistolets')]):
        y=40+row*105;d.multiline_text((6,y+30),caption,font=font(12),fill='white')
        for rank in range(5):
            im=tile(start+rank,label).crop((16,16,112,112));board.paste(im,(110+rank*100,y),im)
    board.save(path/'all-tiers-native.png')
    before=Image.open(DOC/'before-metamorphe-open-helmet-rifle.png').convert('RGBA')
    images=[before,tile(20,'helmet-1')]+[tile(i,'helmet-1') for i in range(21,25)]
    board=Image.new('RGB',(620,360),'#273035');d=ImageDraw.Draw(board)
    for col,(label,im) in enumerate(zip(['Avant 01','ME-POST03','Réf. 02','Réf. 03','Réf. 04','Réf. 05'],images)):
        x=8+col*102;d.text((x,8),label,font=font(12),fill='white')
        native=im.crop((16,16,112,112));board.paste(native,(x,30),native)
        zoom=im.crop((40,34,88,96)).resize((96,124),Image.Resampling.NEAREST)
        board.paste(zoom,(x,160),zoom)
        d.line((x,282,x+96,282),fill='#7b9c9c')
    d.text((10,310),'En haut : pixels natifs. En bas : mêmes pixels au zoom ×2.',font=font(13),fill='white')
    board.save(path/'metamorphe-rifle-native.png')
    print(path)


if __name__=='__main__':main()
