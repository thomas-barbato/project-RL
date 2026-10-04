// Visual pickup sandbox only: no connection to gameplay, turns or save files.
const pickupById = new Map(equipmentModels.map(model => [model.id, model]));
let pickupSelection = 'core:repair_patch';
let pickupReady = false;
let groundPreview = [];
let inventoryPreview = [];
const sceneWithoutLoot = scene;
const groundIconCache = new Map();
const pickupDisplayTiles = new Map();

function readablePickupIcon(source, id) {
  // These two sprites are designed with broad shading for small-size reading.
  // Preserve their authored colors and volume.
  if (id === 'core:repair_patch' || id === 'core:weapon_matter') return source;
  // Presentation renderer: broad color regions replace small surface textures.
  const compact = document.createElement('canvas'); drawTo(compact, source, 24);
  const ctx = compact.getContext('2d'), pixels = ctx.getImageData(0, 0, 24, 24);
  const palette = [[24, 29, 33], [100, 108, 110], [226, 215, 183], [105, 111, 71], [218, 135, 46], [79, 201, 202], [143, 114, 163]];
  const colors = new Int8Array(24 * 24).fill(-1);
  for (let i = 0; i < colors.length; i++) {
    const [r, g, b, a] = pixels.data.subarray(i * 4, i * 4 + 4);
    if (a < 160) continue;
    const light = .2126 * r + .7152 * g + .0722 * b;
    colors[i] = light < 50 ? 0 :
      g > r * 1.28 && b > r * 1.2 ? 5 :
      r > g * 1.2 && g > b * 1.15 ? 4 :
      r > g * 1.18 && b > g * 1.18 ? 6 :
      g > b * 1.25 && r > b * 1.15 && light < 150 ? 3 :
      light > 155 ? 2 : 1;
  }
  const cleaned = colors.slice();
  for (let y = 1; y < 23; y++) for (let x = 1; x < 23; x++) {
    const index = y * 24 + x, color = colors[index];
    if (color < 0 || color === 0) continue;
    const neighbors = new Uint8Array(palette.length);
    let interior = true;
    for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) {
      if (!dx && !dy) continue;
      const next = colors[(y + dy) * 24 + x + dx];
      if (next < 0) interior = false; else neighbors[next]++;
    }
    const majority = neighbors.indexOf(Math.max(...neighbors));
    if (interior && neighbors[color] <= 1 && neighbors[majority] >= 4) cleaned[index] = majority;
  }
  for (let i = 0; i < cleaned.length; i++) {
    const color = cleaned[i];
    if (color < 0) pixels.data.set([0, 0, 0, 0], i * 4);
    else pixels.data.set([...palette[color], 255], i * 4);
  }
  ctx.putImageData(pixels, 0, 0);
  const output = document.createElement('canvas'); drawTo(output, compact, 32);
  return output;
}

function drawGroundObject(ctx, entry, cellSize, x, y) {
  const icon = pickupDisplayTiles.get(entry.id);
  if (!icon) return;
  const logicalSize = Number($('lootsize').value);
  const key = entry.id + '@' + logicalSize;
  if (!groundIconCache.has(key)) {
    // Fewer logical pixels remove fine item detail before any scene enlargement.
    // Inventory keeps the original 32-pixel sprite.
    const compact = document.createElement('canvas');
    drawTo(compact, icon, logicalSize);
    groundIconCache.set(key, compact);
  }
  const size = Math.round(cellSize * logicalSize / 32);
  const inset = Math.round((cellSize - size) / 2);
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(groundIconCache.get(key), x * cellSize + inset, y * cellSize + inset, size, size);
}

scene = function () {
  sceneWithoutLoot();
  if (!pickupReady) return;
  const ctx = $('scene').getContext('2d');
  const positions = [[7, 6], [6, 5], [8, 7]];
  groundPreview.forEach((entry, i) => {
    const [x, y] = positions[i % positions.length];
    drawGroundObject(ctx, entry, tile, x, y);
  });
  drawPickupGround();
};

