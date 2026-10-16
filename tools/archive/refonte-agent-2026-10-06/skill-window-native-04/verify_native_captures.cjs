// Compare each selected native 64px tile with the exact accepted icon.
// Fractional GPU placement uses linear sampling, so the comparison tolerates
// subpixel edge blending and checks nearby integer capture coordinates.
const fs = require('node:fs'), path = require('node:path');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const out = __dirname, root = path.resolve(out, '../../..');
const read = f => JSON.parse(fs.readFileSync(f, 'utf8').replace(/^\uFEFF/, ''));
async function main() {
 const captures = read(path.join(out, 'capture-results.json'));
 const catalogue = read(path.join(root, 'assets/skills/agent/catalogue.json'));
 const review = read(path.join(out, '../skill-icon-review-17/review.json'));
 const results = [];
 for (const c of captures) {
  const meta = await sharp(c.file).metadata();
  if (meta.width !== 960 || meta.height !== 540 || c.process_exit !== 0) throw Error('Capture failed: '+c.skill);
  const skill = catalogue.skills.find(s => s.id === c.skill);
  const entries = catalogue.skills.filter(s => s.branch === skill.branch);
  const selected = entries.findIndex(s => s.id === c.skill);
  const bodyWidth = 904, side = bodyWidth * .18, detailWidth = bodyWidth * .36;
  const branchRight = 28 + side, detailX = 932 - detailWidth;
  const gridX = branchRight + 10, gridWidth = detailX - branchRight - 20;
  const columns = Math.max(1, Math.floor((gridWidth - 12) / 124));
  const stride = (gridWidth - 12) / columns;
  const visibleRows = 2, selectedRow = Math.floor(selected / columns);
  const firstRow = Math.max(0, selectedRow - (visibleRows - 1));
  const x = Math.round(gridX + 6 + (selected % columns)*stride + (stride - 64)*.5);
  const y = 122 + (selectedRow - firstRow)*148;
  const expected = await sharp(path.join(root,'assets/skills/agent',catalogue.art[c.skill].compact)).removeAlpha().raw().toBuffer();
  let best = {mean_absolute_channel_error:Infinity};
  for (let dy=-1;dy<=1;dy++) for(let dx=-1;dx<=1;dx++) {
   const actual = await sharp(c.file).extract({left:x+dx,top:y+dy,width:64,height:64}).removeAlpha().raw().toBuffer();
   let sum=0;for(let i=0;i<expected.length;i++) sum+=Math.abs(actual[i]-expected[i]);
   const error=sum/expected.length;
   if(error<best.mean_absolute_channel_error) best={mean_absolute_channel_error:error,x:x+dx,y:y+dy};
  }
  if(best.mean_absolute_channel_error > 15) throw Error('Unexpected selected icon: '+c.skill+' '+JSON.stringify(best));
  const file=c.skill+'-native-icon.png';
  await sharp(c.file).extract({left:best.x,top:best.y,width:64,height:64}).png().toFile(path.join(out,file));
  results.push({skill:c.skill,variant:c.variant,capture:c.file,dimensions:[960,540],
                native_icon:file,match_error:Number(best.mean_absolute_channel_error.toFixed(3)),
                native_icon_position:[best.x,best.y],accepted_source_match:true});
 }
 const images=[];
 const rows=Math.ceil(results.length/6),height=70+rows*116;
 const pieces=['<rect width="1040" height="'+height+'" fill="#0b1017"/>',
 '<text x="24" y="32" fill="#e1e7ec" font-size="22">Icônes extraites des captures du jeu · 64 px réels</text>'];
 for(let i=0;i<results.length;i++) {
  const c=results[i],candidate=review.candidates.find(v=>v.id===c.skill),x=24+(i%6)*168,y=60+Math.floor(i/6)*116;
  const png=fs.readFileSync(path.join(out,c.native_icon)).toString('base64');
  pieces.push('<image x="'+x+'" y="'+y+'" width="64" height="64" href="data:image/png;base64,'+png+'"/>',
  '<text x="'+x+'" y="'+(y+82)+'" fill="#a6b6c5" font-size="12">'+candidate.number+' · '+c.skill+'</text>');
 }
 const svg='<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="'+height+'"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
 await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-selected-icons.png'));
 fs.writeFileSync(path.join(out,'native-icon-matches.json'),JSON.stringify(results,null,2)+'\n');
 console.log(JSON.stringify({captured:results.length,matches:results.every(c=>c.accepted_source_match),maximum_error:Math.max(...results.map(c=>c.match_error))}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
