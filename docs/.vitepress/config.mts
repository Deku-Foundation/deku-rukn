import { defineConfig } from 'vitepress'

const enSidebar = [
  { text: 'Start here', items: [
    { text: 'Overview', link: '/' },
    { text: 'Install and run', link: '/guide/getting-started' },
    { text: 'Language guide', link: '/guide/language' },
  ] },
  { text: 'Design', items: [
    { text: 'Language specification', link: '/reference/specification' },
    { text: 'Architecture', link: '/reference/architecture' },
    { text: 'Roadmap', link: '/project/roadmap' },
    { text: 'Contributing', link: '/project/contributing' },
  ] },
]

const arSidebar = [
  { text: 'البداية', items: [
    { text: 'نظرة عامة', link: '/ar/' },
    { text: 'التثبيت والتشغيل', link: '/ar/guide/getting-started' },
    { text: 'دليل اللغة', link: '/ar/guide/language' },
  ] },
  { text: 'التصميم', items: [
    { text: 'مواصفات اللغة', link: '/ar/reference/specification' },
    { text: 'البنية الداخلية', link: '/ar/reference/architecture' },
    { text: 'خارطة الطريق', link: '/ar/project/roadmap' },
    { text: 'المساهمة', link: '/ar/project/contributing' },
  ] },
]

const frSidebar = [
  { text: 'Commencer', items: [
    { text: 'Présentation', link: '/fr/' },
    { text: 'Installer et exécuter', link: '/fr/guide/getting-started' },
    { text: 'Guide du langage', link: '/fr/guide/language' },
  ] },
  { text: 'Conception', items: [
    { text: 'Spécification', link: '/fr/reference/specification' },
    { text: 'Architecture', link: '/fr/reference/architecture' },
    { text: 'Feuille de route', link: '/fr/project/roadmap' },
    { text: 'Contribuer', link: '/fr/project/contributing' },
  ] },
]

export default defineConfig({
  base: '/deku-rukn/',
  title: 'Rukn',
  description: 'An experimental Arabic-first programming language built in Rust.',
  lang: 'en',
  cleanUrls: true,
  lastUpdated: true,
  sitemap: { hostname: 'https://deku-foundation.github.io/deku-rukn/' },
  head: [
    ['meta', { name: 'theme-color', content: '#173f32' }],
    ['meta', { property: 'og:site_name', content: 'Rukn language' }],
    ['meta', { property: 'og:type', content: 'website' }],
  ],
  themeConfig: {
    logo: '/rukn-mark.svg',
    siteTitle: 'Rukn',
    outline: { label: 'On this page', level: [2, 3] },
    search: { provider: 'local' },
    socialLinks: [{ icon: 'github', link: 'https://github.com/Deku-Foundation/deku-rukn' }],
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Language reference', link: '/reference/specification' },
      { text: 'Roadmap', link: '/project/roadmap' },
      { text: 'GitHub', link: 'https://github.com/Deku-Foundation/deku-rukn' },
    ],
    sidebar: enSidebar,
    footer: { message: 'Experimental language research · Built in the open', copyright: 'Copyright © 2026 Deku Foundation contributors' },
  },
  locales: {
    root: { label: 'English', lang: 'en', themeConfig: { sidebar: enSidebar } },
    ar: {
      label: 'العربية',
      lang: 'ar',
      dir: 'rtl',
      title: 'ركن',
      description: 'لغة برمجة عربية أولًا قيد التجربة، مبنية باستخدام Rust.',
      themeConfig: {
        outline: { label: 'في هذه الصفحة' },
        nav: [
          { text: 'الدليل', link: '/ar/guide/getting-started' },
          { text: 'مواصفات اللغة', link: '/ar/reference/specification' },
          { text: 'خارطة الطريق', link: '/ar/project/roadmap' },
          { text: 'المستودع', link: 'https://github.com/Deku-Foundation/deku-rukn' },
        ],
        sidebar: arSidebar,
        footer: { message: 'بحث تجريبي في لغة برمجة · التطوير علني', copyright: 'حقوق النشر © 2026 مساهمو مؤسسة Deku' },
      },
    },
    fr: {
      label: 'Français',
      lang: 'fr',
      title: 'Rukn',
      description: 'Un langage de programmation arabe en expérimentation, développé en Rust.',
      themeConfig: {
        outline: { label: 'Sur cette page' },
        nav: [
          { text: 'Guide', link: '/fr/guide/getting-started' },
          { text: 'Spécification', link: '/fr/reference/specification' },
          { text: 'Feuille de route', link: '/fr/project/roadmap' },
          { text: 'Dépôt', link: 'https://github.com/Deku-Foundation/deku-rukn' },
        ],
        sidebar: frSidebar,
        footer: { message: 'Recherche sur un langage expérimental · Développement ouvert', copyright: 'Copyright © 2026 contributeurs de Deku Foundation' },
      },
    },
  },
})
