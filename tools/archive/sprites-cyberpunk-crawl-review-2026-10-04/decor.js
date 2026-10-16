// Review-only connected wall contours and image-based floor materials.
const oldScene = scene;
const materials = { floors: [], walls: [] };
let decorReady = false;
const floorNames = ['asphalte', 'béton', 'métal', 'ruines'];
const wallNames = ['béton', 'acier', 'blindage', 'maçonnerie'];
function materialTile(img, col, row, columns, rows) {
  const c = document.createElement('canvas'); c.width = c.height = 32;
  const x = c.getContext('2d'); x.imageSmoothingEnabled = false;
  // The sheet's separator strokes stay outside the sampled texture region.
  const sw = img.width / columns, sh = img.height / rows, inset = 3;
  x.drawImage(img, col * sw + inset, row * sh + inset, sw - inset * 2, sh - inset * 2, 0, 0, 32, 32);
  return c;
}
function isWall(x, y) {
  if (x < 0 || y < 0 || x >= 22 || y >= 14) return false;
  return x === 0 || y === 0 || x === 21 || y === 13 ||
    (x === 10 && y < 8 && y !== 5) || (y === 8 && x < 10 && x !== 6);
}
function texturedWall(ctx, x, y, material) {
  const px = x * tile, py = y * tile, line = Math.max(1, Math.round(tile / 32));
  ctx.drawImage(materials.walls[material], px, py, tile, tile);
  // Shared edges have no contour, so a continuous wall has no per-cell frame.
  const n = !isWall(x, y - 1), e = !isWall(x + 1, y), s = !isWall(x, y + 1), w = !isWall(x - 1, y);
  ctx.fillStyle = '#151c23';
  if (n) ctx.fillRect(px, py, tile, line);
  if (e) ctx.fillRect(px + tile - line, py, line, tile);
  if (s) ctx.fillRect(px, py + tile - line, tile, line);
  if (w) ctx.fillRect(px, py, line, tile);
  ctx.fillStyle = material === 2 ? '#969c9c' : '#9fa8af';
  if (n) ctx.fillRect(px + line, py + line, tile - 2 * line, line);
  if (w) ctx.fillRect(px + line, py + line, line, tile - 2 * line);
  ctx.fillStyle = '#2a3137';
  if (s) ctx.fillRect(px + line, py + tile - 2 * line, tile - 2 * line, line);
  if (e) ctx.fillRect(px + tile - 2 * line, py + line, line, tile - 2 * line);
}
scene = function () {
  if (!decorReady || $('decor').value === 'old') return oldScene();
  const c = $('scene'); c.width = 22 * tile; c.height = 14 * tile;
  const ctx = c.getContext('2d'); ctx.imageSmoothingEnabled = false;
  const family = Number($('floor').value), wallMaterial = Number($('wall').value);
  for (let y = 0; y < 14; y++) for (let x = 0; x < 22; x++) {
    const variation = (x * 17 + y * 23 + (x * y) % 7) % 4;
    ctx.drawImage(materials.floors[family * 4 + variation], x * tile, y * tile, tile, tile);
  }
  ctx.fillStyle = '#060c1329'; ctx.fillRect(0, 0, c.width, c.height);
  for (let y = 0; y < 14; y++) for (let x = 0; x < 22; x++) if (isWall(x, y)) texturedWall(ctx, x, y, wallMaterial);
  ctx.fillStyle = '#3d5558'; ctx.fillRect(7 * tile + 3, tile + 4, tile * 2 - 6, tile - 8);
  ctx.fillStyle = '#17272c'; ctx.fillRect(7 * tile + 5, tile + 6, tile * 2 - 10, tile - 12);
  ctx.fillStyle = '#68bbc5'; ctx.fillRect(7 * tile + 7, tile + 7, Math.max(4, tile / 3), Math.max(2, tile / 12));
  ctx.fillStyle = '#5a4937'; ctx.fillRect(tile + 5, 10 * tile + 5, tile - 10, tile - 10);
  ctx.strokeStyle = '#927b54'; ctx.strokeRect(tile + 7, 10 * tile + 7, tile - 14, tile - 14);
  // Both openings keep one tile of walkable floor, independent of sprite zoom.
  ctx.fillStyle = '#535c62';
  ctx.fillRect(10 * tile + 2, 5 * tile + 2, 3, tile - 4);
  ctx.fillRect(11 * tile - 5, 5 * tile + 2, 3, tile - 4);
  ctx.fillStyle = '#69b7c2'; ctx.fillRect(10 * tile + 2, 5 * tile + 3, 3, 3);
  for (const [x, y, i] of spots) ctx.drawImage(sprites.enemy[i], x * tile, y * tile, tile, tile);
  ctx.strokeStyle = '#6bcbd6'; ctx.lineWidth = 1; ctx.strokeRect(8 * tile + .5, 6 * tile + .5, tile - 1, tile - 1);
  ctx.drawImage(sprites.class[current()], 8 * tile, 6 * tile, tile, tile);
  if (showGrid) {
    ctx.strokeStyle = '#a9c2cc30'; ctx.beginPath();
    for (let x = 0; x <= 22; x++) { ctx.moveTo(x * tile + .5, 0); ctx.lineTo(x * tile + .5, c.height); }
    for (let y = 0; y <= 14; y++) { ctx.moveTo(0, y * tile + .5); ctx.lineTo(c.width, y * tile + .5); }
    ctx.stroke();
  }
  comparison();
  $('mapnote').textContent = 'Cases de ' + tile + ' × ' + tile + ' pixels · Sol : ' + floorNames[family] + ' · Murs : ' + wallNames[wallMaterial] + '. Bordures continues, grille facultative.';
};
for (const id of ['decor', 'floor', 'wall']) $(id).addEventListener('change', () => { if (sprites.class.length) scene(); });
async function loadDecor() {
  try {
    const sources = await Promise.all(['sols-32-review-v1.png', 'murs-32-review-v1.png'].map(src => new Promise((resolve, reject) => {
      const img = new Image(); img.onload = () => resolve(img); img.onerror = () => reject(new Error('Texture introuvable : ' + src)); img.src = src;
    })));
    for (let row = 0; row < 4; row++) for (let col = 0; col < 4; col++) materials.floors.push(materialTile(sources[0], col, row, 4, 4));
    for (let row = 0; row < 2; row++) for (let col = 0; col < 2; col++) materials.walls.push(materialTile(sources[1], col, row, 2, 2));
    decorReady = true;
    if (sprites.class.length) scene();
  } catch (e) { $('error').style.display = 'block'; $('error').textContent = e.message; }
}
loadDecor();
