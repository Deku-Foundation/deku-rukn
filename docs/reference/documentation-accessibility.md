---
title: Documentation accessibility
description: Reading, contrast, keyboard and right-to-left design principles for the Rukn documentation site.
---

# Documentation accessibility

This page records the practical accessibility baseline for the Rukn documentation site. It is an engineering checklist, not a claim of formal WCAG conformance or a substitute for testing with assistive technology.

## Standards and references

- [WCAG 2.2](https://www.w3.org/TR/WCAG22/) — use at least 4.5:1 contrast for normal text and 3:1 for large text; ensure keyboard access, visible focus, and text resizing.
- [W3C guidance on minimum contrast](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html) — explains why low-contrast text is difficult to read.
- [Unicode UAX #9](https://www.unicode.org/reports/tr9/) — the bidirectional algorithm for mixed Arabic and Latin text.
- [VitePress theme customization](https://vitepress.dev/guide/custom-theme) — use supported theme extension points before adding custom runtime behaviour.

## Implemented in the theme

- Separate light and dark palette tokens, with stronger body and secondary text colors.
- Comfortable paragraph spacing and line height, including a more generous Arabic reading rhythm.
- Underlined documentation links, visible keyboard focus, responsive tables, and clear inline-code surfaces.
- Left-to-right code samples inside right-to-left Arabic pages.
- Reduced-motion support and narrow-screen adjustments.
- No external font download is required; system fonts are used as fallbacks.

## Manual review checklist

- [ ] Check body, secondary, link, inline-code and code-block contrast in both themes with a contrast analyser.
- [ ] Navigate the entire site with keyboard only; confirm focus is always visible and order is logical.
- [ ] Zoom to 200% and check that navigation, tables and code examples remain usable.
- [ ] Review Arabic pages at mobile widths; check punctuation, inline Latin terms, links and code direction.
- [ ] Test with at least one screen reader and browser combination before claiming conformance.
- [ ] Recheck screenshots and computed styles whenever VitePress is upgraded.

## Design decisions

Prefer legibility over decorative density. Do not use color alone to communicate state, do not remove native focus indicators without a visible replacement, and do not treat a successful production build as proof of accessibility. Contrast values and keyboard behaviour must be measured in the rendered site.

Last reviewed: 23 October 2026.
