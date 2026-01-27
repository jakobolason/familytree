<script setup lang="ts">
  definePageMeta({
  auth: false
  })
</script>
<template>
  <div class="h-screen flex flex-col  items-center gap-4">
    <p class="text-gray-500"> Log in</p>
    <MagicForm />
  </div>
</template>
