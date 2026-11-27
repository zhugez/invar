# Landing page design

Scope: `site/public/`. Product truth and the existing Invar identity remain defined at the repository root. This document records the landing page's visual decisions.

The user chose Radixark as the visual reference. The page carries its dark, spacious composition and clear typography into the existing Invar brand. The hero pairs the original tagline with a three-dimensional particle rendering of the invariant-spine mark. Animated particles move around the center, while the reference spine remains fixed.

## Typography and color

Barlow Regular carries headings and prose. Barlow Bold is available for emphasis. Both fonts are hosted locally. Terminal content uses the operating system's monospace stack. Large headings use tight tracking and moderate line height.

- Ground: `#0c0e10`.
- Terminal surface: `#111518`.
- Main text: `#f0f2ed`.
- Secondary text: `#a2aaa9`.
- Rules: `#292e32`.
- Accessible blue accent: `#6989ff`, derived from Invar Signal Blue `#2347f5`.
- Matching result: `#9bc8ac`.
- Changed result: `#eda58b`.

## Layout and behavior

The desktop shell is at most 1312 CSS pixels, including 56-pixel side gutters. Sections use open layouts and thin rules. Primary actions have a light pill treatment. The terminal and install command are the only major bounded UI panels.

The sequence is offer, behavioral comparison, workflow, product boundaries, installation. The demo uses native buttons with pressed states and a live result region. It labels its content illustrative and preserves the product's coverage limitations. Copying the installer uses the native Clipboard API and reports success or failure.

At 760 pixels and below, the hero and main layouts stack. Workflow steps use two columns. Essential navigation remains in the header and the footer provides the full external link set. The shell narrows to 24-pixel gutters, then 18 pixels below 360 pixels.

The canvas pauses outside the viewport and when the document is hidden. Reduced-motion preference produces a static frame and disables smooth scrolling. Text content stays visible if JavaScript is unavailable.

## Verification record

Desktop and phone renders were inspected locally. The browser check passed at 1440, 768, 390, and 320 pixels, including both demo branches, clipboard copying, anchors, and asset loading. The JavaScript syntax check and Wrangler deployment dry run passed.

Review scope: an in-thread review, without an independent reviewer. The optional Impeccable detector returned no findings in degraded regex mode. Its HTML/CSS parser modules were unavailable, so that result is not a complete accessibility audit. No user content, counters, customer claims, or performance benchmarks were invented.
