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

## Surface and button contrast policy

Dark mode now uses neutral charcoal surfaces with a restrained green accent, rather than green for every panel. Brand buttons explicitly pair dark text with a light mint background; secondary buttons use a neutral surface.

| Pair | Calculated contrast ratio | WCAG 2.2 text threshold |
| --- | ---: | ---: |
| Dark body text #f3f5f2 on #121713 | 16.55:1 | 4.5:1 |
| Dark secondary text #cbd4cc on #121713 | 11.95:1 | 4.5:1 |
| Dark accent link #bfe6a6 on #121713 | 13.03:1 | 4.5:1 |
| Dark primary-button text #132018 on #bfe6a6 | 12.09:1 | 4.5:1 |
| Light body text #17261e on #f5f7f2 | 14.61:1 | 4.5:1 |
| Light primary-button text #ffffff on #18563a | 8.64:1 | 4.5:1 |

Ratios are calculated from solid color tokens. This does not prove every rendered state, syntax token, image, focus indicator, or user-selected display setting has passed an accessibility audit.

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
