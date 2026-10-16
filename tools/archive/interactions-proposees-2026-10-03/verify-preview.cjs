const { chromium } = require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const fs = require('node:fs');
const path = require('node:path');
const { pathToFileURL } = require('node:url');

(async () => {
  const browser = await chromium.launch({
    headless: true,
    executablePath: 'C:/Users/User/AppData/Local/ms-playwright/chromium-1169/chrome-win/chrome.exe'
  });
  const results = [];
  const errors = [];
  try {
    for (const width of [736, 320]) {
      const page = await browser.newPage({ viewport: { width, height: 700 }, colorScheme: 'dark' });
      page.on('pageerror', error => errors.push(error.message));
      await page.goto(pathToFileURL(path.join(__dirname, 'preview.html')).href);
      const frame = page.frameLocator('iframe');
      await frame.locator('[data-drawn]').first().waitFor();
      await frame.getByRole('button', { name: 'Couper la branche', exact: true }).click();
      if (!(await frame.getByRole('status').innerText()).includes('Raccourci fermé · capteur arrêté')) {
        throw new Error('The cut circuit does not update its consequences');
      }
      await page.screenshot({ path: path.join(__dirname, `preview-${width}-coupee.png`), fullPage: true });
      await frame.getByRole('button', { name: 'Alimenter la branche', exact: true }).focus();
      await page.keyboard.press('Enter');
      if (!(await frame.getByRole('status').innerText()).includes('Raccourci ouvert · capteur actif')) {
        throw new Error('Keyboard selection does not restore the powered circuit');
      }
      const geometry = await frame.locator('#rl-circuit-proposal').evaluate(root => {
        const diagram = root.querySelector('svg').getBoundingClientRect();
        const marks = Array.from(root.querySelectorAll('svg text')).map(node => {
          const box = node.getBoundingClientRect();
          return { label: node.textContent, x: box.x, right: box.right, y: box.y, bottom: box.bottom, fontSize: parseFloat(getComputedStyle(node).fontSize) };
        });
        return {
          frameWidth: window.innerWidth,
          scrollWidth: document.documentElement.scrollWidth,
          diagram: { x: diagram.x, right: diagram.right, y: diagram.y, bottom: diagram.bottom },
          marks,
          selected: root.querySelector('[aria-pressed="true"]').textContent
        };
      });
      if (geometry.scrollWidth > geometry.frameWidth) throw new Error(`Horizontal overflow at ${width}`);
      for (const mark of geometry.marks) {
        if (mark.x < geometry.diagram.x - 1 || mark.right > geometry.diagram.right + 1 || mark.fontSize < 11) {
          throw new Error(`Unreadable or clipped label at ${width}: ${mark.label}`);
        }
      }
      for (let i = 0; i < geometry.marks.length; i++) {
        for (let j = i + 1; j < geometry.marks.length; j++) {
          const a = geometry.marks[i];
          const b = geometry.marks[j];
          if (a.x < b.right && a.right > b.x && a.y < b.bottom && a.bottom > b.y) {
            throw new Error(`Overlapping labels at ${width}: ${a.label} / ${b.label}`);
          }
        }
      }
      await page.screenshot({ path: path.join(__dirname, `preview-${width}-active.png`), fullPage: true });
      results.push({ width, circuitSwitch: true, keyboard: true, geometry });
      await page.close();
    }
    if (errors.length) throw new Error(errors.join('\n'));
    fs.writeFileSync(path.join(__dirname, 'preview-validation.json'), JSON.stringify({ results, errors }, null, 2));
    process.stdout.write('Preview verified at 736 and 320 pixels; circuit and keyboard controls work; no overlap, clipping or script error.\n');
  } finally {
    await browser.close();
  }
})().catch(error => { process.stderr.write(error.stack + '\n'); process.exitCode = 1; });
