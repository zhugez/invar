import base from './base-worker.js';
import { MODULE_PREFIX, project, card, patchEntry, originalModulePath } from './integration.mjs';

function changedHeaders(response, cacheControl) {
  const headers = new Headers(response.headers);
  for (const key of ['Content-Encoding', 'Content-Length', 'ETag', 'Last-Modified']) headers.delete(key);
  headers.set('Cache-Control', cacheControl);
  return headers;
}

export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    if (url.pathname === '/showcases/_invar-portfolio-home') {
      url.pathname = '/';
      request = new Request(url, request);
    }
    if (url.pathname === project.image || url.pathname === project.showcase) return base.fetch(request, { ASSETS: env.INVAR_MEDIA }, ctx);
    if (url.pathname.startsWith(MODULE_PREFIX)) {
      const pathname = originalModulePath(url.pathname);
      if (!pathname) return new Response('Not found', { status: 404 });
      url.pathname = pathname;
      const response = await env.ASSETS.fetch(new Request(url, request));
      if (response.status !== 200 || !pathname.endsWith('.js')) return response;
      if (request.method === 'HEAD') return new Response(null, { headers: changedHeaders(response, 'public, max-age=31536000, immutable') });
      let source = await response.text();
      if (/\/index-[a-zA-Z0-9_-]+\.js$/.test(pathname)) source = patchEntry(source);
      return new Response(source, {
        headers: changedHeaders(response, 'public, max-age=31536000, immutable'),
      });
    }
    if (['/', '/index.html'].includes(url.pathname) && ['GET', 'HEAD'].includes(request.method)) {
      const headers = new Headers(request.headers);
      for (const name of ['If-None-Match', 'If-Modified-Since', 'Range', 'If-Range']) headers.delete(name);
      const response = await env.ASSETS.fetch(new Request(request, { headers }));
      if (response.status !== 200) return response;
      if (request.method === 'HEAD') return new Response(null, { headers: changedHeaders(response, 'no-cache') });
      return new HTMLRewriter()
        .on('script[type="module"][src^="/assets/index-"]', {
          element(element) { element.setAttribute('src', MODULE_PREFIX + element.getAttribute('src').slice('/assets/'.length)); },
        })
        .on('article[data-project="pingloom"]', {
          element(element) { element.before(card, { html: true }); },
        })
        .transform(new Response(response.body, { headers: changedHeaders(response, 'no-cache') }));
    }
    return base.fetch(request, env, ctx);
  },
};
