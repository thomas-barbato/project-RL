"""Review conditioning: wrist registration stays identical at layer boundaries.

The fixed torso and arm masks share canonical target coordinates. Source paint
is displaced locally around the authored glove; the shoulder seam stays fixed.
No torso recovery patch, synthetic hand or equipment transform is introduced.
"""
from functools import lru_cache
import math
from PIL import Image, ImageChops

POLYS = {
    'independent': [
        [[105,435],[482,435],[483,473],[463,519],[425,556],[390,600],[340,660],[100,660]],
        [[770,435],[1150,435],[1160,660],[910,660],[865,600],[825,556],[792,519],[770,473]]],
    'rifle': [
        [[335,435],[472,435],[480,472],[495,480],[660,463],[672,590],[585,605],[505,605],[335,600]],
        [[770,435],[889,435],[917,486],[1160,486],[1160,642],[855,642],[770,552]]],
    'heavy': [
        [[340,435],[477,435],[485,500],[550,537],[550,674],[426,674],[284,608],[283,526]],
        [[778,435],[919,435],[960,526],[966,608],[894,674],[742,674],[742,537],[779,500]]],
    'pole': [
        [[340,435],[474,435],[479,476],[565,502],[673,538],[700,659],[598,683],[431,656],[337,568]],
        [[760,403],[822,392],[866,414],[896,467],[956,493],[956,627],[845,627],[799,550],[704,531],[699,410]]],
}

def polys(ident, pose):
    # The apron extends further than a glove. Exclude its painted hem from
    # the sleeve layer rather than moving torso pixels together with a hand.
    if ident == 'AR-08' and pose == 'rifle':
        return [[[335,435],[472,435],[480,472],[495,480],[660,463],[672,588],[585,590],[505,590],[335,580]], POLYS[pose][1]]
    if ident == 'AR-08' and pose == 'pole':
        return [[[340,435],[474,435],[479,476],[565,502],[673,538],[688,644],[598,654],[431,644],[337,568]], POLYS[pose][1]]
    return POLYS[pose]

def segment_distance(p, a, b):
    ax, ay = a; bx, by = b; x, y = p
    dx = bx-ax; dy = by-ay
    t = max(0, min(1, ((x-ax)*dx+(y-ay)*dy)/max(1, dx*dx+dy*dy)))
    return math.hypot(x-ax-t*dx, y-ay-t*dy)

def inside(p, poly):
    x, y = p; found = False
    for a, b in zip(poly, poly[1:]+poly[:1]):
        if (a[1]>y) != (b[1]>y) and x < (b[0]-a[0])*(y-a[1])/(b[1]-a[1])+a[0]:
            found = not found
    return found

class Registration:
    def __init__(self, art):
        self.art = art

    def inverse(self, ident, pose, side, x, y):
        poly = polys(ident, pose)[side]
        if not inside((x,y), poly):
            return x, y
        hand = self.art.rig.JOINTS[pose][1][side][2]
        actual = self.art.joints(ident, pose)[side][2]
        edge = min(segment_distance((x,y), a, b) for a,b in zip(poly, poly[1:]+poly[:1]))
        t = max(0, min(1, (230-math.hypot(x-hand[0], y-hand[1]))/165))
        t = t*t*(3-2*t)*min(1, edge/35)
        return tuple(p+(a-h)*t for p,a,h in zip((x,y), actual, hand))

    @lru_cache(maxsize=100)
    def mesh(self, ident, pose, side):
        w,h = self.art.source(ident, pose).size
        result = []
        for y in range(0,h,16):
            for x in range(0,w,16):
                right = min(w,x+16); bottom = min(h,y+16)
                corners = [(x,y),(x,bottom),(right,bottom),(right,y)]
                quad = [self.inverse(ident,pose,side,*p) for p in corners]
                result.append(((x,y,right,bottom),tuple(v for p in quad for v in p)))
        return result

    def registered(self, im, ident, pose, side):
        warped = im.transform(im.size, Image.Transform.MESH,
                              self.mesh(ident,pose,side), Image.Resampling.BICUBIC)
        return self.art.rig.normalize(warped)

    def mask(self, ident, pose):
        art = self.art
        return art.rig.mask((art.MS,art.MS),
                            [[art.rig.master(p) for p in poly] for poly in polys(ident,pose)])

    @lru_cache(maxsize=100)
    def body(self, ident, pose):
        art = self.art
        ref = art.rig.normalize(art.source(ident,'independent'))
        core = ref.copy()
        core.putalpha(ImageChops.multiply(ref.getchannel('A'),
                                         ImageChops.invert(self.mask(ident,'independent'))))
        for side in (0,1):
            layer = self.registered(art.source(ident,pose),ident,pose,side)
            mask = art.rig.mask(layer.size, [[art.rig.master(p) for p in polys(ident,pose)[side]]])
            layer.putalpha(ImageChops.multiply(layer.getchannel('A'),mask))
            core.alpha_composite(layer)
        return core
