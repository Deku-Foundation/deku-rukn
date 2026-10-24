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
  assert.match(html, /name="viewport"/, page.name + ': viewport metadata')
  assert.match(html, /rukn-hero(?:-dark)?\\.svg/, page.name + ': hero artwork reference')
  assert.ok(html.includes(page.marker), page.name + ': localized hero copy')
}

for (const asset of ['rukn-hero.svg', 'rukn-hero-dark.svg']) {
  await access(new URL('../public/' + asset, import.meta.url))
}

const themeCss = await readFile(new URL('../.vitepress/theme/custom.css', import.meta.url), 'utf8')
const directionCss = await readFile(new URL('../.vitepress/theme/direction.css', import.meta.url), 'utf8')
assert.match(themeCss, /grid-template-columns:\\s*minmax\\(0,\\s*1\\.05fr\\)\\s+minmax\\(0,\\s*\\.95fr\\)/, 'hero uses bounded desktop columns')
assert.match(themeCss, /max-width:\\s*min\\(100%,\\s*450px\\)/, 'hero artwork is width-constrained')
assert.doesNotMatch(themeCss, /max-width:\\s*min\\(100%,\\s*60rem\\)/, 'hero text must not escape its column')
assert.match(directionCss, /html\\[dir="rtl"\\]\\s+\\.VPSidebar\\.VPSidebar/, 'RTL sidebar override must outrank scoped component styles')
assert.match(directionCss, /html\\[dir="rtl"\\]\\s+\\.VPContent\\.has-sidebar/, 'RTL content column must reserve space on the inline-start side')

console.log('UI contract passed: English, Arabic, French, responsive hero constraints, and RTL layout overrides.')
