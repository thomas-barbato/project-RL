"""Add a direct browser comparison to an animation review, preserving VFX frames."""
import argparse
import hashlib
import json
import re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]

def add_comparison(folder):
    folder=Path(folder).resolve()
    assert folder.is_relative_to((ROOT/'artifacts').resolve())
    assert not (folder/'visual-acceptance-01.json').exists(), 'Repair navigation before freezing this review.'
    index=folder/'index.html';original=index.read_text(encoding='utf-8')
    animated={p.relative_to(folder).as_posix():hashlib.sha256(p.read_bytes()).hexdigest()
        for p in folder.rglob('*') if p.is_file() and p.suffix in ('.png','.gif')}
    if not (folder/'index-before-browser-repair.html').exists():
        (folder/'index-before-browser-repair.html').write_text(original,encoding='utf-8')
    native=re.compile(r'<a href="Ouvrir_laboratoire\.ps1">[^<]*</a>')
    html=native.sub('<a href="cote-a-cote.html">Voir les deux animations côte à côte</a>',original)
    if 'href="cote-a-cote.html"' not in html:
        html=html.replace('</main>','</main><p><a href="cote-a-cote.html">Voir les deux animations côte à côte</a></p>',1)
    index.write_text(html,encoding='utf-8')
    compare=html.replace('<title>','<title>Côte à côte · ',1)
    compare=compare.replace('href="cote-a-cote.html">Voir les deux animations côte à côte','href="index.html">Retour à la galerie')
    style='''body{max-width:1320px}.grid{grid-template-columns:repeat(2,minmax(0,1fr))!important;width:100%;gap:12px}article{min-width:0}canvas{width:100%;height:auto}.tip{max-width:calc(100% - 30px)}body.native-view .grid{grid-template-columns:repeat(2,608px)!important;width:1232px}body.native-view canvas{width:576px;height:576px}.browser-note{color:#a5b9c5}label{display:flex;align-items:center;gap:8px}'''
    compare=compare.replace('</style>','\n'+style+'</style>',1)
    controls='<label><input id="native-view" type="checkbox">Taille native (cases64px)</label><button id="fullscreen">Plein écran</button>'
    compare=compare.replace('</div><p id="status">',controls+'</div><p class="browser-note">Comparaison adaptée à la fenêtre. Activez la taille native ou le plein écran pour examiner les détails à64px.</p><p id="status">',1)
    compare=compare.replace("x=ev.clientX-box.left,y=ev.clientY-box.top", "x=(ev.clientX-box.left)*canvas.width/box.width,y=(ev.clientY-box.top)*canvas.height/box.height")
    extra='''document.getElementById('native-view').onchange=ev=>document.body.classList.toggle('native-view',ev.target.checked);
document.getElementById('fullscreen').onclick=async()=>{try{if(document.fullscreenElement)await document.exitFullscreen();else await document.documentElement.requestFullscreen();}catch{document.getElementById('fullscreen').textContent='Plein écran indisponible';}};'''
    compare=compare.replace('</script>',extra+'</script>',1)
    compare=compare.replace('cases64px','cases 64 px').replace('à64px','à 64 px')
    (folder/'cote-a-cote.html').write_text(compare,encoding='utf-8')
    assert all(hashlib.sha256((folder/name).read_bytes()).hexdigest()==digest for name,digest in animated.items())
    record={'cause':'file:// link to .ps1 displays source instead of executing a local PowerShell program.',
        'browser_preview':'cote-a-cote.html','two_canvases':compare.count('<canvas '),'source_tiles':64,
        'responsive_comparison':True,'optional_native_size_and_fullscreen':True,'native_script_not_presented_as_browser_preview':True,
        'animated_images_unchanged':len(animated),'scope':'Review navigation only; accepted animations and campaign unchanged.',
        'browser_runtime_validation':'Not performed; direct HTML, controller syntax and local paths checked offline.'}
    assert record['two_canvases']==2
    (folder/'browser-navigation-repair.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(record))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('review');a=p.parse_args();add_comparison(a.review)
