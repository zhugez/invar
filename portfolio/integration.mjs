export const MODULE_PREFIX = '/assets/invar-portfolio-v1/';
export const project = {
  id: 'invar', layer: 'tools', title: 'Invar', category: 'Behavior verification', year: '2026',
  stack: 'Rust · CLI · SHA-256', kind: 'Open source · v0.1',
  image: '/media/invar-v1.webp', video: '/showcases/invar-v1.mp4', showcase: '/showcases/invar-v1.mp4',
  mediaLabel: 'Website screenshot', mediaCredit: 'Original website recording · Original ambient score',
  summary: 'Change code. Preserve behavior.',
  description: 'Invar is a local Rust CLI that captures reviewed commands, requires explicit candidate approval, and compares exit code, stdout and stderr byte for byte against a locked baseline. Passing covers the configured cases. Commands run on your machine without a sandbox.',
  decision: 'Separate capture from approval. Lock an explicitly reviewed candidate hash instead of automatically accepting changed output. The website comparison is illustrative and never executes visitor code.',
  links: [
    { label: 'Explore Invar', url: 'https://invar.lyngocvu.tech/' },
    { label: 'Read the source', url: 'https://github.com/zhugez/invar' },
    { label: 'Alternative website', url: 'https://invar.zhugez.workers.dev/' },
  ],
};

export const card = `<article class="project" data-project="invar">
  <button class="project-preview" data-open="invar" aria-label="Open Invar showcase"><img src="${project.image}" alt="Invar — website screenshot" width="1280" height="720" loading="lazy"><span class="project-chip">05 <span>Around the stack</span></span><span class="view-word">View project</span></button>
  <div class="project-meta"><h3>Invar</h3><p>Behavior verification<span>, 2026</span></p></div>
  <p class="project-summary">${project.summary}</p><p class="project-stack">${project.stack}</p>
  <p class="project-source"><a href="${project.links[0].url}" target="_blank" rel="noopener noreferrer">Open full website<span class="sr-only"> — Invar, new tab</span></a><a href="${project.links[1].url}">Read the source<span class="sr-only"> — Invar</span></a></p>
</article>`;

export function patchEntry(source) {
  const replacements = [
    ['{id:`pingloom`,layer:`tools`', `${JSON.stringify(project)},{id:\`pingloom\`,layer:\`tools\``],
    ['return`/showcases/${e.id}.mp4`', 'return e.showcase??`/showcases/${e.id}.mp4`'],
    ['textContent=`Project showcase · Music and sound effects`', 'textContent=n.mediaCredit??`Project showcase · Music and sound effects`'],
  ];
  for (const [before, after] of replacements) {
    if (source.split(before).length !== 2) throw new Error('The portfolio entry bundle changed. Review its project integration before publishing.');
    source = source.replace(before, after);
  }
  // Keep preloads and imports on the same module URLs, including circular imports.
  return source.replace(/([`"'])assets\//g, `$1${MODULE_PREFIX.slice(1)}`);
}

export function originalModulePath(pathname) {
  if (!pathname.startsWith(MODULE_PREFIX)) return null;
  const file = pathname.slice(MODULE_PREFIX.length);
  return /^[a-zA-Z0-9_-]+(?:\.[a-zA-Z0-9_-]+)*\.js$/.test(file) ? `/assets/${file}` : null;
}

export function binaryResponse(bytes, type, request) {
  const size = bytes.byteLength;
  const headers = new Headers({
    'Content-Type': type, 'Content-Length': String(size),
    'Cache-Control': 'public, max-age=31536000, immutable',
    'X-Content-Type-Options': 'nosniff', 'Accept-Ranges': 'bytes',
  });
  if (!['GET', 'HEAD'].includes(request.method)) return new Response(null, { status: 405, headers: { Allow: 'GET, HEAD' } });
  const range = /^bytes=(\d*)-(\d*)$/i.exec(request.headers.get('Range') ?? '');
  if (request.method === 'GET' && range && (range[1] || range[2]) && !request.headers.has('If-Range')) {
    const start = range[1] ? Number(range[1]) : Math.max(0, size - Number(range[2]));
    const end = range[1] && range[2] ? Math.min(Number(range[2]), size - 1) : size - 1;
    if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start >= size || start > end) {
      headers.set('Content-Range', `bytes */${size}`); headers.set('Content-Length', '0');
      return new Response(null, { status: 416, headers });
    }
    headers.set('Content-Range', `bytes ${start}-${end}/${size}`); headers.set('Content-Length', String(end - start + 1));
    return new Response(bytes.slice(start, end + 1), { status: 206, headers });
  }
  return new Response(request.method === 'HEAD' ? null : bytes, { headers });
}
