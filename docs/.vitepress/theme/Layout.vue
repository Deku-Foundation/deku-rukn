<script setup lang="ts">
import DefaultTheme from 'vitepress/theme'
import { inBrowser, useData } from 'vitepress'
import { watchEffect } from 'vue'

const { lang } = useData()

// VitePress navigates between locales without a full page reload. Keep the
// document direction synchronized with the active route so the navbar does
// not retain the previous locale's layout.
watchEffect(() => {
  if (!inBrowser) return

  const locale = lang.value.toLowerCase()
  const direction = locale === 'ar' || locale.startsWith('ar-') ? 'rtl' : 'ltr'
  document.documentElement.setAttribute('dir', direction)
})
</script>

<template>
  <DefaultTheme.Layout />
</template>
