<script setup lang="ts">
import * as z from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";

const toast = useToast()
const route = useRoute() // You likely need the token from the URL query params

// 1. Update Schema: Add confirmPassword and use .refine() for equality check
const schema = z.object({
  password: z.string().min(8, "Must be at least 8 characters"),
  confirmPassword: z.string().min(8, "Must be at least 8 characters")
}).refine((data) => data.password === data.confirmPassword, {
  message: "Passwords do not match",
  path: ["confirmPassword"], // This puts the error on the 'confirmPassword' field
});

type Schema = z.output<typeof schema>;

// 2. Update State to match new schema
const state = reactive<Partial<Schema>>({
  password: undefined,
  confirmPassword: undefined,
});

async function onSubmit(event: FormSubmitEvent<Schema>) {
  // NOTE: 'signIn' is for logging in. For resetting a password,
  // you usually make a POST request to your backend with the new password.
  try {
    const result = await $fetch('http://localhost:8086/api/auth/reset', {
      method: 'POST',
      body: {
        password: event.data.password,
        token: route.query.token // Assuming the reset token is in the URL
      }
    });

    toast.add({
      title: "Success",
      description: "Password has been reset successfully.",
      color: "success",
    });

    await navigateTo('/login');

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

    <UFormField label="New Password" name="password">
      <UInput v-model="state.password" type="password" placeholder="Enter new password" />
    </UFormField>

    <UFormField label="Confirm Password" name="confirmPassword">
      <UInput v-model="state.confirmPassword" type="password" placeholder="Repeat new password" />
    </UFormField>

    <UButton type="submit"> Reset Password </UButton>
  </UForm>
</template>
