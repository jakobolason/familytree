<script setup lang="ts">
import { z } from 'zod';

interface MemberData {
  pid: string;
  email?: string | null;
  phoneNr?: string | null;
  address?: string | null;
  city?: string | null;
}

const props = defineProps<{
  member: MemberData
}>();
const emit = defineEmits(['success', 'cancel']);
const toast = useToast();

const isSaving = ref(false);

const state = reactive({
  email: props.member.email || '',
  phoneNr: props.member.phoneNr || '',
  address: props.member.address || '',
  city: props.member.city || ''
});

const validate = (state: any) => {
  const errors = [];
  if (!state.email) errors.push({ path: 'email', message: 'Email er påkrævet' });
  return errors;
};

const onSubmit = async () => {
  isSaving.value = true;

  try {
    const payload = {
      medlem_pid: props.member.pid,
      changeable_fields: {
        email: state.email || null,
        phoneNr: state.phoneNr || null,
        address: state.address || null,
        city: state.city || null
      }
    };

    await $fetch(`/api/medlem`, {
      method: 'PUT',
      body: payload
    });

    toast.add({ title: 'Gemt!', description: 'Oplysningerne er opdateret.', color: 'green' });
    emit('success');

  } catch (error: any) {
    console.error(error);
    toast.add({
      title: 'Fejl',
      description: error.statusMessage || 'Kunne ikke gemme ændringer.',
      color: 'red'
    });
  } finally {
    isSaving.value = false;
  }
};
</script>

<template>
  <UForm :state="state" :validate="validate" @submit="onSubmit" class="w-full">

    <div class="w-full max-w-sm mx-auto flex flex-col gap-4 my-2">

      <UFormGroup label="Email" name="email" required>
        <UInput v-model="state.email" icon="i-heroicons-envelope" placeholder="navn@mail.dk" />
      </UFormGroup>

      <UFormGroup label="Telefonnummer" name="phoneNr">
        <UInput v-model="state.phoneNr" icon="i-heroicons-phone" placeholder="+45 12 34 56 78" />
      </UFormGroup>

      <UFormGroup label="Adresse" name="address">
        <UInput v-model="state.address" icon="i-heroicons-map-pin" placeholder="Gadenavn 1" />
      </UFormGroup>

      <UFormGroup label="By" name="city">
        <UInput v-model="state.city" icon="i-heroicons-building-office-2" placeholder="Postnr. By" />
      </UFormGroup>

    </div>


    <div class="flex justify-between items-center max-w-sm mx-auto w-full">
      <UButton type="submit" label="Gem ændringer" :loading="isSaving" color="primary" />
      <UButton label="Annuller" @click="$emit('cancel')" color="gray" variant="ghost" :disabled="isSaving" />
    </div>

  </UForm>
</template>
