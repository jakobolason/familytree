<script setup lang="ts">
interface MedlemResponse {
  name: string;
  email?: string | null;
  phone_nr?: string | null;
  address?: string | null;
  city?: string | null;
  birthdate?: string | null;
  finalDate?: string | null;
  status: string;
  parents_pid?: any;
  children_pid?: any;
  partner_pid?: string | null;
  previous_partners?: any;
}

const props = defineProps<{
  member: MedlemResponse
}>();

// --- Helper Functions ---

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
  if (!props.member.birthdate) return null;
  const birth = new Date(props.member.birthdate);
  console.log('finalDate', props.member.finalDate);
  const end = props.member.finalDate ? new Date(props.member.finalDate) : new Date();

  let age = end.getFullYear() - birth.getFullYear();
  const m = end.getMonth() - birth.getMonth();
  if (m < 0 || (m === 0 && end.getDate() < birth.getDate())) {
    age--;
  }
  return age;
};


// Helper to count relationships if they are arrays
const getCount = (jsonVal: any) => {
  if (Array.isArray(jsonVal)) return jsonVal.length;
  return 0;
};
</script>

<template>
  <div class="space-y-6">

    <div class="flex items-start gap-4">
      <UAvatar :alt="member.name" size="3xl"
        class="ring-2 ring-white dark:ring-gray-900 bg-gray-100 dark:bg-gray-800" />
      <div class="flex-1 min-w-0">
        <div class="flex justify-between items-start">
          <div>
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white truncate">
              {{ member.name }}
            </h2>
            <p v-if="!member.finalDate" class="text-sm text-gray-500 dark:text-gray-400 mt-1">
              {{ member.city || 'Ingen by angivet' }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <UDivider />

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

      <div v-if="!member.finalDate" class="space-y-3">
        <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
          Kontakt
        </h3>

        <div class="space-y-2">
          <div class="flex items-center gap-3 text-sm">
            <UIcon name="i-heroicons-envelope" class="w-5 h-5 text-gray-400" />
            <span v-if="member.email" class="text-gray-900 dark:text-gray-200">{{ member.email }}</span>
            <span v-else class="text-gray-400 italic">Ingen email</span>
          </div>

          <div class="flex items-center gap-3 text-sm">
            <UIcon name="i-heroicons-phone" class="w-5 h-5 text-gray-400" />
            <span v-if="member.phoneNr" class="text-gray-900 dark:text-gray-200">{{ member.phoneNr }}</span>
            <span v-else class="text-gray-400 italic">Intet nummer</span>
          </div>

          <div class="flex items-center gap-3 text-sm">
            <UIcon name="i-heroicons-map-pin" class="w-5 h-5 text-gray-400" />
            <span v-if="member.address" class="text-gray-900 dark:text-gray-200">
              {{ member.address }}<br v-if="member.city">
              {{ member.city }}
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
              <div class="w-0.5 h-full bg-gray-200 dark:bg-gray-700" v-if="member.finalDate"></div>
            </div>
            <div>
              <p class="text-xs text-gray-500">Født</p>
              <p class="text-sm font-medium">{{ formatDate(member.birthdate) || 'Ukendt' }}</p>
            </div>
          </div>

          <div v-if="member.finalDate" class="flex gap-3">
            <div class="flex flex-col items-center">
              <div class="w-2 h-2 rounded-full bg-gray-500 mt-2"></div>
            </div>
            <div>
              <p class="text-xs text-gray-500">Død</p>
              <p class="text-sm font-medium">{{ formatDate(member.finalDate) }}</p>
            </div>
          </div>

          <div v-if="member.birthdate" class="bg-gray-50 dark:bg-gray-800/50 p-2 rounded-md text-xs text-gray-500">
            <span v-if="member.finalDate">Blev {{ getAge() }} år gammel.</span>
            <span v-else>Nuværende alder: {{ getAge() }} år.</span>
          </div>
        </div>
      </div>
    </div>

    <UDivider />
    <!--
    <div>
      <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-3">
        Relationer
      </h3>

      <div class="grid grid-cols-2 gap-4">
        <UCard :ui="{ body: { padding: 'p-3' } }">
          <div class="flex justify-between items-center">
            <span class="text-sm text-gray-500">Forældre</span>
            <UBadge variant="soft" color="white">{{ getCount(member.parents_pid) }}</UBadge>
          </div>
        </UCard>

        <UCard :ui="{ body: { padding: 'p-3' } }">
          <div class="flex justify-between items-center">
            <span class="text-sm text-gray-500">Børn</span>
            <UBadge variant="soft" color="white">{{ getCount(member.children_pid) }}</UBadge>
          </div>
        </UCard>

        <UCard v-if="member.partner_pid" :ui="{ body: { padding: 'p-3' } }" class="col-span-2">
          <div class="flex flex-col gap-1">
            <span class="text-xs text-gray-500 uppercase">Nuværende Partner ID</span>
            <span class="font-mono text-xs truncate">{{ member.partner_pid }}</span>
          </div>
        </UCard>
      </div>
    </div>
    -->

  </div>
</template>
