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
    const result = await $fetch('/api/magic-link', {
      method: 'POST',
      body: {
        email: event.data.email,
      }
    })

    if (!result?.ok) {
      toast.add({
        title: "Fejl",
        description: result.error || "Der gik noget galt.",
        color: "error",
      });
    } else {
      toast.add({
        title: "Success",
        description: "Du skulle gerne have modtaget en mail, med et magisk link.",
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
