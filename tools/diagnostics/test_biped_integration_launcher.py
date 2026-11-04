"""Exercise both real launchers from another cwd and preserve user data."""
import hashlib
import json
import os
import subprocess
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/biped-integration-01'


def main():
    package=ROOT/'dist/Project-RL'
    results=[]
    for name,launcher,data in [('checkout',ROOT/'Jouer.cmd',package),('portable',package/'Jouer.cmd',package),('missing-data',ROOT/'Jouer.cmd',OUT/'intentionally-missing-data')]:
        env=dict(os.environ,PROJECT_RL_DATA_DIR=str(data))
        result=subprocess.run([os.environ.get('COMSPEC','cmd.exe'),'/d','/c',str(launcher),'--verify-assets'],cwd=OUT,env=env,capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=120)
        (OUT/f'launcher-{name}.log').write_text(result.stdout+result.stderr,encoding='utf-8')
        if name=='missing-data':assert result.returncode!=0 and '[ASSETS]' in result.stderr
        else:assert result.returncode==0 and 'Verified ' in result.stdout,(name,result.stdout,result.stderr)
        results.append({'case':name,'launcher':str(launcher),'exit_code':result.returncode,'argument':'--verify-assets'})
    folder=Path(os.environ['APPDATA'])/'ProjectRL'
    before=json.loads((OUT/'user-data-before.json').read_text(encoding='utf-8'))
    after={p.relative_to(folder).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in folder.rglob('*') if p.is_file()}
    changed=sorted(k for k in set(before)|set(after) if before.get(k)!=after.get(k))
    cadence=json.loads((OUT/'cadence-setting.json').read_text(encoding='utf-8'))
    assert changed==(['graphics.json'] if cadence['previous_walk_duration_ms']!=400 else [])
    assert json.loads((folder/'graphics.json').read_text(encoding='utf-8'))['walk_duration_ms']==400
    report={'status':'passed','actual_launchers':results,'cwd':str(OUT),'saved_runs_unchanged':True,'changed_user_files':changed,'authorized_preference_change':cadence,'release_sha256':hashlib.sha256((ROOT/'target/release/project-rl.exe').read_bytes()).hexdigest()}
    (OUT/'launcher-validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report))


if __name__=='__main__':main()
