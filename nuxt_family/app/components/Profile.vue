<script setup lang="ts">
const { isOpen } = useProfile();

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
  <UModal v-model:open="isOpen" title="Profil Indstillinger" size="xl">
    <template #body>
      <UTabs orientation="vertical" :items="items" class="w-full h-[400px] gap-8"
        :ui="{ list: { width: 'w-48', tab: { height: 'h-12' } }, container: 'h-full' }">
        <template #info>
          <div class="space-y-4">
            <h2 class="text-lg font-semibold">Personlige Oplysninger</h2>
            <p class="text-gray-500">Formular til navn, email osv.</p>
          </div>
        </template>

        <template #password>
          <ResetForm />
        </template>

      </UTabs>
    </template>
  </UModal>
</template>
