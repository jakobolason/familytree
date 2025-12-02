<script setup lang="ts">
import * as z from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";

const {signIn } = useAuth()
const toast = useToast()

const schema = z.object({
  email: z.email("Invalid email"),
  password: z
    .string("Password is required")
    .min(8, "Must be at least 8 characters"),
});

type Schema = z.output<typeof schema>;

const state = reactive<Partial<Schema>>({
  email: undefined,
  password: undefined,
});

async function onSubmit(event: FormSubmitEvent<Schema>) {
  try {
    const result = await signIn({
      email: event.data.email,
      password: event.data.password,
    }, {callbackUrl: '/'});

    if (result?.error) {
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
        title: "Authentication Failed",
        description: "Invalid email or password.",
        color: "error",
      });
    } else {
      toast.add({
        title: "Error",
        description: "An unexpected error occurred. Please try again.",
        color: "error",
      });
      console.error('Login error:', );
      console.log(error);
    }
      }
  console.log(event.data);
}
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
    <UFormField label="Email" name="email">
      <UInput v-model="state.email" />
    </UFormField>

    <UFormField label="Password" name="password">
      <UInput v-model="state.password" type="password" />
    </UFormField>

    <UButton type="submit"> Submit </UButton>
  </UForm>
</template>
