<script setup lang="ts">
// 1. Props: Accepts an array of PIDs the user has permission to edit
const props = defineProps<{
  pids: string[]
}>();

// 2. State
// We default to the first PID in the list if available
const selectedPid = ref<string | null>(props.pids.length > 0 ? props.pids[0] : null);
const memberData = ref(null);
const status = ref('idle'); // 'idle' | 'pending' | 'success' | 'error'
const isEditing = ref(false);

// 3. Fetch Function
const fetchMember = async (pid: string) => {
  if (!pid) return;

  status.value = 'pending';
  // Clear old data so we don't show "User A" while loading "User B"
  memberData.value = null;

  try {
    const data = await $fetch(`/api/medlem/${pid}`);
    memberData.value = data;
    status.value = 'success';
  } catch (err) {
    console.error('Fetch error:', err);
    status.value = 'error';
  }
};

// 4. Watch for Selection Changes
// Whenever the user picks a new person from the dropdown, we fetch their data
watch(selectedPid, (newPid) => {
  if (newPid) {
    isEditing.value = false; // Always return to "View" mode when switching people
    fetchMember(newPid);
  }
}, { immediate: true }); // immediate: true ensures we fetch the first one on load

// 5. Handlers
const handleEditSuccess = () => {
  isEditing.value = false;
  if (selectedPid.value) {
    fetchMember(selectedPid.value); // Refresh to show new data
  }
};
</script>

<template>
  <div class="h-full flex flex-col relative">

    <div class="flex justify-between items-end mb-6 pb-4 border-b border-gray-200 dark:border-gray-800">

      <div class="w-full max-w-xs">
        <label v-if="pids.length > 1" class="text-xs font-semibold text-gray-500 uppercase mb-1 block">
          Vælg Profil
        </label>

        <USelectMenu v-if="pids.length > 1" v-model="selectedPid" :options="pids" placeholder="Vælg person..."
          class="w-full" />

        <div v-else>
          <h2 class="text-xl font-bold" v-if="memberData">{{ memberData.name }}</h2>
          <USkeleton v-else class="h-8 w-48" />
        </div>
      </div>

      <UButton v-if="!isEditing && status === 'success'" icon="i-heroicons-pencil-square" size="sm"
        @click="isEditing = true">
        Rediger
      </UButton>
    </div>

    <div class="flex-1 overflow-y-auto">

      <div v-if="status === 'pending'" class="space-y-6 animate-pulse">
        <div class="flex items-center gap-4">
          <div class="h-16 w-16 bg-gray-200 dark:bg-gray-800 rounded-full"></div>
          <div class="space-y-2">
            <div class="h-5 w-40 bg-gray-200 dark:bg-gray-800 rounded"></div>
            <div class="h-4 w-24 bg-gray-200 dark:bg-gray-800 rounded"></div>
          </div>
        </div>
        <div class="h-32 w-full bg-gray-200 dark:bg-gray-800 rounded"></div>
      </div>

      <div v-else-if="status === 'error'" class="flex flex-col items-center justify-center h-48 text-gray-500">
        <UIcon name="i-heroicons-exclamation-triangle" class="w-8 h-8 mb-2" />
        <p>Kunne ikke hente data.</p>
        <UButton label="Prøv igen" variant="link" @click="fetchMember(selectedPid!)" />
      </div>

      <div v-else-if="memberData">

        <div class="transition-opacity duration-200">

          <div v-if="isEditing">
            <EditMedlemForm :member="{ pid: selectedPid!, ...memberData }" @success="handleEditSuccess"
              @cancel="isEditing = false" />
          </div>

          <div v-else>
            <MedlemDetails :member="memberData" />
          </div>

        </div>
      </div>

      <div v-else class="text-gray-500 text-center mt-10">
        Vælg en profil for at se detaljer.
      </div>
    </div>

  </div>
</template>
