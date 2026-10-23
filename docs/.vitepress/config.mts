import { defineConfig } from 'vitepress'

const repoUrl = 'https://github.com/Deku-Foundation/deku-rukn'

const enSidebar = [
  { text: 'Start here', items: [
    { text: 'Overview', link: '/' },
    { text: 'Install and run', link: '/guide/getting-started' },
    { text: 'Language guide', link: '/guide/language' },
  ] },
  { text: 'Language reference', items: [
    { text: 'Specification (draft)', link: '/reference/specification' },
    { text: 'Interpreter architecture', link: '/reference/architecture' },
  ] },
  { text: 'Engineering and research', items: [
    { text: 'Unicode and security', link: '/reference/unicode-and-security' },
    { text: 'Testing strategy', link: '/reference/testing-strategy' },
    { text: 'Design decisions', link: '/project/design-decisions' },
  ] },
  { text: 'Project', items: [
    { text: 'Roadmap', link: '/project/roadmap' },
    { text: 'Contributing', link: '/project/contributing' },
  ] },
]

const arSidebar = [
  { text: 'ابدأ هنا', items: [
    { text: 'نظرة عامة', link: '/ar/' },
    { text: 'التثبيت والتشغيل', link: '/ar/guide/getting-started' },
    { text: 'دليل اللغة', link: '/ar/guide/language' },
  ] },
  { text: 'مرجع اللغة', items: [
    { text: 'المواصفات (مسودة)', link: '/ar/reference/specification' },
    { text: 'بنية المفسر', link: '/ar/reference/architecture' },
  ] },
  { text: 'الهندسة والأبحاث', items: [
    { text: 'Unicode والأمان', link: '/ar/reference/unicode-and-security' },
    { text: 'استراتيجية الاختبار', link: '/ar/reference/testing-strategy' },
    { text: 'قرارات التصميم', link: '/ar/project/design-decisions' },
  ] },
  { text: 'المشروع', items: [
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
  { text: 'Référence du langage', items: [
    { text: 'Spécification (brouillon)', link: '/fr/reference/specification' },
    { text: 'Architecture de l’interpréteur', link: '/fr/reference/architecture' },
  ] },
  { text: 'Ingénierie et recherche', items: [
    { text: 'Unicode et sécurité', link: '/fr/reference/unicode-and-security' },
    { text: 'Stratégie de test', link: '/fr/reference/testing-strategy' },
    { text: 'Décisions de conception', link: '/fr/project/design-decisions' },
  ] },
  { text: 'Projet', items: [
    { text: 'Feuille de route', link: '/fr/project/roadmap' },
    { text: 'Contribuer', link: '/fr/project/contributing' },
  ] },
]

export default defineConfig({
  base: '/deku-rukn/',
  title: 'Rukn — ركن',
  description: 'An experimental Arabic-first programming language, implemented in Rust.',
  lang: 'en',
  cleanUrls: true,
  lastUpdated: true,
  sitemap: { hostname: 'https://deku-foundation.github.io/deku-rukn/' },
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/deku-rukn/rukn-mark.svg' }],
    ['meta', { name: 'theme-color', content: '#173f32' }],
    ['meta', { name: 'color-scheme', content: 'light dark' }],
    ['meta', { property: 'og:site_name', content: 'Rukn — ركن' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Rukn — Arabic-first programming language' }],
    ['meta', { property: 'og:description', content: 'An experimental Arabic-first programming language, implemented in Rust.' }],
    ['meta', { name: 'twitter:card', content: 'summary' }],
  ],
  themeConfig: {
    logo: '/rukn-mark.svg',
    siteTitle: 'Rukn · ركن',
    outline: { label: 'On this page', level: [2, 3] },
    search: { provider: 'local' },
    socialLinks: [{ icon: 'github', link: repoUrl }],
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Specification', link: '/reference/specification' },
      { text: 'Research', items: [
        { text: 'Unicode and security', link: '/reference/unicode-and-security' },
        { text: 'Testing strategy', link: '/reference/testing-strategy' },
        { text: 'Design decisions', link: '/project/design-decisions' },
      ] },
      { text: 'Roadmap', link: '/project/roadmap' },
      { text: 'GitHub', link: repoUrl },
    ],
    sidebar: enSidebar,
    footer: { message: 'Experimental language research · Built in the open', copyright: 'Copyright © 2026 Deku Foundation contributors' },
    docFooter: { prev: 'Previous page', next: 'Next page' },
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
        outline: { label: 'في هذه الصفحة', level: [2, 3] },
        nav: [
          { text: 'الدليل', link: '/ar/guide/getting-started' },
          { text: 'المواصفات', link: '/ar/reference/specification' },
          { text: 'الأبحاث', items: [
            { text: 'Unicode والأمان', link: '/ar/reference/unicode-and-security' },
            { text: 'استراتيجية الاختبار', link: '/ar/reference/testing-strategy' },
            { text: 'قرارات التصميم', link: '/ar/project/design-decisions' },
          ] },
          { text: 'خارطة الطريق', link: '/ar/project/roadmap' },
          { text: 'المستودع', link: repoUrl },
        ],
        sidebar: arSidebar,
        footer: { message: 'بحث تجريبي في لغة برمجة · التطوير علني', copyright: 'حقوق النشر © 2026 مساهمو مؤسسة Deku' },
        docFooter: { prev: 'الصفحة السابقة', next: 'الصفحة التالية' },
      },
    },
    fr: {
      label: 'Français',
      lang: 'fr',
      title: 'Rukn',
      description: 'Un langage de programmation arabe expérimental, développé en Rust.',
      themeConfig: {
        outline: { label: 'Sur cette page', level: [2, 3] },
        nav: [
          { text: 'Guide', link: '/fr/guide/getting-started' },
          { text: 'Spécification', link: '/fr/reference/specification' },
          { text: 'Recherche', items: [
            { text: 'Unicode et sécurité', link: '/fr/reference/unicode-and-security' },
            { text: 'Stratégie de test', link: '/fr/reference/testing-strategy' },
            { text: 'Décisions de conception', link: '/fr/project/design-decisions' },
          ] },
          { text: 'Feuille de route', link: '/fr/project/roadmap' },
          { text: 'Dépôt', link: repoUrl },
        ],
        sidebar: frSidebar,
        footer: { message: 'Recherche sur un langage expérimental · Développement ouvert', copyright: 'Copyright © 2026 contributeurs de Deku Foundation' },
        docFooter: { prev: 'Page précédente', next: 'Page suivante' },
      },
    },
  },
})
