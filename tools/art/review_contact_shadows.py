"""Separate contact-shadow layers matching the existing runtime's dark bands.

This draws only the code-native shadow effect, never character pixels.
"""
import math

from PIL import Image, ImageDraw


def contact_shadow(art, anchor):
    bounds = art.getbbox()
    assert bounds
    left, top, right, bottom = bounds
    band = max(4, min(12, round((bottom-top)*0.08)))
    alpha = art.getchannel('A')
    xs = [x for y in range(bottom-band, bottom)
          for x in range(left, right) if alpha.getpixel((x, y)) > 128]
    assert xs
    width = max(16, min(art.width-8, max(xs)-min(xs)+6))
    center = round((min(xs)+max(xs))/2)
    center = max(math.ceil(width/2)+1, min(art.width-math.ceil(width/2)-1, center))
    y = min(art.height-5, anchor[1]-1)
    shadow = Image.new('RGBA', art.size)
    # Same colors/alphas and three-band shape as src/game_art.rs; 64 px scale.
    for dy, w in [(-2, width-8), (0, width), (2, width-8)]:
        layer = Image.new('RGBA', art.size)
        draw = ImageDraw.Draw(layer)
        draw.rectangle((center-w//2, y+dy, center-w//2+w-1, y+dy+1),
                       fill=(10, 15, 17, round(255*0.24)))
        shadow.alpha_composite(layer)
    layer = Image.new('RGBA', art.size)
    draw = ImageDraw.Draw(layer)
    w = max(2, width-12)
    draw.rectangle((center-w//2, y, center-w//2+w-1, y+1),
                   fill=(8, 12, 14, round(255*0.38)))
    shadow.alpha_composite(layer)
    return shadow, dict(center=[center, y], width=width, band_height=2,
                        palette_origin='src/game_art.rs::draw_contact_shadow')
