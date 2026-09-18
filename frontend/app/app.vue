<script setup>
useHead({
  meta: [{ name: 'viewport', content: 'width=device-width, initial-scale=1' }],
  link: [{ rel: 'icon', href: '/favicon.ico' }],
  htmlAttrs: {
    lang: 'en'
  }
})

const title = 'Wistoft og Vestergaard familien'
const description = 'Familie træ og kontakt information'

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description
})
</script>

<template>
  <NuxtLayout>
    <UApp>
      <UMain>
        <NuxtPage />
      </UMain>
    </UApp>
  </NuxtLayout>
</template>
