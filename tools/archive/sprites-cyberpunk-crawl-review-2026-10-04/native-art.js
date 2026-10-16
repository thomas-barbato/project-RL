// Every coordinate is a pixel on the final 32x32 grid.
// No source illustration, resampling or color reduction is used to build these.
(function (root) {
  function drawing(size) {
    const pixels = Array.from({ length: size }, () => Array(size).fill(null));
    function dot(x, y, color) {
      if (x >= 0 && y >= 0 && x < size && y < size) pixels[y][x] = color;
    }
    function rect(x, y, width, height, color) {
      for (let j = y; j < y + height; j++) for (let i = x; i < x + width; i++) dot(i, j, color);
    }
    function line(x, y, colors) { colors.forEach((color, i) => dot(x + i, y, color)); }
    function poly(points, color) {
      for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
        let inside = false;
        for (let i = 0, j = points.length - 1; i < points.length; j = i++) {
          const [ax, ay] = points[i], [bx, by] = points[j];
          if ((ay > y + .5) !== (by > y + .5) && x + .5 < (bx - ax) * (y + .5 - ay) / (by - ay) + ax) inside = !inside;
        }
        if (inside) dot(x, y, color);
      }
    }
    function finish(name, kind = 'item') {
      const colors = [null];
      for (const row of pixels) for (const color of row) if (!colors.includes(color)) colors.push(color);
      return { name, kind, width: size, height: size, colors, pixels: pixels.map(row => row.map(color => colors.indexOf(color))) };
    }
    return { dot, rect, line, poly, finish };
  }
  const steel = { outline: '#243b49', dark: '#435f70', mid: '#829ba5', light: '#d9ddc9', shine: '#f2ead0' };
  function medicalBox() {
    const d = drawing(32);
    // Flat rectangular lid viewed from above; no sloping faces.
    d.rect(4,7,24,18,'#84302e');
    d.rect(5,8,22,16,'#cf5248');
    d.rect(5,8,22,1,'#f08d6b'); d.rect(5,9,1,14,'#e9765a');
    d.rect(6,23,21,1,'#a93e38');
    d.rect(14,10,4,12,'#f4e6c6'); d.rect(10,14,12,4,'#f4e6c6');
    return d.finish('Boîte de soins');
  }
  function ammoCartridge() {
    const d = drawing(32);
    // One broad cartridge in side profile: pointed projectile and brass case.
    d.poly([[16,3],[19,7],[21,11],[21,13],[22,13],[22,27],[23,27],[23,29],[9,29],[9,27],[10,27],[10,13],[11,13],[11,11],[13,7]],'#745642');
    d.poly([[16,4],[19,9],[20,12],[12,12],[13,9]],'#bf8a63');
    d.poly([[16,5],[16,11],[13,11],[14,8]],'#e4ba8c');
    d.rect(11,13,10,14,'#c4a659');
    d.rect(12,14,2,12,'#ead18a'); d.rect(19,14,2,12,'#ab8b4a');
    d.rect(11,13,10,1,'#efce83');
    d.rect(10,27,12,1,'#e0bf72'); d.rect(10,28,12,1,'#ad8849');
    return d.finish('Munitions');
  }
  function rifle() {
    const d = drawing(32), edge = '#495c62', bronze = '#b3a17d', steel = '#d5dac7';
    // Broad horizontal receiver, stock, grip and magazine match the reference.
    d.poly([[2,11],[7,11],[9,13],[9,16],[5,18],[2,18]],edge);
    d.poly([[3,12],[6,12],[8,14],[5,16],[3,17]],bronze); d.rect(3,12,3,1,'#e2cf9e');
    d.rect(8,9,15,8,edge); d.rect(9,10,13,6,'#899f9f');
    d.rect(9,10,12,2,steel); d.rect(12,8,7,2,edge); d.rect(13,8,5,1,steel);
    d.rect(14,13,5,2,'#6fc9bf'); d.rect(9,15,4,1,'#6b8085');
    d.rect(22,11,7,4,edge); d.rect(23,11,6,1,steel); d.rect(27,10,2,6,edge);
    d.poly([[10,16],[14,16],[13,23],[9,24],[9,21]],edge);
    d.rect(10,18,2,4,bronze); d.dot(10,18,'#e2cf9e');
    d.poly([[17,16],[22,16],[23,22],[21,25],[17,24]],edge);
    d.poly([[18,17],[21,17],[22,22],[20,24],[18,23]],bronze); d.rect(18,17,3,1,'#e2cf9e');
    return d.finish('Brenek A-80');
  }
  function armor() {
    const d = drawing(32), s = steel;
    // Front-facing flat silhouette with matching shoulder pads.
    d.poly([[8,5],[13,5],[13,4],[19,4],[19,5],[24,5],[29,10],[27,16],[24,14],[24,28],[8,28],[8,14],[5,16],[3,10]],s.dark);
    d.rect(13,5,6,3,null);
    d.poly([[5,9],[8,6],[12,6],[11,12],[5,14]],s.light);
    d.poly([[20,6],[24,6],[27,9],[27,14],[21,12]],s.light);
    d.rect(8,6,3,1,s.shine); d.rect(21,6,3,1,s.shine);
    d.poly([[11,10],[14,11],[18,11],[21,10],[23,21],[9,21]],s.mid);
    d.rect(11,12,10,7,s.light); d.rect(12,12,8,1,s.shine);
    d.rect(9,21,6,3,s.dark); d.rect(17,21,6,3,s.dark);
    d.rect(9,25,6,2,s.mid); d.rect(17,25,6,2,s.mid);
    d.rect(11,19,2,1,'#d5a14c'); d.rect(19,19,2,1,'#d5a14c');
    return d.finish('Plastron Bastion');
  }
  function hero(gear = 1) {
    const d = drawing(32), s = steel;
    // Head: helmet, face and broad visor are readable without surface texture.
    d.poly([[13,2],[19,2],[22,6],[21,11],[17,13],[12,11],[10,7]],s.outline);
    d.poly([[13,3],[18,3],[20,6],[18,10],[12,9],[11,6]],s.light);
    d.poly([[19,4],[21,6],[20,10],[18,11],[18,6]],s.mid);
    d.rect(13,4,4,1,s.shine);
    d.rect(12,7,7,2,'#365764');
    d.rect(13,7,5,1,'#68d1cc');
    d.rect(14,10,3,2,'#c7a387');
    d.dot(17,10,'#96715c');
    // Body and arms, built on the same grid as the helmet.
    d.poly([[10,11],[20,11],[24,15],[24,22],[19,24],[11,24],[6,21],[6,15]],s.outline);
    d.rect(7,14,4,4,gear ? s.mid : '#576d78');
    d.rect(8,12,4,4,gear ? s.light : '#80929a');
    d.rect(8,12,3,1,gear ? s.shine : '#a6b4b6');
    d.rect(21,14,3,5,s.dark);
    d.rect(20,12,3,4,gear ? s.mid : '#576d78');
    d.rect(20,12,2,1,gear ? s.light : '#80929a');
    d.rect(11,13,9,7,gear ? s.mid : '#536874');
    if (gear) {
      d.rect(12,14,7,5,s.light); d.rect(13,14,5,1,s.shine);
      d.rect(15,15,1,4,s.mid); d.rect(12,19,3,2,s.dark); d.rect(16,19,4,2,s.dark);
    }
    d.rect(7,18,3,4,'#3f5665'); d.rect(7,21,2,2,'#a8997c');
    d.rect(22,18,3,4,'#435f70'); d.rect(22,21,2,2,'#a8997c');
    d.rect(11,21,9,2,'#526373'); d.rect(15,21,2,1,'#d7a15c');
    // Separated legs and broad boot caps keep the stance clear at 32 pixels.
    d.rect(11,23,4,6,'#526573'); d.rect(12,24,2,4,'#80929a');
    d.rect(17,23,4,6,'#364f60'); d.rect(17,24,2,3,'#647f8b');
    d.rect(9,29,6,2,s.outline); d.rect(9,29,5,1,s.mid); d.dot(9,29,s.light);
    d.rect(17,29,6,2,s.outline); d.rect(17,29,5,1,'#69838e');
    if (gear === 1) {
      d.poly([[23,21],[27,15],[29,14],[28,18],[24,23]],s.dark);
      d.line(26,16,[s.light,s.shine]); d.line(25,18,[s.light,s.mid]);
      d.line(24,20,[s.light,s.mid]); d.rect(22,22,3,1,'#a5b17e'); d.dot(23,23,'#92734c');
    }
    if (gear === 2) {
      d.poly([[16,21],[22,16],[28,15],[29,17],[24,20],[20,23]],s.outline);
      d.rect(23,16,5,1,s.light); d.rect(20,18,5,2,s.mid); d.rect(18,20,3,2,'#83925e');
      d.rect(21,20,2,3,s.dark); d.dot(25,18,'#e3ad5c');
    }
    return d.finish('BRÈCHE · essai natif', 'actor');
  }
  function floor(family, variant) {
    const palettes = [
      ['#28343d','#223039','#34444d','#42545c'],
      ['#3e4c51','#334247','#526167','#677277'],
      ['#344951','#2b3e47','#4e6670','#75848a'],
      ['#3d4544','#2e3738','#5b645c','#777e66']
    ];
    const [base,dark,mid,light] = palettes[family], d = drawing(32);
    d.rect(0,0,32,32,base);
    const offset = variant * 3;
    d.rect(4 + offset % 7,5,4,1,mid); d.rect(5 + offset % 7,6,2,1,dark);
    d.rect(18,18 + variant,5,1,dark); d.rect(19,19 + variant,3,1,mid);
    d.rect(8,25 - variant,3,1,mid);
    if (family === 2) { d.rect(0,10,32,1,dark); d.rect(0,11,32,1,mid); d.rect(26,13,1,5,light); }
    if (family === 1) { d.rect(26,4 + variant,2,3,mid); d.dot(27,4 + variant,light); }
    if (family === 3) { d.rect(2,16,3,2,mid); d.rect(3,17,4,1,light); }
    return d.finish('Sol natif ' + family + '-' + variant,'floor');
  }
  function wall() {
    const d = drawing(32);
    d.rect(0,0,32,32,'#293d4b'); d.rect(1,1,30,3,'#a4b6b7');
    d.rect(1,4,30,3,'#738d9b'); d.rect(2,7,28,21,'#536f81');
    d.rect(2,8,1,19,'#7a929e'); d.rect(5,13,23,1,'#405f72');
    d.rect(6,14,22,1,'#6b8493'); d.rect(28,8,2,21,'#344f61');
    d.rect(3,28,27,2,'#253e51');
    return d.finish('Mur natif','wall');
  }
  const items = {
    'core:repair_patch': medicalBox(),
    'core:weapon_matter': ammoCartridge(),
    'core:modele_brenek_a80': rifle(),
    'core:plastron_bastion': armor()
  };
  const data = { version: 2, creation: 'authored_on_native_pixel_grids', items,
    floors: Array.from({length:4},(_,family)=>Array.from({length:4},(_,variant)=>floor(family,variant))), wall: wall() };
  if (typeof module === 'object' && module.exports) module.exports = data;
  else root.nativeReviewArt = data;
})(typeof window !== 'undefined' ? window : globalThis);
