<script setup lang="ts">
import * as z from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";

const toast = useToast()

const schema = z.object({
  email: z.email("Invalid email"),
});

type Schema = z.output<typeof schema>;

const state = reactive<Partial<Schema>>({
  email: undefined,
});

async function onSubmit(event: FormSubmitEvent<Schema>) {
  try {
    const payload = {
      email: event.data.email
    }
    console.log('payload: ', event.data, payload);
    const result = await fetch('http://localhost:8086/api/magic-link', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload)
    })

    console.log('result: ', result);
    if (!result?.ok) {
      toast.add({
        title: "Authentication Failed",
        description: result.error || "Invalid email or password.",
        color: "error",
      });
    } else {
      toast.add({
        title: "Success",
        description: "You have been logged in successfully.",
        color: "success",
      });
      // Optional: redirect after successful login
      await navigateTo('/');
    }
  } catch (error) {
    if (String(error).includes("FetchError")) {
      console.error("Unauthorized credentials")
      toast.add({
        title: "Sending error",
        description: "Invalid email.",
        color: "error",
      });
    } else {
      toast.add({
        title: "Error",
        description: "An unexpected error occurred. Please try again.",
        color: "error",
      });
      console.error('Magic link creation error:');
      console.log(error);
    }
  }
  console.log(event.data);
}
</script>

<template>
  <p> Brug din email, så modtager du en mail du kan logge ind med!</p>
  <UForm :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
    <UFormField label="Email" name="email">
      <UInput v-model="state.email" />
    </UFormField>
    <UButton type="submit"> Submit </UButton>
  </UForm>
</template>
