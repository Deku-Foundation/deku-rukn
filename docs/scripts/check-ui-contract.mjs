import assert from 'node:assert/strict'
import { access, readFile } from 'node:fs/promises'

const htmlPages = [
  { name: 'English', path: new URL('../.vitepress/dist/index.html', import.meta.url), lang: 'en', dir: 'ltr', marker: 'Arabic-first programming' },
  { name: 'Arabic', path: new URL('../.vitepress/dist/ar/index.html', import.meta.url), lang: 'ar', dir: 'rtl', marker: 'لغة برمجة عربية أولًا' },
  { name: 'French', path: new URL('../.vitepress/dist/fr/index.html', import.meta.url), lang: 'fr', dir: 'ltr', marker: 'Programmer d’abord en arabe' }
]

for (const page of htmlPages) {
  const html = await readFile(page.path, 'utf8')
  assert.match(html, new RegExp('<html[^>]*\\blang="' + page.lang + '"'), page.name + ': lang attribute')
  assert.match(html, new RegExp('<html[^>]*\\bdir="' + page.dir + '"'), page.name + ': dir attribute')
  assert.ok(html.includes('name="viewport"'), page.name + ': viewport metadata')
  assert.ok(html.includes('rukn-hero.svg') || html.includes('rukn-hero-dark.svg'), page.name + ': hero artwork reference')
  assert.ok(html.includes(page.marker), page.name + ': localized hero copy')
}

for (const asset of ['rukn-hero.svg', 'rukn-hero-dark.svg']) {
  await access(new URL('../public/' + asset, import.meta.url))
}

const themeCss = await readFile(new URL('../.vitepress/theme/custom.css', import.meta.url), 'utf8')
const directionCss = await readFile(new URL('../.vitepress/theme/direction.css', import.meta.url), 'utf8')
const darkHero = await readFile(new URL('../public/rukn-hero-dark.svg', import.meta.url), 'utf8')

assert.ok(themeCss.includes('grid-template-columns: minmax(0, 1.05fr) minmax(0, .95fr);'), 'hero uses bounded desktop columns')
assert.ok(themeCss.includes('width: min(100%, 450px);'), 'hero artwork is width-constrained')
assert.ok(!themeCss.includes('max-width: min(100%, 60rem)'), 'hero text must not escape its column')
assert.ok(themeCss.includes('.dark .VPHome .VPHero .image-container .VPImage.light'), 'light hero artwork is hidden in dark mode')
assert.ok(themeCss.includes('html:not(.dark) .VPHome .VPHero .image-container .VPImage.dark'), 'dark hero artwork is hidden in light mode')
assert.ok(themeCss.includes('align-items: center;'), 'hero buttons and controls have explicit center alignment')
assert.ok(themeCss.includes('max-width: min(100%, 74ch);'), 'homepage code examples use a centered reading measure')
assert.ok(themeCss.includes('--vp-c-bg: #111418;'), 'dark canvas uses a neutral charcoal, not a green cast')
assert.ok(darkHero.includes('#252a30') && !darkHero.includes('#202923'), 'dark illustration uses neutral charcoal surfaces')
assert.ok(directionCss.includes('html[dir="rtl"] .VPSidebar.VPSidebar'), 'RTL sidebar override must outrank scoped component styles')
assert.ok(directionCss.includes('html[dir="rtl"] .VPContent.has-sidebar'), 'RTL content column must reserve space on the inline-start side')
assert.ok(directionCss.includes('html[dir="rtl"] .VPNavBar .container {\n  direction: ltr;'), 'RTL navbar outer frame stays stable')
assert.ok((await readFile(new URL('../.vitepress/config.mts', import.meta.url), 'utf8')).includes("{ text: 'Project', items: ["), 'long top-level nav entries are grouped')

console.log('UI contract passed: locale direction, theme artwork visibility, dark palette, buttons, homepage code, and RTL navigation.')
