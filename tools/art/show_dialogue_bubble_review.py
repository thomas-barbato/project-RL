"""Three native client captures for DIALOGUE-BUBBLE-01; no new dialog text."""
from pathlib import Path
import json,re,hashlib
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/handcrafted-places-review-01/dialogue-bubbles-01'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def publish():
    checks=json.loads((OUT/'checks.json').read_text(encoding='utf-8'))
    assert not checks['production_enabled'] and checks['render_is_read_only']
    assert checks['native_cell']==64 and not (OUT/'stderr.log').read_text().strip()
    vx,vy,vw,vh=checks['viewport']
    for i,c in enumerate(checks['candidates'],1):
        assert c['full_existing_text'] and c['speaker_visible']
        im=Image.open(OUT/f'bubble-{i}.png')
        assert im.size==(1600,1000)
        im.crop((round(vx),round(vy),round(vx+vw),round(vy+vh))).save(OUT/f'bubble-{i}-native.png')
    page=OUT.parent/'index.html';text=page.read_text(encoding='utf-8')
    text=re.sub(r'<!-- dialogue-bubbles:start -->.*?<!-- dialogue-bubbles:end -->','',text,flags=re.S)
    section=f'''<!-- dialogue-bubbles:start --><section id="bulles-01" style="border:1px solid #89764f;padding:18px;margin:18px 0;background:#12171e"><h2>DIALOGUE-BUBBLE-01 · trois propositions à comparer</h2><p>Même scène réelle et dialogue complet déjà validé de l’officier, dans les trois versions. Aucun nouveau texte narratif, aucune intégration pour le moment. Le joueur et l’interlocuteur restent visibles. Images natives, cases de 64 px.</p><ol><li><b>01 · Bulle sombre :</b> près de l’interlocuteur, liseré chaud et pointe de dialogue. Lecture directe, présence discrète.</li><li><b>02 · Bulle blindée :</b> cartouche métallique en bas de la scène, texture de l’interface et liaison vers le PNJ. Davantage de largeur pour les longs échanges.</li><li><b>03 · Bulle terminal :</b> vert phosphore, lignes d’écran et angles techniques, près de l’interlocuteur. Variante plus informatique.</li></ol><nav style="display:flex;gap:10px;margin:16px 0"><button data-bubble="1">01 · Sombre</button><button data-bubble="2">02 · Blindée</button><button data-bubble="3">03 · Terminal</button></nav><div style="overflow:auto"><img id="bubble-preview" src="dialogue-bubbles-01/bubble-1-native.png" width="{round(vw)}" height="{round(vh)}" style="max-width:none" alt="Bulle de dialogue dans le client réel, taille native"></div><p><a id="bubble-full" href="dialogue-bubbles-01/bubble-1.png">Voir la capture complète avec l’interface</a></p><p>Le choix porte d’abord sur le rendu. Les réponses de dialogue et les fenêtres de commerce, soins et amélioration seront ensuite adaptées sans supprimer leurs fonctions. Les contours vert/rouge sont proposés plus bas dans cette même page.</p><script>document.querySelectorAll('[data-bubble]').forEach(b=>b.addEventListener('click',()=>{{let n=b.dataset.bubble;document.getElementById('bubble-preview').src='dialogue-bubbles-01/bubble-'+n+'-native.png';document.getElementById('bubble-full').href='dialogue-bubbles-01/bubble-'+n+'.png';document.querySelectorAll('[data-bubble]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));}}));</script></section><!-- dialogue-bubbles:end -->'''
    page.write_text(text.replace('</header>','</header>'+section,1),encoding='utf-8')
    report=dict(review_id='DIALOGUE-BUBBLE-01',date='2026-10-18',status='awaiting_visual_choice',production_enabled=False,checks=checks,captures_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.glob('*.png'))})
    (DOC/'dialogue-bubbles-01.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(gallery=str(page),native_candidates=3)))


if __name__=='__main__':publish()
