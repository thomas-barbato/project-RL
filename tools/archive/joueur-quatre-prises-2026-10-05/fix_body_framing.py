from pathlib import Path
root=Path(__file__).resolve().parents[2]
p=root/'tools/art/build_player_outfits.py';s=p.read_text(encoding='utf-8')
start=s.index('    result = []\n',s.index('def frames('));end=s.index('        alpha = frame.getchannel("A")\n        bottom',start)
s=s[:start]+'''    alpha = sheet.getchannel("A").point(lambda a: 255 if a >= 128 else 0)
    grouped = alpha.filter(ImageFilter.MaxFilter(3))
    grouped_pixels = grouped.load()
    original_pixels = alpha.load()
    figures = {}
    for component in components((x,y) for y in range(sheet.height) for x in range(sheet.width) if grouped_pixels[x,y]):
        points = [point for point in component if original_pixels[point]]
        if len(points) < 100: continue
        # Assign the whole connected figure to its cell, then crop its actual bounds.
        # Blades and boots may cross an imaginary grid line; that line must not clip them.
        cx, cy = sum(x for x,y in points)/len(points), sum(y for x,y in points)/len(points)
        index = min(rows-1, int(cy*rows/sheet.height))*columns + min(columns-1, int(cx*columns/sheet.width))
        if index not in figures or len(points) > len(figures[index]): figures[index] = points
    result = []
    for index in range(count):
        points = figures[index]
        left, top = min(x for x,y in points), min(y for x,y in points)
        right, bottom = max(x for x,y in points)+1, max(y for x,y in points)+1
        frame = sheet.crop((left, top, right, bottom))
        kept = Image.new("L", frame.size)
        for x,y in points: kept.putpixel((x-left,y-top), 255)
        frame.putalpha(kept)
''' +s[end:]
s=s.replace('half_band = max(2, round(frame.width*0.025))','half_band = max(2, round(frame.height*0.10))')
p.write_text(s,encoding='utf-8',newline='\n')
