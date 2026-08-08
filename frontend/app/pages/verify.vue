<script setup lang="ts">
definePageMeta({
  public: true
})
const route = useRoute()
const toast = useToast()
const { fetch } = useUserSession()

async function onSubmit(token: String) {
  try {
    let response = await $fetch('/api/auth/login', {
      method: 'POST',
      body: {
        token: token,
      }
    })

    if (!response?.ok) {
      toast.add({
        title: "Authentication Failed",
        description: response.error || "Invalid magic link.",
        color: "error",
      });
    } else {
      await fetch()
      toast.add({
        title: "Success",
        description: "Du blev logget ind med et magisk link!",
        color: "success",
      });
      await navigateTo('/');
    }
  } catch (error) {
    console.log('error: ', error);
    if (String(error).includes("FetchError")) {
      console.error("Unauthorized credentials")
      toast.add({
        title: "Authentication Failed",
        description: "Invalid magic link.",
        color: "error",
      });
    } else {
      toast.add({
        title: "Error",
        description: "An unexpected error occurred. Please try again.",
        color: "error",
      });
      console.error('Login error:',);
      console.log(error);
    }
  }
}

onMounted(async () => {
  const token = route.query.token
  if (!token) {
    toast.add({ title: 'Error', description: 'No token found in URL', color: 'error' })
    return
  }

  try {
    await onSubmit(String(token))
  } catch (error) {
    console.log('error: ', error);
    toast.add({ title: 'Error', description: 'Invalid or expired link', color: 'error' })
    await navigateTo('/login')
  }
})
</script>

<template>
  <div class="flex flex-col items-center justify-center h-screen">
    <UIcon name="i-heroicons-arrow-path" class="animate-spin w-10 h-10" />
    <p class="mt-4 text-gray-500">Bekræfter dit link...</p>
  </div>
</template>
