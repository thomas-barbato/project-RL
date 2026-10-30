"""Copy a built-in vertical outfit output and preserve prompt/provenance."""
from pathlib import Path
import argparse,hashlib,json,shutil
from PIL import Image

def main():
    p=argparse.ArgumentParser();p.add_argument('id');p.add_argument('source');p.add_argument('--record-only',action='store_true');p.add_argument('--review',choices=['C10','C11','C12','C11-back-r02'],default='C10');a=p.parse_args()
    root=Path(__file__).resolve().parents[2];docroot=root/'docs/reviews/vertical-walk-18';path=docroot/f'prompts-{a.review}.json';doc=json.loads(path.read_text('utf8'))
    job=next(j for j in doc['jobs']+doc.get('edits',[]) if j['id']==a.id)
    source=Path(a.source).resolve();assert source.is_relative_to(Path(r'C:\Users\User\.codex\generated_images').resolve()) and not source.is_symlink()
    target=root/job['output'];target.parent.mkdir(parents=True,exist_ok=True)
    sha=hashlib.sha256(source.read_bytes()).hexdigest()
    if target.exists():assert hashlib.sha256(target.read_bytes()).hexdigest()==sha
    else:shutil.copy2(source,target)
    im=Image.open(target);assert 'A' in im.getbands() and im.getchannel('A').getextrema()[0]==0
    job.update(status='drawn_pending_native_review',source_sha256=sha,selected_sha256=sha)
    if job in doc.get('edits',[]):
        original=next(j for j in doc['jobs'] if j['profile']==job['profile'] and j['family']==job['family']);original.update(selected_output=job['output'],selected_sha256=sha)
    records=docroot/f'generation-{a.review}';records.mkdir(exist_ok=True)
    (records/(a.id+'.json')).write_text(json.dumps({**job,'mode':'builtin_imagegen','generated_source':str(source),'dimensions':im.size,'sha256':sha},ensure_ascii=False,indent=2)+'\n','utf8')
    if not a.record_only:
        pending=path.with_suffix('.next.json')
        pending.write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n','utf8');pending.replace(path)
    print(json.dumps({'id':a.id,'copied':target.relative_to(root).as_posix(),'dimensions':im.size,'sha256':sha}))

if __name__=='__main__':main()