function drawPickupGround() {
  const c = $('lootground'), ctx = c.getContext('2d'), t = 32;
  ctx.clearRect(0, 0, c.width, c.height);
  ctx.imageSmoothingEnabled = false;
  const floorIndex = Number($('floor').value);
  for (let y = 0; y < 5; y++) for (let x = 0; x < 7; x++) {
    if (decorReady && $('decor').value === 'new') {
      ctx.drawImage(materials.floors[floorIndex * 4 + (x * 17 + y * 23) % 4], x * t, y * t, t, t);
      ctx.fillStyle = '#060c1329'; ctx.fillRect(x * t, y * t, t, t);
    } else {
      ctx.fillStyle = ['#20272c', '#242a30'][(x + y) % 2];
      ctx.fillRect(x * t, y * t, t, t);
    }
  }
  if (sprites.class.length) ctx.drawImage(sprites.class[current()], t, 2 * t, t, t);
  for (const entry of groundPreview) {
    drawGroundObject(ctx, entry, t, entry.x, entry.y);
    if (entry.id === pickupSelection) {
      ctx.strokeStyle = '#6bcbd6'; ctx.lineWidth = 1;
      ctx.strokeRect(entry.x * t + .5, entry.y * t + .5, t - 1, t - 1);
    }
  }
  $('lootgroundcaption').textContent = 'Objets de ' + $('lootsize').value + ' px dans des cases de 32 px.';
}

function renderPickupInventory() {
  const list = $('lootinventory'); list.replaceChildren();
  if (!inventoryPreview.length) {
    const empty = document.createElement('p'); empty.className = 'lootempty';
    empty.textContent = 'Le sac est vide. Ramasse un objet pour y voir son image.';
    list.append(empty);
  }
  for (const entry of inventoryPreview) {
    const model = pickupById.get(entry.id);
    const row = document.createElement('button'); row.className = 'lootrow';
    row.dataset.model = model.id; row.dataset.quantity = entry.quantity;
    row.setAttribute('aria-pressed', model.id === pickupSelection);
    const icon = document.createElement('canvas'); drawTo(icon, pickupDisplayTiles.get(model.id), 32);
    const label = document.createElement('span'), name = document.createElement('strong'), type = document.createElement('small');
    name.textContent = model.name; type.textContent = equipmentLabels[model.category] || equipmentLabels[model.group];
    label.append(name, type);
    const quantity = document.createElement('span'); quantity.className = 'quantity'; quantity.textContent = '×' + entry.quantity;
    row.append(icon, label, quantity);
    row.addEventListener('click', () => selectPickup(model.id, false));
    list.append(row);
  }
}

function renderPickup() {
  const model = pickupById.get(pickupSelection);
  $('lootmodel').value = model.id;
  const detail = $('lootselected'); detail.replaceChildren();
  const icon = document.createElement('canvas'); drawTo(icon, pickupDisplayTiles.get(model.id), 32);
  const label = document.createElement('div'), name = document.createElement('strong'), info = document.createElement('p');
  name.textContent = model.name; info.className = 'caption';
  info.textContent = groundPreview.some(entry => entry.id === model.id) ? 'Au sol · prêt à ramasser' : 'Dans le sac';
  label.append(name, info); detail.append(icon, label);
  $('lootpick').disabled = !groundPreview.some(entry => entry.id === model.id);
  $('lootdrop').disabled = !inventoryPreview.some(entry => entry.id === model.id);
  renderPickupInventory();
  drawPickupGround();
  if (sprites.class.length) scene();
}

function selectPickup(id, stage = true) {
  if (!pickupReady || !pickupById.has(id)) return;
  pickupSelection = id;
  if (stage) {
    groundPreview = groundPreview.filter(entry => entry.id !== id && !(entry.x === 3 && entry.y === 2));
    groundPreview.unshift({ id, quantity: 1, x: 3, y: 2 });
    $('lootstatus').textContent = 'Objet placé au sol pour examiner son dessin.';
  } else {
    $('lootstatus').textContent = groundPreview.some(entry => entry.id === id) ? 'Objet sélectionné au sol.' : 'Objet sélectionné dans le sac.';
  }
  renderPickup();
}

