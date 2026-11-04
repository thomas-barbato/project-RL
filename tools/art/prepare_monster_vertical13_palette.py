"""Separate red chitin, pale/olive joints and neutral shell in fixed palettes."""
import prepare_monster_vertical11_palette as common
from prepare_monster_vertical28_color import quant as source_quant,channel_lookup as source_lookup


def native_rank(value):
    return int(value+.5)


def quant(values):
    return source_quant(values,rank=native_rank)


def channel_lookup(source,target):
    return source_lookup(source,target,rank=native_rank)


def material(c):
    r,g,b,a=c
    if not a:return 0
    high=max(r,g,b);low=min(r,g,b)
    if r>35 and 100*g<72*r and 100*b<65*r:return 1
    if high>45 and 100*g>=65*r and 100*b<88*g and 100*(high-low)>12*high:return 2
    if high>24:return 3
    return 0


def main(batch=13):
    common.OUT=common.ROOT/f'assets/source-art/animation-polish-01/monster-vertical-{batch}/r01'
    common.REVIEW=f'MONSTER-VERTICAL{batch}-r01'
    common.MATERIAL_NAME='arthropod_materials'
    common.MATERIAL_KINDS=[(1,'red_chitin_and_folds'),(2,'pale_and_olive_joints'),(3,'neutral_shell')]
    common.material=material
    common.quant=quant
    common.channel_lookup=channel_lookup
    common.main()


if __name__=='__main__':main()
