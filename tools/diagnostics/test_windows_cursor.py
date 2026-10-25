"""Build native Win32 tests from the actual vendored cursor repair and tests.

The standalone harness avoids downloading miniquad's unrelated dev-dependencies.
It changes only its own test thread's cursor counter, restoring it on scope exit.
"""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/cursor-visibility-01'
SOURCE = ROOT / 'vendor/miniquad/src/native/windows.rs'


def main():
    source = SOURCE.read_text(encoding='utf-8')
    start = source.index('// ShowCursor is a counter, not a boolean.')
    end = source.index('// IME composition form constants', start)
    production_and_tests = source[start:end]
    bindings = '''
#![allow(non_snake_case, clippy::upper_case_acronyms)]
use std::ffi::c_void;
type DWORD = u32;
const TRUE: i32 = 1;
const CURSOR_SHOWING: DWORD = 1;
const CURSOR_SUPPRESSED: DWORD = 2;
const NULL: *mut c_void = std::ptr::null_mut();
const IDC_ARROW: *const u16 = 32512usize as *const u16;
const IDC_HAND: *const u16 = 32649usize as *const u16;
const IDC_CROSS: *const u16 = 32515usize as *const u16;
#[link(name="user32")]
extern "system" {
    fn ShowCursor(show: i32) -> i32;
    fn LoadCursorW(instance: *mut c_void, name: *const u16) -> *mut c_void;
}
'''
    OUT.mkdir(parents=True, exist_ok=True)
    harness = OUT / 'cursor-native-tests.rs'
    executable = OUT / 'cursor-native-tests.exe'
    harness.write_text(bindings+production_and_tests, encoding='utf-8')
    subprocess.run(['rustc', '--edition=2018', '--test', str(harness), '-o', str(executable)], check=True)
    result = subprocess.run([str(executable), '--test-threads=1'], capture_output=True, text=True)
    (OUT / 'native-tests.log').write_text(result.stdout+result.stderr, encoding='utf-8')
    print(result.stdout+result.stderr)
    result.check_returncode()
    (OUT / 'native-tests.json').write_text(json.dumps({
        'status': 'passed', 'native_tests': 3,
        'production_source': str(SOURCE.relative_to(ROOT)),
        'production_function_and_tests_sha256': hashlib.sha256(production_and_tests.encode()).hexdigest(),
        'backend_sha256': hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
        'method': 'Actual production function and native Win32 tests extracted verbatim; real user32 calls',
        'scope': 'test-thread visibility counter restored; no focus, input or system-setting changes'
    }, indent=2)+'\n', encoding='utf-8')


if __name__ == '__main__':
    main()
