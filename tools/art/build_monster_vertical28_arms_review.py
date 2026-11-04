"""Reopen28 arms with direct previous/new comparison and accepted29/30."""
import argparse
import json
import re
import build_monster_vertical10_review as previous

base=previous.base
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-28-arms/r01/current'
base.PREFIX='monster-vertical-28-arms/r01/current/'
base.REVIEW='MONSTER-VERTICAL28-ARMS01-r01'

def retained_bank():
    retained=previous.retained_bank()
    retained['items']=[p for p in previous.previous.bank('monster-vertical-10') if p['id']!='MON-060-0']+retained['items']
    retained['labels']=['29 · Démon cornu aux sabots · validé','30 · Démon aux plaques osseuses · validé']+retained['labels']
    assert len(retained['items'])==len(retained['labels'])==29
    return retained

def label_comparison_columns(html):
    start=html.index('<!-- MONSTER-VERTICAL01 SCRIPT -->');end=html.index('<!-- /MONSTER-VERTICAL01 SCRIPT -->',start)
    script=html[start:end]
    marker='async function loadMvertical()'
    caption="function mverticalColumns(){const item=mverticalItems.find(i=>i.id===$('mvertical-item').value),headers=item.headers||['Frontal conservé','Marche actuelle','Nouvelle marche'],pair=$('mvertical-pair').value,columns=pair==='all'?[0,1,2]:pair==='idle'?[0,2]:[1,2];$('mvertical-pair').options[0].textContent=headers[1]+' / '+headers[2];$('mvertical-pair').options[1].textContent=headers[0]+' / '+headers[2];$('mvertical-columns').textContent=columns.map(i=>headers[i]).join(' · ')+'. Taille native, cases de 64 px.';}\n"
    assert marker in script
    script=script.replace(marker,caption+marker,1)
    script=script.replace("mverticalReady=true;mverticalPainted='';","mverticalReady=true;mverticalColumns();mverticalPainted='';",1)
    script,n=re.subn(r"\$\('mvertical-pair'\)\.onchange=.*?\n","$('mvertical-pair').onchange=()=>{mverticalColumns();mverticalPainted='';paintMvertical();};\n",script,count=1)
    assert n==1
    return html[:start]+script+html[end:]

def main(page_only=False):
    retained=retained_bank()
    path=base.OUT/'checks.json';data=json.loads(path.read_text(encoding='utf-8'))
    item=data['items'][0];assert item['id']=='MON-060-0'
    item['scope_note']='28 · bras repris · épaules relâchées, coudes abaissés, longues griffes vers le bas · 29 et 30 validés.'
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    base.main(page_only,labels=['28 · Démon rouge élancé · bras repris'],previous=retained,
              intro='Reprise ciblée des bras du n°28. Les n°29 et 30 sont validés et restent inchangés. Comparer le frontal conservé, les anciens bras et la nouvelle posture, de face et de dos à 400 ms. Les trente sélections précédentes restent disponibles avec une seule entrée du n°28.')
    html=base.PAGE.read_text(encoding='utf-8');start=html.index('<section id="monstres-vertical">');end=html.index('</section>',start)
    section=html[start:end].replace('deux appuis alternés sur 800 ms','un cycle de 800 ms adapté aux appuis de chaque morphologie')
    section=section.replace('Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.','N°28 : vérifier la détente des épaules, les coudes abaissés, les avant-bras verticaux et les griffes ouvertes. Les jambes gardent leur pas court ; le frontal d’origine revient à l’arrêt. Les n°29 et 30 sont validés.')
    html=html[:start]+section+html[end:]
    base.PAGE.write_text(label_comparison_columns(html),encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args();main(args.page_only)
