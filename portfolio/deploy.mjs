import { readFile, writeFile, mkdir, copyFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { createHash } from 'node:crypto';

const directory = path.dirname(fileURLToPath(import.meta.url));
const files = [
  ['worker.js', 'application/javascript+module'], ['base-worker.js', 'application/javascript+module'],
  ['integration.mjs', 'application/javascript+module'], ['media/invar-v1.webp', 'application/octet-stream'],
  ['showcases/invar-v1.mp4', 'application/octet-stream'],
];
const metadata = JSON.parse(await readFile(path.join(directory, 'metadata.json'), 'utf8'));
const modules = await Promise.all(files.map(async ([name, type]) => {
  const bytes = await readFile(path.join(directory, name));
  return { name, type, bytes, sha256: createHash('sha256').update(bytes).digest('hex') };
}));
const manifest = { metadata, modules: modules.map(({ name, type, bytes, sha256 }) => ({ name, type, size: bytes.length, sha256 })) };
if (process.argv[2] === '--package') {
  if (!process.argv[3]) throw new Error('Supply the publication directory.');
  const target = path.resolve(process.argv[3]);
  for (const { name } of modules) {
    await mkdir(path.dirname(path.join(target, name)), { recursive: true });
    await copyFile(path.join(directory, name), path.join(target, name));
  }
  await writeFile(path.join(target, 'manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
  console.log(JSON.stringify({ packaged: target, modules: manifest.modules }));
} else if (process.argv[2] === '--deploy') {
  if (!process.env.CLOUDFLARE_API_TOKEN) throw new Error('Set CLOUDFLARE_API_TOKEN to deploy.');
  const account = process.env.CLOUDFLARE_ACCOUNT_ID ?? 'd1a029d95859b4294bea3ebe9f19f079';
  const form = new FormData();
  form.append('metadata', new Blob([JSON.stringify(metadata)], { type: 'application/json' }));
  for (const { name, type, bytes } of modules) form.append(name, new Blob([bytes], { type }), name);
  const response = await fetch(`https://api.cloudflare.com/client/v4/accounts/${account}/workers/scripts/portfolio-dev`, {
    method: 'PUT', headers: { Authorization: `Bearer ${process.env.CLOUDFLARE_API_TOKEN}` }, body: form,
  });
  const result = await response.json();
  if (!response.ok || !result.success) throw new Error(JSON.stringify({ status: response.status, errors: result.errors }));
  console.log(JSON.stringify({ deployed: true, deployment_id: result.result.deployment_id }));
} else {
  console.log(JSON.stringify(manifest, null, 2));
}