function resetPickup() {
  pickupSelection = 'core:repair_patch';
  groundPreview = [
    { id: pickupSelection, quantity: 1, x: 3, y: 2 },
    { id: 'core:weapon_matter', quantity: 6, x: 5, y: 1 },
    { id: 'core:salvaged_component', quantity: 1, x: 2, y: 3 }
  ];
  const firearm = equipmentModels.find(model => model.group === 'firearms');
  const armor = pickupById.get('core:plastron_bastion') || equipmentModels.find(model => model.group === 'armor');
  inventoryPreview = [
    { id: 'core:repair_patch', quantity: 1 },
    { id: 'core:weapon_matter', quantity: 12 },
    { id: firearm.id, quantity: 1 },
    { id: armor.id, quantity: 1 },
    { id: 'core:charged_battery', quantity: 1 }
  ];
  $('lootstatus').textContent = 'Choisis un objet, puis essaie de le ramasser.';
  renderPickup();
}

function pickUpPreview() {
  const index = groundPreview.findIndex(entry => entry.id === pickupSelection);
  if (index < 0) return;
  const [entry] = groundPreview.splice(index, 1), model = pickupById.get(entry.id);
  const max = model.maximum_stack || 1;
  let remaining = entry.quantity;
  for (const stack of inventoryPreview.filter(stack => stack.id === entry.id)) {
    const amount = Math.min(remaining, max - stack.quantity);
    stack.quantity += amount; remaining -= amount;
  }
  while (remaining > 0) {
    const amount = Math.min(remaining, max);
    inventoryPreview.push({ id: entry.id, quantity: amount }); remaining -= amount;
  }
  $('lootstatus').textContent = 'Dans le sac : ' + model.name + '.';
  renderPickup();
}

function dropPreview() {
  const index = inventoryPreview.findIndex(entry => entry.id === pickupSelection);
  if (index < 0) return;
  const entry = inventoryPreview[index];
  entry.quantity -= 1;
  if (!entry.quantity) inventoryPreview.splice(index, 1);
  const existing = groundPreview.find(entry => entry.id === pickupSelection);
  if (existing) existing.quantity += 1;
  else {
    // This sandbox has one focus cell; replace its previous demonstration item.
    groundPreview = groundPreview.filter(entry => !(entry.x === 3 && entry.y === 2));
    groundPreview.unshift({ id: pickupSelection, quantity: 1, x: 3, y: 2 });
  }
  $('lootstatus').textContent = 'Au sol : ' + pickupById.get(pickupSelection).name + '.';
  renderPickup();
}

function initializePickup() {
  if (pickupReady) return;
  for (const [id, sprite] of equipmentTiles) pickupDisplayTiles.set(id, readablePickupIcon(sprite, id));
  pickupReady = true;
  const select = $('lootmodel'); select.replaceChildren();
  for (const [group, label] of [['supplies', 'Soins, munitions et matériel'], ['armor', 'Armures et protections'], ['firearms', 'Armes à distance'], ['melee', 'Armes de mêlée']]) {
    const section = document.createElement('optgroup'); section.label = label;
    for (const model of equipmentModels.filter(model => model.group === group)) {
      const option = document.createElement('option'); option.value = model.id; option.textContent = model.name;
      section.append(option);
    }
    select.append(section);
  }
  select.disabled = false; $('lootreset').disabled = false;
  select.addEventListener('change', () => selectPickup(select.value));
  $('lootsize').addEventListener('change', renderPickup);
  $('lootpick').addEventListener('click', pickUpPreview);
  $('lootdrop').addEventListener('click', dropPreview);
  $('lootreset').addEventListener('click', resetPickup);
  $('lootground').addEventListener('click', event => {
    const rect = $('lootground').getBoundingClientRect();
    const x = Math.floor((event.clientX - rect.left) * $('lootground').width / rect.width / 32);
    const y = Math.floor((event.clientY - rect.top) * $('lootground').height / rect.height / 32);
    const entry = groundPreview.find(entry => entry.x === x && entry.y === y);
    if (entry) selectPickup(entry.id, false);
  });
  resetPickup();
}
document.addEventListener('equipment-selected', event => selectPickup(event.detail));
document.addEventListener('equipment-ready', initializePickup);
if (window.equipmentPreviewReady) initializePickup();
