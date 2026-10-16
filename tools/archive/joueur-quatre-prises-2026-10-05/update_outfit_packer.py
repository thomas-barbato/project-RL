from pathlib import Path
root=Path(__file__).resolve().parents[2]
p=root/'tools/art/build_player_outfits.py';s=p.read_text(encoding='utf-8')
s=s.replace('SOURCE = ROOT / "artifacts/joueur-armes-integrees-2026-10-05"','SOURCE = ROOT / "artifacts/joueur-quatre-prises-2026-10-05"\nPREVIOUS = ROOT / "artifacts/joueur-armes-integrees-2026-10-05"')
s=s.replace('SIDE = 128','SIDE = 160\nBODY_HEIGHT = 120')
s=s.replace('POSES = ("side", "two-handed", "raised")','POSES = ("side", "melee-two-handed", "two-handed", "raised", "dual-melee", "dual-raised", "dual-mixed")')
begin=s.index('        size = (round(frame.width * 120 / frame.height), 120)')
end=s.index('        result.append(tile)',begin)
s=s[:begin]+'''        alpha = frame.getchannel("A")
        bottom = alpha.getbbox()[3]-1
        sole = alpha.crop((0, max(0, bottom-5), frame.width, bottom+1)).getbbox()
        center = (sole[0]+sole[2]-1)/2
        half_band = max(2, round(frame.width*0.025))
        top = alpha.crop((round(center)-half_band, 0, round(center)+half_band+1, bottom+1)).getbbox()[1]
        scale = BODY_HEIGHT/(bottom-top+1)
        size = (round(frame.width*scale), round(frame.height*scale))
        x, y = round(SIDE/2-center*scale), SIDE-4-size[1]
        if x < 1 or y < 1 or x+size[0] >= SIDE:
            raise ValueError(f"Figure exceeds common body frame: {name}/{index}: {size}, {x}/{y}")
        tile = Image.new("RGBA", (SIDE, SIDE))
        tile.paste(frame.resize(size, Image.Resampling.NEAREST), (x, y))
''' + s[end:]
begin=s.index('    classes = frames("classes-armees.png", 3, 3, 9)')
end=s.index('    output = {}',begin)
s=s[:begin]+'''    previous_classes = frames("classes-armees.png", 3, 3, 9, PREVIOUS)
    unarmed = frames("classes-casquees-poses.png", 3, 3, 3, UNARMED_SOURCE)
    unarmed += frames("tenues-side-final.png", 4, 4, 14, UNARMED_SOURCE)
    prepared = {"unarmed": unarmed}
    for pose in POSES:
        if pose in ("two-handed", "raised"):
            row = 1 if pose == "two-handed" else 2
            classes = previous_classes[row*3:row*3+3]
            armor = frames(f"tenues-{pose}.png", 4, 4, 14, PREVIOUS)
        else:
            classes = frames(f"classes-{pose}.png", 3, 1, 3)
            armor = frames(f"tenues-{pose}.png", 4, 4, 14)
        # Canonical single-hand figures hold the weapon in the character RIGHT hand.
        if pose == "side":
            classes = [tile.transpose(Image.Transpose.FLIP_LEFT_RIGHT) for tile in classes]
        if pose == "raised":
            classes = [tile.transpose(Image.Transpose.FLIP_LEFT_RIGHT) for tile in classes]
            armor = [tile.transpose(Image.Transpose.FLIP_LEFT_RIGHT) for tile in armor]
        prepared[pose] = classes + armor
''' + s[end:]
s=s.replace('for side in (32, 128):','for side in (40, 128):')
begin=s.index('    names = ["classes-armees.png"]')
end=s.index('    manifest = {',begin)
s=s[:begin]+'''    sources = [SOURCE/f"{family}-{pose}.png" for pose in POSES if pose not in ("two-handed", "raised") for family in ("classes", "tenues")]
    sources += [PREVIOUS/"classes-armees.png", PREVIOUS/"tenues-two-handed.png", PREVIOUS/"tenues-raised.png"]
''' + s[end:]
s=s.replace('"version": 1, "outfit_count": 17, "armor_armed_variants": 42,','"version": 2, "outfit_count": 17, "armor_armed_variants": 98,')
s=s.replace('"feet_baseline": 31,','"source_frame": SIDE, "body_height": BODY_HEIGHT, "world_cell": 40, "world_body_height": 30, "feet_baseline": 155,')
s=s.replace('"sources": {name: hashlib.sha256((SOURCE/name).read_bytes()).hexdigest() for name in names},','"sources": {str(path.relative_to(ROOT)).replace("\\\\", "/"): hashlib.sha256(path.read_bytes()).hexdigest() for path in sources},')
begin=s.index('def preview(prepared):')
end=s.index('\n\ndef main():',begin)
s=s[:begin]+'''def preview(prepared):
    board = Image.new("RGBA", (1280, 17*176+36), (19, 25, 27, 255))
    draw = ImageDraw.Draw(board)
    for col, pose in enumerate(POSES):
        draw.text((col*160+160, 6), pose, fill="white")
    for index in range(17):
        y = index*176+36
        draw.text((8, y+20), ("Breche", "Vigie", "Creuset")[index] if index<3
                  else str(98+index)+" "+MODELS[index-3], fill="white")
        for col, pose in enumerate(POSES):
            board.alpha_composite(prepared[pose][index].resize((128, 128), Image.Resampling.NEAREST), (160+col*160, y))
            board.alpha_composite(prepared[pose][index].resize((40, 40), Image.Resampling.NEAREST), (200+col*160, y+130))
    board.save(SOURCE / "tenues-assemblees.png")
''' + s[end:]
s=s.replace('3 poses, 32/128 px','7 armed poses, 40/128 px; common 30px body')
p.write_text(s,encoding='utf-8',newline='\n')
p=root/'tools/art/build_cyberpunk_assets.py';s=p.read_text(encoding='utf-8')
s=s.replace('{"pistols", "handguns", "revolvers"}','{"pistols", "handguns", "revolvers", "submachine_guns"}')
s=s.replace('if model["group"] == "melee":\n        return "Side"','if model["group"] == "melee":\n        return "MeleeTwoHanded" if model["category"] in {"axes", "swords"} else "Side"')
needle='    outputs = {\n        "items-128.png"'
s=s.replace(needle,'''    ground = [grid_image(icon["sprite"]) for icon in icons["icons"]]
    melee_source = ROOT/"artifacts/joueur-quatre-prises-2026-10-05/icone-melee-source.png"
    sword = Image.open(melee_source).convert("RGBA")
    sword.putalpha(sword.getchannel("A").point(lambda a: 255 if a >= 128 else 0))
    sword = sword.crop(sword.getchannel("A").getbbox())
    sword.thumbnail((28, 28), Image.Resampling.NEAREST)
    tile = Image.new("RGBA", (32, 32))
    tile.paste(sword, ((32-sword.width)//2, (32-sword.height)//2))
    ground[keys.index("melee")] = tile
    outputs = {
        "items-128.png"''')
s=s.replace('"ground-32.png": atlas([grid_image(icon["sprite"]) for icon in icons["icons"]], 32),','"ground-32.png": atlas(ground, 32),')
p.write_text(s,encoding='utf-8',newline='\n')
