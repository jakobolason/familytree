<script setup lang="ts">
interface MemberResponse {
  medlem: {
    name: string;
    email?: string | null;
    phone_nr?: string | null;
    address?: string | null;
    city?: string | null;
    birthdate?: string | null;
    finalDate?: string | null;
  }
  parents_pid?: any;
  children_pid?: any;
  partner_pid?: string | null;
  previous_partners?: any;
}

const props = defineProps<{
  member: MemberResponse
}>();

const partnerLabel = computed(() => {
  if (!props.member.partner) return 'Partner';

  if (props.member.partner.finalDate) {
    return 'Afdød partner';
  }

  return 'Partner';
});

</script>

<template>
  <div class="space-y-6">
    <div>
      <MedlemContact :medlem="member.medlem" />
      <div v-if="member.partnerPid">
        <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-3">
          Relationer
        </h3>
        <div>

          <UCard :ui="{ body: { padding: 'p-3' } }" class="col-span-2">
            <div>
              <span class="text-xs text-gray-500 uppercase">{{ partnerLabel }}</span>
              <MedlemContact :medlem="member.partner" />
            </div>
          </UCard>
        </div>
      </div>
    </div>
  </div>
</template>
