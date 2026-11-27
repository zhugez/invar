// Recovered unchanged from the active portfolio-dev version c436384a-0371-402f-85c3-dfc83f5f8b7d.
export default {
  async fetch(request, { ASSETS }) {
    const range = /^bytes=(\d*)-(\d*)$/i.exec(request.headers.get('Range') ?? '');
    if (request.method !== 'GET' || !/^\/showcases\/[a-z0-9-]+\.mp4$/.test(new URL(request.url).pathname) ||
        !range || (!range[1] && !range[2]) || request.headers.has('If-Range'))
      return ASSETS.fetch(request);
    const headers = new Headers(request.headers);
    headers.delete('Range');
    const asset = await ASSETS.fetch(new Request(request, { headers }));
    if (asset.status !== 200 || !asset.headers.get('Content-Type')?.startsWith('video/mp4')) return asset;
    // ponytail: buffer these small clips; use range-capable storage if clip size or traffic grows.
    const file = await asset.arrayBuffer();
    const size = file.byteLength;
    const start = range[1] ? Number(range[1]) : Math.max(0, size - Number(range[2]));
    const end = range[1] && range[2] ? Math.min(Number(range[2]), size - 1) : size - 1;
    const responseHeaders = new Headers(asset.headers);
    responseHeaders.delete('Content-Encoding');
    responseHeaders.set('Accept-Ranges', 'bytes');
    if (start >= size || start > end) {
      responseHeaders.set('Content-Range', `bytes */${size}`);
      responseHeaders.set('Content-Length', '0');
      return new Response(null, { status: 416, headers: responseHeaders });
    }
    responseHeaders.set('Content-Range', `bytes ${start}-${end}/${size}`);
    responseHeaders.set('Content-Length', String(end - start + 1));
    return new Response(file.slice(start, end + 1), { status: 206, headers: responseHeaders });
  },
};
