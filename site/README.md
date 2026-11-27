# Invar landing page

Static HTML, CSS, and JavaScript served by Cloudflare Workers Static Assets at https://invar.lyngocvu.tech/. The public fallback is https://invar.zhugez.workers.dev/. No build step or runtime dependencies.

## Preview

From the repository root:

```sh
python3 -m http.server 8787 --bind 127.0.0.1 --directory site/public
```

Open http://127.0.0.1:8787/.

## Check

Requires Node.js and the `agent-browser` CLI:

```sh
node --check site/public/main.js
node site/check.mjs
```

The browser check covers horizontal overflow at desktop, tablet, and phone widths, both demo results, clipboard copying, section anchors, and static asset loading. To check production:

```sh
node site/check.mjs https://invar.lyngocvu.tech
```

## Deploy

With Wrangler authenticated to the account that owns `lyngocvu.tech`:

```sh
npx wrangler@4.168.0 deploy --config site/wrangler.jsonc
```

The config names the Worker `invar` and attaches `invar.lyngocvu.tech` as a custom domain. `_headers` sets the site's security policy and asset caching. `404.html` handles unknown paths.

This account protects Workers with Cloudflare Access by default. The application `invar - Cloudflare Workers` attaches the existing reusable public-bypass policy to this Worker only. Preserve that application when redeploying the public landing page. It is managed separately from Wrangler.

The first deployment was verified over public HTTPS on November 27, 2026. Production browser checks passed. The homepage returned 200, and unknown paths and `/_headers` returned 404.

A later Firefox failure revealed inconsistent DNS results. Both authoritative nameservers and the Cloudflare, Google, and Quad9 public resolvers returned the correct IPv4 and IPv6 addresses. The deployment environment's default resolver returned `ENOTFOUND` for IPv4. A browser using the authoritative IPv4 address reached the custom domain over valid HTTPS with status 200. This confirms service availability through that address, not successful resolution on every client network.

The `workers_dev: true` setting is now applied in Cloudflare, providing the fallback above. Browser checks passed on that hostname using ordinary DNS. If the custom domain fails in Firefox, clear its DNS cache at `about:networking#dns` and retry, or use the fallback. Do not recreate the managed DNS record or hard-code an edge IP as a permanent workaround.

## Content and assets

Product claims, workflow commands, installer support, and limitations come from the repository README, PRODUCT.md, SECURITY.md, and ordering example. The browser demo is explicitly illustrative. It does not execute Invar or visitor code.

The logo, favicon, social card, and Barlow fonts come from `brand/`. The font license ships as `public/assets/OFL.txt`. The social card retains embedded origin metadata. The canvas spine is drawn locally and does not use an external image service. No analytics, tracking cookies, remote fonts, or third-party scripts are included.
