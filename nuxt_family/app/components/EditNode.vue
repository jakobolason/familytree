<script setup lang="ts">
const { isOpen, nodeData, closeEditNode } = useEditNode();
</script>

<template>
  <USlideover v-model:open="isOpen" title="Edit Node">
    <template #body>
      <h1>Edit node</h1>
      <!-- Access nodeData here if needed -->
      <pre>{{ nodeData }}</pre>
      <LoginForm />
    </template>
  </USlideover>
</template>
