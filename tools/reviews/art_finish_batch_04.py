"""ART-FINISH-04: two remaining current worn outfit candidates, not runtime assets."""
import json
import sys
from PIL import Image
from art_finish_batch_02 import ROOT,sha,write
from art_finish_batch_03 import condition as condition_outfits

ART=ROOT/'assets/source-art/art-finish-review-04'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-04'
OUT=ROOT/'artifacts/art-quality-audit-01'
ENTRIES=[
    ('FIN-26','Robe de la Seconde Ombre','core:veste_de_la_seconde_ombre',217,109),
    ('FIN-27','Tenue du nomade','core:veste_a_fibres_croisees',218,114),
]


def prepare():
    assert not (DOC/'acceptance.json').exists(), 'Accepted review is frozen'
    ART.mkdir(parents=True,exist_ok=True)
    DOC.mkdir(parents=True,exist_ok=True)
    atlas=Image.open(ROOT/'assets/sprites/equipment-native64/outfits-128.png')
    for code,_,_,outfit,_ in ENTRIES:
        index=(outfit-212)*3*16
        x,y=index%16*128,index//16*128
        atlas.crop((x,y,x+128,y+128)).save(ART/f'{code}-reference.png')
    if not (DOC/'production-before.json').exists():
        paths=[p for p in (ROOT/'assets/sprites').rglob('*') if p.is_file()]
        paths+=[ROOT/'assets/maps/prison-street-expanded-01'/n for n in ['street.json','street.png','layout.txt']]
        write(DOC/'production-before.json',{p.relative_to(ROOT).as_posix():sha(p) for p in paths})
    print('Prepared FIN-26 and FIN-27 references; ordinary artwork unchanged')


if __name__=='__main__':
    if '--condition' in sys.argv:
        condition_outfits(art=ART,doc=DOC,entries=ENTRIES,review_id='ART-FINISH-04')
    else:
        prepare()
