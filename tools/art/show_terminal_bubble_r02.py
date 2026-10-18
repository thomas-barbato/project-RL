"""Publish the chosen terminal bubble's revision from native GPU captures."""
from pathlib import Path
import hashlib,json,re
import numpy as np
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/handcrafted-places-review-01/dialogue-bubbles-01'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def publish():
    checks=json.loads((OUT/'checks-r02.json').read_text(encoding='utf-8'))
    assert checks['revision']=='r02' and not checks['production_enabled']
    assert checks['no_close_button'] and checks['full_existing_text']
    assert checks['closures_checked']==['inside','outside','escape']
    assert checks['close_click_consumed'] and checks['render_is_read_only']
    assert not (OUT/'stderr.log').read_text().strip()
    full=Image.open(OUT/'terminal-r02.png').convert('RGB')
    bg=Image.open(OUT/'terminal-r02-background.png').convert('RGB')
    closed=Image.open(OUT/'terminal-r02-closed.png').convert('RGB')
    unread=Image.open(OUT/'terminal-r02-unread.png').convert('RGB')
    ready=Image.open(OUT/'terminal-r02-ready.png').convert('RGB')
    assert checks['notification_hidden_while_open'] and checks['quest_notification_stays_read_after_closing']
    assert checks['quest_state_unchanged']
    # The notification must be absent both while speaking and after closing.
    sx,sy,sw,sh=map(round,checks['speaker'])
    notification_box=(sx,sy-32,sx+sw,sy)
    assert np.array_equal(np.asarray(bg.crop(notification_box)),np.asarray(closed.crop(notification_box)))
    assert not np.array_equal(np.asarray(bg.crop(notification_box)),np.asarray(unread.crop(notification_box)))
    assert not np.array_equal(np.asarray(closed.crop(notification_box)),np.asarray(ready.crop(notification_box)))
    contact_box=(sx-12,sy-42,sx+sw+12,sy+sh+8)
    for label,image in [('unread',unread),('read',closed),('ready',ready)]:
        image.crop(contact_box).save(OUT/f'terminal-r02-notification-{label}.png')
    assert full.size==bg.size==(1600,1000)
    a=np.asarray(full).astype(float);b=np.asarray(bg).astype(float)
    root_x,root_y,_=checks['tail_root']
    fill=np.array(checks['fill_rgba'][:3],dtype=float)
    alpha=checks['fill_rgba'][3]/255
    errors=[]
    for dy in [-1,0,1]:
        for dx in [-6,-2,2,6]:
            x,y=round(root_x+dx),round(root_y+dy)
            expected=b[y,x]*(1-alpha)+fill*alpha
            errors.append(float(np.max(np.abs(a[y,x]-expected))))
    assert max(errors)<=2, dict(tail_root_blend_error=max(errors))
    # A centered 1 px line covers two texels at 50% each under GPU antialiasing.
    # Check that exact coverage near the corner, rather than requiring opaque texels.
    x,y,_,_=map(round,checks['bounds'])
    for row in [y+4,y+8]:
        accent=np.array([154,210,158],dtype=float)
        assert np.max(np.abs(a[row,x-1]-(accent+b[row,x-1])/2))<=2
        assert np.max(np.abs(a[row,x]-(accent+a[row,x+3])/2))<=2
        assert np.array_equal(a[row,x-2],b[row,x-2])
        assert np.array_equal(a[row,x+1],a[row,x+3])
    vx,vy,vw,vh=checks['viewport']
    box=(round(vx),round(vy),round(vx+vw),round(vy+vh))
    full.crop(box).save(OUT/'terminal-r02-native.png')
    bg.crop(box).save(OUT/'terminal-r02-background-native.png')
    closed.crop(box).save(OUT/'terminal-r02-closed-native.png')
    unread.crop(box).save(OUT/'terminal-r02-unread-native.png')
    frame_paths=sorted(OUT.glob('terminal-r02-frame-*.png'))
    assert len(frame_paths)==checks['animation_frames']
    frames=[Image.open(p).convert('RGB') for p in frame_paths]
    ax,ay,aw,ah=map(round,checks['animation_crop'])
    assert all(f.size==(aw,ah) for f in frames)
    assert np.array_equal(np.asarray(frames[-2]),np.asarray(full.crop((ax,ay,ax+aw,ay+ah)))), 'Animated final pose differs from static capture'
    cx,cy,cw,ch=map(round,checks['cursor'])
    cursor_diff=np.any(np.asarray(frames[-2])!=np.asarray(frames[-1]),axis=2)
    expected_cursor=np.zeros(cursor_diff.shape,dtype=bool)
    expected_cursor[cy-ay:cy-ay+ch,cx-ax:cx-ax+cw]=True
    assert np.array_equal(cursor_diff,expected_cursor), 'End cursor differs from its native block'
    durations=[25]*len(frames);durations[-2:]=[500,500]
    animation=OUT/'terminal-r02-writing.webp'
    frames[0].save(animation,save_all=True,append_images=frames[1:],duration=durations,loop=1,lossless=True,method=4)
    frames[len(frames)//2].save(OUT/'terminal-r02-mid.png')
    frames[-2].save(OUT/'terminal-r02-dialogue-static.png')
    # Reproducible intermediate screenshots are no longer needed after packing.
    for p in frame_paths:
        assert p.resolve().parent==OUT.resolve() and p.name.startswith('terminal-r02-frame-')
        p.unlink()
    verification=dict(review='DIALOGUE-BUBBLE-01',revision='r02',tail_joint_no_separator=True,single_alpha_layer=True,maximum_joint_blend_error=max(errors),fill_alpha=alpha,uniform_thin_corners=True,no_close_button=True,closures_without_game_action=checks['closures_checked'],progressive_writing=True,characters_per_second=checks['characters_per_second'],source_animation_frames=len(frames),playback_ms=sum(durations),final_text_matches_static=True,blinking_block_verified=True,quest_notification_stays_read_after_closing=True,quest_state_unchanged=True,ready_to_turn_in_symbol='?',ready_symbol_distinct=True,animation_sha256=hashlib.sha256(animation.read_bytes()).hexdigest())
    (OUT/'verification-r02.json').write_text(json.dumps(verification,indent=2)+'\n',encoding='utf-8')
    page=OUT.parent/'index.html';text=page.read_text(encoding='utf-8')
    text=re.sub(r'<!-- dialogue-bubbles:start -->.*?<!-- dialogue-bubbles:end -->','',text,flags=re.S)
    section=f'''<!-- dialogue-bubbles:start --><section id="bulles-02" style="border:1px solid #89764f;padding:18px;margin:18px 0;background:#12171e"><h2>Bulle terminal · r02 — version corrigée à valider</h2><p>Contour fin continu, sans barre à la base de la pointe ni renforts aux coins. Fond légèrement transparent, texte opaque, écriture progressive et curseur terminal clignotant à la fin. Aucun bouton ni consigne dans la bulle. La notification de quête consultée reste masquée après fermeture ; cela ne termine pas la quête. Une quête prête à rendre affiche un « ? » distinct, proposé ci-dessous. Aperçu natif du client réel, cases de 64 px.</p><button id="bubble-reopen" hidden>Rouvrir l’aperçu</button><div style="overflow:auto"><div id="terminal-bubble-stage" style="position:relative;width:{round(vw)}px;height:{round(vh)}px"><img id="terminal-bubble-background" src="dialogue-bubbles-01/terminal-r02-background-native.png" width="{round(vw)}" height="{round(vh)}" style="max-width:none" alt="Scène réelle du camp"><img id="terminal-bubble-image" width="{aw}" height="{ah}" style="position:absolute;left:{ax-round(vx)}px;top:{ay-round(vy)}px;max-width:none" alt="Écriture progressive du dialogue"><span id="terminal-end-cursor" hidden style="position:absolute;left:{cx-round(vx)}px;top:{cy-round(vy)}px;width:{cw}px;height:{ch}px;background:rgb(154,210,158);animation:terminal-cursor-r02 1s steps(1,end) infinite"></span></div></div><style>#bulles-02 [hidden]{{display:none!important}}@keyframes terminal-cursor-r02{{0%,49%{{visibility:visible}}50%,100%{{visibility:hidden}}}}</style><div class="pair" style="margin-top:18px"><figure><figcaption>! · Nouveauté à consulter</figcaption><img src="dialogue-bubbles-01/terminal-r02-notification-unread.png" alt="Notification encore non consultée"></figure><figure><figcaption>Consultée · quête en cours</figcaption><img src="dialogue-bubbles-01/terminal-r02-notification-read.png" alt="Aucune notification après lecture"></figure><figure><figcaption>? · Quête prête à rendre (proposé)</figcaption><img src="dialogue-bubbles-01/terminal-r02-notification-ready.png" alt="Point d’interrogation pour le retour au PNJ"></figure></div><p><a href="dialogue-bubbles-01/terminal-r02.png">Capture complète, texte terminé</a> · <a href="dialogue-bubbles-01/bubble-3.png">Version précédente</a></p><script>(()=>{{let opened=true,replay=0,timer;const reduced=matchMedia('(prefers-reduced-motion: reduce)').matches;const bg=document.getElementById('terminal-bubble-background'),img=document.getElementById('terminal-bubble-image'),cursor=document.getElementById('terminal-end-cursor'),reopen=document.getElementById('bubble-reopen');img.addEventListener('load',()=>{{clearTimeout(timer);if(opened&&!reduced)timer=setTimeout(()=>{{if(opened)cursor.hidden=false;}},{sum(durations)});}});function paint(){{clearTimeout(timer);cursor.hidden=true;img.hidden=!opened;reopen.hidden=opened;bg.src='dialogue-bubbles-01/terminal-r02-'+(opened?'background':'closed')+'-native.png';if(opened)img.src=reduced?'dialogue-bubbles-01/terminal-r02-dialogue-static.png':'dialogue-bubbles-01/terminal-r02-writing.webp?replay='+(++replay);}}document.addEventListener('pointerdown',e=>{{if(!opened||e.button!==0||e.target.closest('#bubble-reopen,#sons-dialogue-01'))return;opened=false;paint();e.preventDefault();e.stopImmediatePropagation();}},true);document.addEventListener('keydown',e=>{{if(opened&&e.key==='Escape'){{opened=false;paint();e.preventDefault();e.stopImmediatePropagation();}}}},true);reopen.addEventListener('click',e=>{{e.preventDefault();e.stopPropagation();opened=true;paint();}});paint();}})();</script></section><!-- dialogue-bubbles:end -->'''
    page.write_text(text.replace('</header>','</header>'+section,1),encoding='utf-8')
    report=dict(review_id='DIALOGUE-BUBBLE-01',revision='r02',date='2026-10-18',selected_style='03-terminal',status='awaiting_revised_visual_approval',production_enabled=False,checks=checks,gpu_verification=verification,captures_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.glob('terminal-r02*.png'))})
    (DOC/'dialogue-bubbles-02.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(gallery=str(page),revision='r02',joint_verified=True,uniform_corners=True,interactive_dismissal=True)))


if __name__=='__main__':publish()
