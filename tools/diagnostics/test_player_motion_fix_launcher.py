"""Check current real launchers without opening or saving a user campaign."""
import hashlib
import json
import os
import subprocess
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/player-motion-fix-01'


def main():
    rows=[]; package=ROOT/'dist/Project-RL'
    for label,launcher,data in [('checkout',ROOT/'Jouer.cmd',package),('portable',package/'Jouer.cmd',package),('missing-data',ROOT/'Jouer.cmd',OUT/'intentionally-missing-data')]:
        env=dict(os.environ,PROJECT_RL_DATA_DIR=str(data))
        r=subprocess.run([os.environ.get('COMSPEC','cmd.exe'),'/d','/c',str(launcher),'--verify-assets'],cwd=OUT,env=env,capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=120)
        (OUT/f'launcher-{label}.log').write_text(r.stdout+r.stderr,encoding='utf-8')
        assert (r.returncode!=0 and '[ASSETS]' in r.stderr) if label=='missing-data' else (r.returncode==0 and 'Verified ' in r.stdout),(label,r.stdout,r.stderr)
        rows.append({'case':label,'exit_code':r.returncode,'args':['--verify-assets']})
    before=json.loads((OUT/'before-launcher-user-data.json').read_text(encoding='utf-8')); after={}
    for p in (Path(os.environ['APPDATA'])/'ProjectRL').rglob('*'):
        if not p.is_file():continue
        try:sha=hashlib.sha256(p.read_bytes()).hexdigest()
        except PermissionError:
            assert p.suffix=='.lock' and p.stat().st_size==0; sha=hashlib.sha256(b'').hexdigest()
        after[p.relative_to(Path(os.environ['APPDATA'])/'ProjectRL').as_posix()]=sha
    assert before==after,'User files changed while verifying the launchers; do not restore automatically'
    result={'status':'passed','actual_launchers':rows,'user_files_unchanged':True,'user_file_hashes_checked':len(before),'graphical_user_campaign_launched':False,'release_sha256':hashlib.sha256((ROOT/'target/release/project-rl.exe').read_bytes()).hexdigest()}
    (OUT/'launcher-validation.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8');print(json.dumps(result))


if __name__=='__main__':main()
