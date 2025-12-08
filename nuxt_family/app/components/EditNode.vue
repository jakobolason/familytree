<script setup lang="ts">
const { isOpen, nodeData, closeEditNode } = useEditNode();
</script>

<template>
  <USlideover v-model:open="isOpen" title="Edit Node">
    <template #body>
      <h1>Person oplysninger</h1>
      <pre>{{ nodeData }}</pre>
      <!--<LoginForm /> -->
      <p> Redigering af oplysninger kommer senere ... </p>
    </template>
  </USlideover>
</template>
