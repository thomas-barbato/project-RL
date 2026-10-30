"""Lossless browser copies of the real C06 framebuffer contact boards."""
from pathlib import Path
from PIL import Image
import json,hashlib
import argparse

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/vertical-walk-review-18/current/extension-C06/engine'

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--only');args=parser.parse_args()
    rows=[]
    manifest=OUT/'browser-captures.json'
    if args.only:
        rows=[r for r in json.loads(manifest.read_text('utf8'))['captures'] if r['case']!=args.only]
    for path in sorted(OUT.glob('*.png')):
        if args.only and path.stem!=args.only:continue
        image=Image.open(path).convert('RGBA');target=path.with_suffix('.webp');image.save(target,lossless=True,method=6,exact=True)
        decoded=Image.open(target).convert('RGBA');assert decoded.size==image.size and decoded.tobytes()==image.tobytes()
        rows.append({'case':path.stem,'file':target.name,'source_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'rgba_sha256':hashlib.sha256(image.tobytes()).hexdigest(),'lossless_roundtrip':True})
    (OUT/'browser-captures.json').write_text(json.dumps({'review':'VERT18-C06','captures':rows,'raw_pngs_preserved':True},indent=2)+'\n','utf8')
    print(json.dumps({'lossless_engine_boards':len(rows),'raw_pngs_preserved':True}))

if __name__=='__main__':main()
