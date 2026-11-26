# Invar visual identity

This is the proposed identity created for the requested brand kit. The existing product name and tagline remain authoritative. Aesthetic choices below are professional defaults for this deliverable, not previously confirmed user preferences.

## Direction: Invariant spine

A fixed vertical stem joins two slightly displaced horizontal bars. This custom I is the central brand mark. The displacement represents code changing around observed behavior. It is a visual metaphor, not a promise that Invar proves correctness or provides a security boundary.

The visual world borrows the discipline of a technical drawing and the directness of a code diff: fixed reference axes, decisive rectangles, exact labels, and generous open space. Use the mark's geometry as the main image. Avoid shields, padlocks, checkmark seals, AI sparkles, terminal-green decoration, glows, and gradients.

## System

- Primary color: Signal Blue, #2347F5.
- Text: Graphite, #17262C. Light ground: Chalk, #F3F6F7.
- Secondary text: Steel, #586A74. Decorative separators: Line, #CFD8DC.
- Typeface: self-hosted Barlow, regular 400 and bold 700. Use the local system monospace only for code, filenames, and literal values.
- Logo lettering is custom SVG path geometry. It does not depend on Barlow or a system font.
- Large headings use bold weight and tight line height. Body text stays readable and direct.
- Use square geometry, thin rules, and a restrained spacing scale. Avoid rounded-card page scaffolds.
- Blue identifies the brand. Deep green, rust, and ochre identify labeled product states and remain separate from the identity palette.

## Brand-kit surface

The static preview opens with a large blue identity plate and a white vector logo. A short introduction leads to mark construction, real logo downloads, palette, typography, voice, and applications. The construction slider moves the caps while retaining the same stem. It is explicitly an illustration and does not alter exported assets or execute CLI commands.

The page works offline, uses native navigation and download links, supports narrow screens and keyboard navigation, and honors reduced motion. Light and dark presentations use the same identity assets and color definitions. Source files and usage rules live in brand/.

## Other surfaces

CLI output stays plain and legible in the user's terminal. Do not require a brand font or add decoration to machine-readable output. Documentation uses the primary logo on a light ground and the reversed logo on a dark ground. The social image uses the same fixed spine and the existing tagline. Small icons use only the mark.

## Sources of truth

- brand/make_assets.py: shared mark, lettering, and generated SVG geometry.
- brand/tokens.json: generated palette and typography values.
- brand/tokens.css: generated web tokens.
- brand/README.md: usage, voice, export notes, and scope.
- brand/fonts/OFL.txt: retained font license.

No trademark clearance, external adoption, or proven competitive advantage is implied by this visual work.
