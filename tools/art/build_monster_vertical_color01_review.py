"""Compare the exact demon poses before/after palette, keeping all six models."""
import argparse
import json
import build_monster_vertical01_review as base

base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-color-01/r01/current'
base.PREFIX='monster-vertical-color-01/r01/current/'
base.REVIEW='MONSTER-VERTICAL-COLOR01-r01'

def main(page_only=False):
    old=json.loads((base.ROOT/'artifacts/animation-polish-01/monster-vertical-02/r01/current/checks.json').read_text(encoding='utf-8'))
    first=json.loads((base.ROOT/'artifacts/animation-polish-01/monster-vertical-01/r01/current/checks.json').read_text(encoding='utf-8'))
    previous={'items':[{**p,'capture_base':'monster-vertical-02/r01/current/'} for p in old['items']]+[{**p,'capture_base':'monster-vertical-01/r01/current/'} for p in first['items'] if p['id']!='MON-144-0'],
              'labels':['04 · Possédé · validé','05 · Démon massif · validé','06 · Cultiste à la hache · validé','01 · Cultiste · premier lot','02 · Cultiste au fusil · premier lot']}
    base.main(page_only,labels=['03 · Démon ailé · rouge harmonisé'],previous=previous,
              intro='Le lot 04–06 est validé. Sur 03, la palette du rouge est harmonisée avec le repos. Les poses, contours, cornes, griffes et le frontal sont conservés. Les six modèles restent sélectionnables.')
    html=base.PAGE.read_text(encoding='utf-8')
    start=html.index('<section id="monstres-vertical">'); end=html.index('</section>',start)
    section=html[start:end]
    section=section.replace('Nouveaux dessins en revue.','03 : couleur corrigée à examiner. 04–06 : dessins validés.')
    section=section.replace('Marche actuelle / nouvelle marche','Avant / Après').replace('Frontal conservé / nouvelle marche','Frontal conservé / Après')
    section=section.replace('Marche actuelle · nouvelle marche. Taille native, cases de 64 px ; mêmes décor, trajet et éclairage.','Même marche avant et après harmonisation du rouge. Même décor, trajet, éclairage et cadence ; aucune pose redessinée.')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','Comparer le rouge en marche au frontal, de face et de dos. La correction est une palette préparée par le moteur ; les PNG sources restent inchangés.')
    html=html[:start]+section+html[end:]
    # These are viewer captions only; captured actor pixels stay untouched.
    marker=html.index('<!-- MONSTER-VERTICAL01 SCRIPT -->')
    script=html[marker:].replace('marche actuelle au même rythme · nouvelle marche','avant · après').replace('Frontal conservé · nouvelle marche','Frontal conservé · après').replace('Marche actuelle au même rythme · nouvelle marche','Avant · après')
    base.PAGE.write_text(html[:marker]+script,encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--page-only',action='store_true'); args=parser.parse_args(); main(args.page_only)
