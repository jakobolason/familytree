<script setup lang="ts">
import { z } from 'zod'; // Optional: for nice validation schema if you want

// Define the shape of data we receive
interface MemberData {
  pid: string; // We need the ID for the API call
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

// Local loading state for the save button
const isSaving = ref(false);

// Initialize form state with prop data (converting nulls to empty strings for inputs)
const state = reactive({
  email: props.member.email || '',
  phoneNr: props.member.phone_nr || '',
  address: props.member.address || '',
  city: props.member.city || ''
});

// Simple validation: Email is likely required based on your Rust "Set(email)" vs "Set(Some(phone))"
const validate = (state: any) => {
  const errors = [];
  if (!state.email) errors.push({ path: 'email', message: 'Email er påkrævet' });
  return errors;
};

// --- Submit Handler ---
const onSubmit = async () => {
  isSaving.value = true;

  try {
    // Construct the payload matching your Rust "ChangeableFields" struct
    // We explicitly send the strings. Empty strings might need to be treated as null
    // depending on how strict your backend validation is, but usually sending the string is fine.
    const payload = {
      email: state.email,
      phoneNr: state.phone_nr || null, // Send null if empty to clear it
      address: state.address || null,
      city: state.city || null
    };

    // Perform the API request
    await $fetch(`/api/medlem/${props.member.pid}`, {
      method: 'PUT', // or PATCH/POST depending on your route setup
      body: payload
    });

    toast.add({ title: 'Gemt!', description: 'Oplysningerne er opdateret.', color: 'green' });
    emit('success'); // Tell parent to reload data

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
  <UForm :state="state" :validate="validate" @submit="onSubmit" class="space-y-4">

    <UFormGroup label="Email" name="email" required>
      <UInput v-model="state.email" icon="i-heroicons-envelope" placeholder="navn@mail.dk" />
    </UFormGroup>

    <UFormGroup label="Telefonnummer" name="phoneNr">
      <UInput v-model="state.phoneNr" icon="i-heroicons-phone" placeholder="+45 12 34 56 78" />
    </UFormGroup>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <UFormGroup label="Adresse" name="address">
        <UInput v-model="state.address" icon="i-heroicons-map-pin" placeholder="Gadenavn 1" />
      </UFormGroup>

      <UFormGroup label="By" name="city">
        <UInput v-model="state.city" icon="i-heroicons-building-office-2" placeholder="Postnr. By" />
      </UFormGroup>
    </div>

    <UDivider class="my-6" />

    <div class="flex justify-end gap-3">
      <UButton label="Annuller" color="gray" variant="ghost" @click="$emit('cancel')" :disabled="isSaving" />
      <UButton type="submit" label="Gem ændringer" color="black" :loading="isSaving" />
    </div>

  </UForm>
</template>
