<script setup lang="ts">
import * as z from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";

const toast = useToast()
const route = useRoute()

const schema = z.object({
  password: z.string().min(8, "Adgangskoden skal være mindst 8 karakterer"),
  confirmPassword: z.string().min(8, "Adgangskoden skal være mindst 8 karakterer")
}).refine((data) => data.password === data.confirmPassword, {
  message: "Adgangskoderne er ikke de samme",
  path: ["confirmPassword"],
});

type Schema = z.output<typeof schema>;

const state = reactive<Partial<Schema>>({
  password: undefined,
  confirmPassword: undefined,
});

async function onSubmit(event: FormSubmitEvent<Schema>) {
  try {
    let response = await $fetch('/api/auth/reset', {
      method: 'POST',
      body: {
        password: event.data.password,
      }
    })

    toast.add({
      title: "Success",
      description: "Password has been reset successfully.",
      color: "success",
    });

    await navigateTo('/');

  } catch (error) {
    toast.add({
      title: "Error",
      description: error.data?.message || "Failed to reset password.",
      color: "error",
    });
  }
}
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">

    <UFormField label="Ny adgangskode" name="password">
      <UInput v-model="state.password" type="password" placeholder="Super sikkert kodeord" />
    </UFormField>

    <UFormField label="Bekræft adgangskode" name="confirmPassword">
      <UInput v-model="state.confirmPassword" type="password" placeholder="Gentage sikre kodeord" />
    </UFormField>

    <UButton type="submit"> Reset Password </UButton>
  </UForm>
</template>
