<script setup lang="ts">
  definePageMeta({
  auth: false
  })
</script>
<template>
  <div>
    <UPageHero
      title="Familie Krøniken"
      description="Fin lille familietræ hjemmeside"
      :links="[
        {
          label: 'Åbn familietræ',
          to: '/tree',
          trailingIcon: 'i-lucide-arrow-right',
          size: 'xl',
        },
      ]"
    />

  </div>
</template>
