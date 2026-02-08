<script setup lang="ts">
interface MedlemResponse {
  name: string;
  email?: string | null;
  phone_nr?: string | null;
  address?: string | null;
  city?: string | null;
  birthdate?: string | null;
  finalDate?: string | null;
}

const props = defineProps<{
  medlem: MedlemResponse
}>();

// Format dates (e.g., "1990-05-20" -> "20. maj 1990")
const formatDate = (dateStr?: string | null) => {
  if (!dateStr) return null;
  return new Date(dateStr).toLocaleDateString('da-DK', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });
};
// Calculate Age (if alive) or Age at Death
const getAge = () => {
  const birthdate = props.medlem.birthdate;
  if (!birthdate) return null;
  const birth = new Date(birthdate);
  const finalDate = props.medlem.finalDate
  console.log('finalDate', finalDate);
  const end = finalDate ? new Date(finalDate) : new Date();

  let age = end.getFullYear() - birth.getFullYear();
  const m = end.getMonth() - birth.getMonth();
  console.log('calculated m: ', m)
  if (m < 0 || (m === 0 && end.getDate() < birth.getDate())) {
    age--;
  }
  return age;
};

</script>

<template>
  <div class="flex items-start gap-4">
    <UAvatar :alt="medlem.name" size="3xl" class="ring-2 ring-white dark:ring-gray-900 bg-gray-100 dark:bg-gray-800" />
    <div class="flex-1 min-w-0">
      <div class="flex justify-between items-start">
        <div>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white truncate">
            {{ medlem.name }}
          </h2>
          <p v-if="!medlem.finalDate" class="text-sm text-gray-500 dark:text-gray-400 mt-1">
            {{ medlem.city || 'Ingen by angivet' }}
          </p>
        </div>
      </div>
    </div>
  </div>
  <div v-if="!medlem.finalDate" class="space-y-3">
    <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
      Kontakt
    </h3>

    <div class="space-y-2">
      <div class="flex items-center gap-3 text-sm">
        <UIcon name="i-heroicons-envelope" class="w-5 h-5 text-gray-400" />
        <span v-if="medlem.email" class="text-gray-900 dark:text-gray-200">{{ medlem.email }}</span>
        <span v-else class="text-gray-400 italic">Ingen email</span>
      </div>

      <div class="flex items-center gap-3 text-sm">
        <UIcon name="i-heroicons-phone" class="w-5 h-5 text-gray-400" />
        <span v-if="medlem.phoneNr" class="text-gray-900 dark:text-gray-200">{{ medlem.phoneNr
        }}</span>
        <span v-else class="text-gray-400 italic">Intet nummer</span>
      </div>

      <div class="flex items-center gap-3 text-sm">
        <UIcon name="i-heroicons-map-pin" class="w-5 h-5 text-gray-400" />
        <span v-if="medlem.address" class="text-gray-900 dark:text-gray-200">
          {{ medlem.address }}<br v-if="medlem.city">
          {{ medlem.city }}
        </span>
        <span v-else class="text-gray-400 italic">Ingen adresse</span>
      </div>
    </div>
  </div>

  <div class="space-y-3">
    <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
      Tidslinje
    </h3>

    <div class="space-y-4">
      <div class="flex gap-3">
        <div class="flex flex-col items-center">
          <div class="w-2 h-2 rounded-full bg-primary-500 mt-2"></div>
          <div class="w-0.5 h-full bg-gray-200 dark:bg-gray-700" v-if="medlem.finalDate"></div>
        </div>
        <div>
          <p class="text-xs text-gray-500">Født</p>
          <p class="text-sm font-medium">{{ formatDate(medlem.birthdate) || 'Ukendt' }}</p>
        </div>
      </div>

      <div v-if="medlem.finalDate" class="flex gap-3">
        <div class="flex flex-col items-center">
          <div class="w-2 h-2 rounded-full bg-gray-500 mt-2"></div>
        </div>
        <div>
          <p class="text-xs text-gray-500">Død</p>
          <p class="text-sm font-medium">{{ formatDate(medlem.finalDate) }}</p>
        </div>
      </div>

      <div v-if="medlem.birthdate" class="bg-gray-50 dark:bg-gray-800/50 p-2 rounded-md text-xs text-gray-500">
        <span v-if="medlem.finalDate">Blev {{ getAge() }} år gammel.</span>
        <span v-else>Nuværende alder: {{ getAge() }} år.</span>
      </div>
    </div>
  </div>

  <UDivider />

</template>
