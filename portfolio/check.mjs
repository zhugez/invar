import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import vm from 'node:vm';
import { MODULE_PREFIX, project, card, patchEntry, originalModulePath, binaryResponse } from './integration.mjs';
import base from './base-worker.js';

const fixture = 'var projects=[{id:`pingloom`,layer:`tools`}];function pi(e){return`/showcases/${e.id}.mp4`}function credit(n){const label={};label.textContent=`Project showcase · Music and sound effects`;return label.textContent}';
const patched = patchEntry(fixture);
const evaluated = vm.runInNewContext(patched + ';JSON.stringify({projects,showcase:pi(projects[0]),oldShowcase:pi(projects[1]),credit:credit(projects[0]),oldCredit:credit(projects[1])})');
const result = JSON.parse(evaluated);
assert.equal(result.projects.length, 2);
assert.equal(result.projects[0].id, 'invar');
assert.equal(result.showcase, project.showcase);
assert.equal(result.oldShowcase, '/showcases/pingloom.mp4');
assert.equal(result.credit, project.mediaCredit);
assert.equal(result.oldCredit, 'Project showcase · Music and sound effects');
assert.throws(() => patchEntry('unexpected bundle'), /bundle changed/);
assert.throws(() => patchEntry(fixture + fixture), /bundle changed/);
assert.equal(originalModulePath(MODULE_PREFIX + 'three.core-DrJ08-gy.js'), '/assets/three.core-DrJ08-gy.js');
assert.equal(originalModulePath(MODULE_PREFIX + '../secret.js'), null);
assert.equal(originalModulePath('/assets/index.js'), null);
assert.equal((card.match(/data-project="invar"/g) ?? []).length, 1);
assert.ok(card.includes('loading="lazy"'));

const data = new Uint8Array([0, 1, 2, 3, 4]).buffer;
const request = (range, method = 'GET', more = {}) => new Request('https://example.invalid/showcases/invar-v1.mp4', { method, headers: { ...(range ? { Range: range } : {}), ...more } });
for (const [range, status, expected, contentRange] of [
  [null, 200, [0, 1, 2, 3, 4], null], ['bytes=1-3', 206, [1, 2, 3], 'bytes 1-3/5'],
  ['bytes=3-', 206, [3, 4], 'bytes 3-4/5'], ['bytes=-2', 206, [3, 4], 'bytes 3-4/5'],
  ['bytes=0-999', 206, [0, 1, 2, 3, 4], 'bytes 0-4/5'], ['bytes=5-', 416, [], 'bytes */5'],
  ['bytes=3-1', 416, [], 'bytes */5'], ['bytes=-0', 416, [], 'bytes */5'],
  ['bytes=99999999999999999999-', 416, [], 'bytes */5'],
]) {
  const response = binaryResponse(data, 'video/mp4', request(range));
  assert.equal(response.status, status, range);
  assert.equal(response.headers.get('Content-Range'), contentRange);
  assert.deepEqual([...new Uint8Array(await response.arrayBuffer())], expected);
}
const head = binaryResponse(data, 'video/mp4', request('bytes=1-3', 'HEAD'));
assert.equal(head.status, 200);
assert.equal(head.headers.get('Content-Length'), '5');
assert.equal((await head.arrayBuffer()).byteLength, 0);
assert.equal(binaryResponse(data, 'video/mp4', request(null, 'POST')).status, 405);
assert.equal(binaryResponse(data, 'video/mp4', request('bytes=1-3', 'GET', { 'If-Range': 'unknown' })).status, 200);
let forwarded = null;
const oldResponse = await base.fetch(request('bytes=1-2'), { ASSETS: { async fetch(req) { forwarded = req; return new Response(data, { headers: { 'Content-Type': 'video/mp4' } }); } } });
assert.equal(oldResponse.status, 206);
assert.equal(forwarded.headers.get('Range'), null);
assert.deepEqual([...new Uint8Array(await oldResponse.arrayBuffer())], [1, 2]);

if (process.argv[2]) {
  const source = patchEntry(await readFile(process.argv[2], 'utf8'));
  const syntax = spawnSync(process.execPath, ['--check', '--input-type=module'], { input: source, encoding: 'utf8' });
  assert.equal(syntax.status, 0, syntax.stderr);
  assert.ok(source.includes(JSON.stringify(project)));
  assert.ok(source.includes(MODULE_PREFIX.slice(1) + 'gallery-'));
  console.log('PASS: current production bundle accepts the integration and remains valid JavaScript');
}
console.log('PASS: project data, original behavior, module routing, card markup, GET/HEAD, and video range handling');
