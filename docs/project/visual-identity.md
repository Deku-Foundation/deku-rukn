---
title: Visual identity and design system
description: The visual identity, SVG logo assets, colours, accessibility, and content rules for Rukn within Deku Foundation.
---

# Visual identity and design system

Rukn is a language-research project within Deku Foundation. Its visual system should be recognisably part of the foundation, while giving the language its own mark and vocabulary. The goal is a quiet, technical, documentation-first identity—not a separate brand that conflicts with the foundation.

## Official assets

![Rukn wordmark on a light surface](/rukn-wordmark.svg)

![Rukn wordmark for a dark surface](/rukn-wordmark-inverse.svg)

| Asset | Intended use |
| --- | --- |
| `/rukn-mark.svg` | Square mark, favicon, compact icon and avatar |
| `/rukn-wordmark.svg` | Main wordmark on light or neutral surfaces |
| `/rukn-wordmark-inverse.svg` | Wordmark on a dark evergreen surface |

The SVG sources remain editable. They use vector paths and system-font fallbacks; no external font or image request is needed. The files include a title and description for assistive technology. For a website header, pair the mark with the visible text name “Rukn · ركن” so the name is not communicated by an icon alone.

## Concept

The mark combines an arch/corner with a grounded baseline. The geometry evokes a supporting “rukn” (ركن), while a small gold point suggests a token or point of syntax. It shares the foundation’s architectural, evergreen direction without reusing the foundation mark itself.

The symbol is a visual metaphor, not a claim that the language already has a complete compiler, package ecosystem, or self-hosting implementation.

## Colour tokens

These values intentionally align with the Deku Foundation website design tokens.

| Token | Hex | Purpose |
| --- | --- | --- |
| Evergreen | `#12372A` | Primary dark surface and mark background |
| Deep evergreen | `#0B271D` | Darkest surfaces |
| Leaf green | `#27734F` | Primary action and links on light surfaces |
| Deep green | `#18563A` | High-contrast green text and actions |
| Soft green | `#7DBB81` | Supporting marks and secondary accents |
| Pale mint | `#BFE6A6` | Bright accent on dark surfaces |
| Paper | `#F5F7F2` | Primary light background |
| Soft paper | `#EDF2E9` | Alternate surface |
| Off-white | `#F0F7EA` | Logo foreground and dark-surface text |
| Muted text | `#58675E` | Secondary copy on light surfaces |
| Gold | `#D5AF66` | A limited accent; never the only status signal |

Do not use pale mint for small text on white, or muted text on dark backgrounds without checking contrast. The site’s design-token check is a regression guard for selected pairs, not a complete WCAG audit.

## Typography

Prefer system-font stacks, with Arabic-capable fallbacks such as Noto Sans Arabic and Tahoma. The site does not bundle or download fonts. Arabic technical prose should remain readable around Latin identifiers, file paths and code snippets; code blocks remain left-to-right even on Arabic pages.

## Layout and behaviour

- Keep pages content-first, with short paragraphs, informative headings, readable line length and obvious next steps.
- Use the same information architecture in English, Arabic and French. Locales should be equivalent in meaning and project status, not mechanically identical word for word.
- Arabic pages must set the locale to Arabic and use right-to-left direction. Inline source code and terminal output must keep left-to-right direction.
- Keep keyboard focus visible, preserve keyboard navigation and respect reduced-motion preferences.
- Use status labels and words, not colour alone, to distinguish implemented features, research proposals and deferred work.
- Prefer local SVG assets and progressive enhancement; do not add tracking pixels, autoplay media or external font loading for decoration.

## Content and product-status rules

Rukn is **experimental and pre-alpha**. Use “research,” “prototype,” “planned,” or “open question” when appropriate. Do not describe it as production-ready, self-hosting, feature-complete, formally secure, or ABI-stable without code and a reviewed release decision that support the claim.

A language feature is considered implemented only when the code, tests and specification agree. A research page or roadmap item is not evidence of implementation.

## Change process

When changing logo geometry, colour tokens or page-wide components:

1. Keep the editable SVG source in `docs/public/`.
2. Update favicon and site metadata references if an asset name changes.
3. Check the light and dark themes, mobile layout, Arabic RTL rendering and focus states.
4. Run `npm ci --no-audit --no-fund` and `npm run build` from `docs/`.
5. Ask for visual review and retain the build result in the pull request.

## Inspiration and standards

- [Deku Foundation website](https://deku-foundation.github.io/) — shared palette and visual direction.
- [WCAG 2.2](https://www.w3.org/TR/WCAG22/) — accessibility requirements and guidance.
- [W3C SVG Accessibility API Mappings](https://www.w3.org/TR/svg-aam-1.0/) — accessible names and descriptions for SVG.
- [W3C SVG 2](https://www.w3.org/TR/SVG2/) — the SVG model.

These references inform the implementation; they do not imply the whole site has been independently certified.
