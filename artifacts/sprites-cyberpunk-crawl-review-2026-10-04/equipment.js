// One explicitly mapped candidate sprite per physical base ID, without affixes.
const equipmentModels = [...window.equipmentCatalogData.models, ...window.pickupCatalogData.models];
const equipmentTiles = new Map();
const equipmentLabels = { firearms: 'Arme à distance', melee: 'Arme de mêlée', body: 'Protection du corps', head: 'Protection de la tête', hands: 'Protection des mains', feet: 'Protection des pieds', consumable: 'Soins', material: 'Matériel et ressources' };
let chosenEquipment = null;
function framedSupplySprite(img) {
  const initial = raster(img, 0, 0, 1, 1);
  const alpha = initial.getContext('2d').getImageData(0, 0, 32, 32).data;
  let left = 32, top = 32, right = -1, bottom = -1;
  for (let y = 0; y < 32; y++) for (let x = 0; x < 32; x++) {
    if (alpha[(y * 32 + x) * 4 + 3] < 160) continue;
    left = Math.min(left, x); top = Math.min(top, y); right = Math.max(right, x); bottom = Math.max(bottom, y);
  }
  if (right < left) return initial;
  const width = right - left + 1, height = bottom - top + 1;
  const ratio = 26 / Math.max(width, height), dw = Math.round(width * ratio), dh = Math.round(height * ratio);
  const fitted = document.createElement('canvas'); fitted.width = fitted.height = 32;
  const ctx = fitted.getContext('2d'); ctx.imageSmoothingEnabled = false;
  ctx.drawImage(initial, left, top, width, height, Math.round((32 - dw) / 2), Math.round((32 - dh) / 2), dw, dh);
  return fitted;
}
function showEquipment(model) {
  chosenEquipment = model.id;
  drawTo('equipmentlarge', equipmentTiles.get(model.id), 128);
  drawTo('equipmentnative', equipmentTiles.get(model.id), 32);
  $('equipmentname').textContent = model.name;
  $('equipmentkind').textContent = (equipmentLabels[model.category] || equipmentLabels[model.group]) + ' · Modèle de base · Proposition à examiner.';
  document.querySelectorAll('#equipmentcards .card').forEach(c => c.classList.toggle('selected', c.dataset.model === model.id));
  document.dispatchEvent(new CustomEvent('equipment-selected', { detail: model.id }));
}
function renderEquipment() {
  if (!equipmentTiles.size) return;
  const group = $('equipmentfilter').value, term = $('equipmentsearch').value.trim().toLocaleLowerCase('fr');
  const selectedModels = equipmentModels.filter(m => (group === 'all' || m.group === group) && (m.name.toLocaleLowerCase('fr').includes(term) || m.id.includes(term) || (m.search_aliases || []).some(alias => alias.includes(term))));
  $('equipmentcount').textContent = selectedModels.length + ' modèle' + (selectedModels.length > 1 ? 's' : '') + ' affiché' + (selectedModels.length > 1 ? 's' : '') + ' · ' + equipmentModels.length + ' références couvertes dans le catalogue.';
  const grid = $('equipmentcards'); grid.replaceChildren();
  for (const model of selectedModels) {
    const b = document.createElement('button'); b.className = 'card'; b.dataset.model = model.id; b.title = model.id;
    const c = document.createElement('canvas'); drawTo(c, equipmentTiles.get(model.id), 96);
    const title = document.createElement('strong'); title.textContent = model.name;
    const type = document.createElement('small'); type.textContent = equipmentLabels[model.category] || equipmentLabels[model.group];
    b.append(c, title, type); b.addEventListener('click', () => showEquipment(model)); grid.append(b);
  }
  if (selectedModels.length) showEquipment(selectedModels.find(m => m.id === chosenEquipment) || selectedModels[0]);
}
async function loadEquipment() {
  try {
    const files = [...new Set(equipmentModels.map(m => m.atlas))], atlasImages = new Map();
    await Promise.all(files.map(file => new Promise((resolve, reject) => {
      const img = new Image(); img.onload = () => { atlasImages.set(file, img); resolve(); }; img.onerror = () => reject(new Error('Planche introuvable : ' + file)); img.src = file;
    })));
    for (const model of equipmentModels) {
      const col = model.index % model.columns, row = Math.floor(model.index / model.columns);
      const img = atlasImages.get(model.atlas);
      equipmentTiles.set(model.id, model.group === 'supplies' ? framedSupplySprite(img) : raster(img, col, row, model.columns, model.rows));
    }
    renderEquipment(); window.equipmentPreviewReady = true;
    $('equipmentfilter').addEventListener('change', renderEquipment);
    $('equipmentsearch').addEventListener('input', renderEquipment);
    document.dispatchEvent(new Event('equipment-ready'));
  } catch (e) { $('error').style.display = 'block'; $('error').textContent = e.message; }
}
loadEquipment();
