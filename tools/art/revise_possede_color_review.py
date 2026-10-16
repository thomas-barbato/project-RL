"""Update review46's unapproved proposal to the clarified whole-body colour."""
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=ROOT/'tools/art/prepare_agent_skill_animation_possede_color.py'
s=p.read_text(encoding='utf-8')
replacements=[
    ('TR64-BODY03','TR64-BODY04'),
    ('Recoloration proposée · même pose, rouge et ivoire conservés','Recoloration proposée · corps chair, os ivoire, yeux rouges'),
    ('remplacement des parties violettes par une teinte chair · rouge et ivoire conservés','torse, bras et jambes couleur chair · os ivoire et yeux rouges conservés'),
    ('Le premier essai gris a été écarté. Cette variante propose une chair beige et brune sur les parties précédemment violettes, avec la pose et la silhouette du Possédé.',
     'Les essais gris et chair limitée aux jambes ont été écartés. Cette variante propose la même chair beige et brune sur le torse, les bras et les jambes, avec la pose et la silhouette du Possédé.'),
    ('imagegen-possede-chair-prompt.txt','imagegen-possede-whole-body-chair-prompt.txt'),
    ("'palette':'rouge, ivoire, chair beige/brune'","'palette':'chair beige/brune sur tout le corps, os ivoire, yeux rouges'"),
    ("'user_request':'Non met de la couleur chaire sur son corps pour voir'","'user_request':'non mais sur les 2 parties de son corps :)'"),
    ('Only the formerly violet regions recoloured.','Whole upper and lower body flesh recoloured. Ivory bones and red eyes retained.'),
    ('violet_pixels_in_lower_half','violet_pixels_in_body'),
]
for before,after in replacements:
    assert before in s,before
    s=s.replace(before,after)
p.write_text(s,encoding='utf-8')
p=ROOT/'src/agent_skill_lab_review/fire_painted/possede_color.rs'
p.write_text(p.read_text(encoding='utf-8').replace('TR64-BODY03','TR64-BODY04'),encoding='utf-8')
