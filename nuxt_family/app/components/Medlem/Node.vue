<script setup lang="ts">
const { isOpen, nodeData, closeMedlemNode } = useMedlemNode();
console.log('nodeData in MedlemNode.vue: ', nodeData.pid);
const memberDetails = ref(null);
const status = ref('idle'); // 'idle' | 'pending' | 'success' | 'error'

// 2. Define a manual fetch function that TAKES the ID
const fetchMember = async (id: string) => {
  status.value = 'pending';
  memberDetails.value = null;

  try {
    const data = await $fetch(`/api/medlem/${id}`);
    memberDetails.value = data;
    console.log('got medlem data: ', data)
    status.value = 'success';
  } catch (err) {
    console.error(err);
    status.value = 'error';
  }
};
onMounted(() => {
  watch([isOpen, nodeData], ([newOpen, newNodeData]) => {
    if (newOpen && newNodeData?.pid) {
      console.log('Conditions met. Fetching data for:', newNodeData.pid);
      fetchMember(newNodeData.pid);
    } else if (!newOpen) {
      // When closed, values should be set to null
      memberDetails.value = null;
    }
  });
})

</script>

<template>
  <USlideover v-model:open="isOpen" title="Edit Node">
    <template #body>
      <h1>Person oplysninger</h1>
      <div v-if="status === 'pending'" class="p-4 space-y-4">
        <div class="flex items-center gap-4">
          <USkeleton class="h-16 w-16 rounded-full" />
          <div class="space-y-2">
            <USkeleton class="h-4 w-48" />
            <USkeleton class="h-4 w-24" />
          </div>
        </div>
        <USkeleton class="h-32 w-full" />
      </div>

      <div v-else-if="memberDetails" class="p-4">
        <MedlemDetails :member="memberDetails" />
      </div>

      <div v-else class="p-8 text-center text-gray-500">
        <UIcon name="i-heroicons-exclamation-circle" class="w-8 h-8 mx-auto mb-2" />
        <p>Kunne ikke hente oplysninger.</p>
      </div>
    </template>
  </USlideover>
</template>
