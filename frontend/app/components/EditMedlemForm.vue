<script setup lang="ts">
import { z } from "zod";
import { parseDate, type CalendarDate } from "@internationalized/date";

interface MemberData {
  pid: string;
  email?: string | null;
  phoneNr?: string | null;
  address?: string | null;
  city?: string | null;
  name?: string | null;
  birthdate?: string | null;
}

const props = defineProps<{
  member: MemberData;
}>();

const emit = defineEmits(["success", "cancel"]);
const toast = useToast();

const isSaving = ref(false);

const state = reactive({ ...props.member.medlem });

const birthdateDate = computed({
  get() {
    if (!state.birthdate) return undefined;
    return parseDate(state.birthdate);
  },
  set(newDate: CalendarDate | undefined | null) {
    if (!newDate) {
      state.birthdate = "";
      return;
    }
    state.birthdate = newDate.toString();
  },
});

const validate = (state: any) => {
  const errors = [];
  if (!state.email)
    errors.push({ path: "email", message: "Email er påkrævet" });
  return errors;
};

const onSubmit = async () => {
  isSaving.value = true;

  try {
    const payload = {
      medlem_pid: props.member.pid,
      changeable_fields: state,
    };

    await $fetch(`/api/medlem`, {
      method: "PUT",
      body: payload,
    });

    toast.add({
      title: "Gemt!",
      description: "Oplysningerne er opdateret.",
      color: "green",
    });
    emit("success");
  } catch (error: any) {
    console.error(error);
    toast.add({
      title: "Fejl",
      description: error.statusMessage || "Kunne ikke gemme ændringer.",
      color: "red",
    });
  } finally {
    isSaving.value = false;
  }
};
</script>

<template>
  <UForm :state="state" :validate="validate" @submit="onSubmit" class="w-full">
    <div class="w-full max-w-sm mx-auto flex flex-col gap-4 my-2">
      <UFormField label="Navn" name="name">
        <UInput
          v-model="state.name"
          icon="i-heroicons-user"
          placeholder="Hans Hansen"
          class="w-full"
        />
      </UFormField>
      <UFormField label="Email" name="email" required>
        <UInput
          v-model="state.email"
          icon="i-heroicons-envelope"
          placeholder="navn@mail.dk"
          class="w-full"
        />
      </UFormField>

      <UFormField label="Telefonnummer" name="phoneNr">
        <UInput
          v-model="state.phoneNr"
          icon="i-heroicons-phone"
          class="w-full"
          placeholder="+45 12 34 56 78"
        />
      </UFormField>

      <UFormField label="Adresse" name="address">
        <UInput
          v-model="state.address"
          icon="i-heroicons-map-pin"
          class="w-full"
          placeholder="Gadenavn 1"
        />
      </UFormField>

      <UFormField label="By" name="city">
        <UInput
          v-model="state.city"
          icon="i-heroicons-building-office-2"
          class="w-full"
          placeholder="Postnr. By"
        />
      </UFormField>
      <UFormField label="Fødselsdag" name="birthdate">
        <UInputDate
          v-model="birthdateDate"
          icon="i-heroicons-cake"
          class="w-full"
          locale="da-DK"
        />
      </UFormField>
    </div>

    <div class="flex justify-between items-center max-w-sm mx-auto w-full">
      <UButton
        type="submit"
        label="Gem ændringer"
        :loading="isSaving"
        color="primary"
      />
      <UButton
        label="Annuller"
        @click="$emit('cancel')"
        color="gray"
        variant="ghost"
        :disabled="isSaving"
      />
    </div>
  </UForm>
</template>
