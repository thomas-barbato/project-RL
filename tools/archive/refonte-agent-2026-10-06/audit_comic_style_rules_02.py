"""Record feedback and audit the existing comic trial without changing game art."""
from pathlib import Path
import hashlib, json, shutil
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
BASE=Path(__file__).resolve().parent
OUT=BASE/'comic-style-review-02'
SOURCES={
    1:'exec-15ca4191-c330-4e5e-abed-34cc421479f4.png',
    2:'exec-a3b3a08c-4ca0-4b1e-98cd-be196b31b340.png',
    3:'exec-52b39e6f-ba60-4b11-95ac-582b0f0e42f7.png',
    4:'exec-8c51185d-7df3-4e94-8de1-414d5cb20845.png',
    5:'exec-0b4af9ca-a38e-47dc-91e6-b83a60d3286d.png',
}
GENERATED=Path('C:/Users/User/.codex/generated_images/01a11010-433f-7fc2-ac29-ffb696dec5ea')

def main():
    OUT.mkdir(exist_ok=True);(OUT/'sources').mkdir(exist_ok=True)
    for n,name in SOURCES.items():
        target=OUT/'sources'/f'BD-C0{n}.png'
        if not target.exists():shutil.copy2(GENERATED/name,target)
    rig_path=ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json'
    rig=json.loads(rig_path.read_text(encoding='utf-8'))
    mismatches=[r['path'] for r in rig['files'] if hashlib.sha256((ROOT/r['path']).read_bytes()).hexdigest()!=r['binary_sha256']]
    assert not mismatches,mismatches
    measurements=[]
    for n in range(1,6):
        current=Image.open(BASE/'comic-style-review-01'/f'reference-C{n}-native64.png').convert('RGBA')
        trial=Image.open(BASE/'comic-style-review-01'/f'BD-C0{n}-tile32.png').convert('RGBA')
        count=lambda im:sum(a>=128 for a in im.getchannel('A').tobytes())
        measurements.append({'review_id':f'BD-C0{n}','approved_equipped_opaque_pixels':count(current),
            'first_comic_trial_opaque_pixels':count(trial),
            'ratio':round(count(trial)/count(current),3),
            'measurement_scope':'complete_equipped_frame;not_a_body_only_anatomical_measurement'})
    payload={
        'date':'2026-10-10','review_id':'BD-STYLE01-RULES02',
        'style_quality':'accepted_by_user;does_not_accept_new_anatomy_equipment_geometry_or_whole_game_migration',
        'user_feedback':[
            'en terme de qualité ca me va, mais en terme de aille sur 32 px, les dessins sont encore plus petits qu’avant',
            'tu peux meme les rendre un peu plus grand , enfin tu dois profiter des 32 px :) et n’oublie pas que l’affichage des armes peut déborder :)',
            'est-ce que tu respectes les regles mises en places pour la création de personnage et le placement de l’équipement ? car la seule chose qui doit changer par rapport à ca c’est le style de dessin',
        ],
        'invariants':{
            'logical_tile':[32,32],'use_body_tile_fully':True,'equipment_overflow_allowed':True,
            'never_scale_body_to_whole_equipment_bounds':True,
            'fixed_body_between_equipment_configurations':True,'separate_equipment_arm_hand_layers':True,
            'reuse_approved_hand_elbow_and_two_handed_grips':True,
            'weapon_axis_follows_forearm':True,'shield_center_on_hand_covers_hand_and_forearm':True,
            'armor_and_weapon_palettes_independent':True,
            'content_ids_and_gameplay_unchanged':True,
        },
        'current_trial_audit':{
            'revision_01':'complete_equipped_illustrations;not_a_validated_modular_layer_bank',
            'revision_02_sources':'provisional_design_studies;changed_proportions;set_aside_after_user_clarification',
            'technical_conformance':'not_certified;do_not_integrate_or_represent_as_approved_rig_replacement',
            'native_coverage_measurements':measurements,
        },
        'runtime_art_changes':0,'active_rig_files_verified':len(rig['files']),
        'next_review_basis':'approved_body_and_attachment_coordinates;drawing_style_only;numbered_native_32px_comparisons',
    }
    (OUT/'rules-and-feedback-02.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    doc=ROOT/'docs/REVUE_TEXTES_EFFETS_AGENT.md'
    marker='### BD-STYLE01-RULES02 — qualité acceptée, gabarit et prises conservés'
    text=doc.read_text(encoding='utf-8')
    if marker not in text:
        doc.write_text(text+'\n\n'+marker+'\n\n'
            'Le joueur accepte la qualité du dessin BD, mais demande des personnages plus présents dans les 32 px et rappelle que les armes peuvent déborder. '
            'Sa clarification confirme que les règles validées de création et de placement de l’équipement restent applicables : seul le style de dessin change. '
            'L’échelle ne doit jamais dépendre de l’encombrement des armes. Les corps fixes, couches séparées, prises et orientations validées restent la référence.\n\n'
            'Les illustrations complètes de la première planche ne constituent pas une banque modulaire vérifiée. Les nouvelles études ayant modifié les proportions sont conservées comme essais, sans intégration ni validation de leurs géométries. '
            'Le souhait de différencier casques et tenues est noté ; aucune nouvelle identité concrète n’est acceptée par cet accord sur la qualité. '
            'Le prochain rendu doit être présenté à taille native avec contrôle des couches et des points d’attache.\n\n'
            'Trace et audit : [BD-STYLE01-RULES02](../artifacts/refonte-agent-2026-10-06/comic-style-review-02/rules-and-feedback-02.json).\n',encoding='utf-8',newline='\n')
    print(json.dumps({'active_rig_files_verified':len(rig['files']),'runtime_art_changes':0,'measurements':measurements}))

if __name__=='__main__':main()
