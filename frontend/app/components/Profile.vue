<script setup lang="ts">
const { isOpen } = useProfile();
const { user } = useUserSession();

const { data: medlem_pids, status, error } = await useFetch('/api/user/medlem_pids');

if (error.value) {
  console.log('API Error:', error.value.statusCode, error.value.statusMessage);
}
console.log('medlem_pids: ', medlem_pids.value);
const items = [
  {
    label: 'Oplysninger',
    icon: 'i-heroicons-user',
    slot: 'info' // Defines which #slot to render
  },
  {
    label: 'Skift Adgangskode',
    icon: 'i-heroicons-key',
    slot: 'password'
  },

];
</script>

<template>
  <UModal v-model:open="isOpen" title="Profil Indstillinger" size="xxl">
    <template #body>
      <UTabs orientation="horizontal" :items="items" class="w-full h-[400px] gap-8"
        :ui="{ list: { width: 'w-48', tab: { height: 'h-12' } }, container: 'h-full' }">
        <template #info>
          <ProfileOverview :pids="medlem_pids" />
        </template>

        <template #password>
          <ResetForm />
        </template>

      </UTabs>
    </template>
  </UModal>
</template>
