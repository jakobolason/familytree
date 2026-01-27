<script setup lang="ts">
 definePageMeta({
  auth: false
  })
const route = useRoute()
const { signIn } = useAuth()
const toast = useToast() // Assuming you use Nuxt UI toast

async function onSubmit(token: String) {
  try {
    const result = await signUp({
      token: token
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



// We run this immediately when the page loads
onMounted(async () => {
  const token = route.query.token

  if (!token) {
    toast.add({ title: 'Error', description: 'No token found in URL', color: 'error' })
    return
  }

  try {
    // 1. Call the 'magic-link' provider we defined in Step 1
    // 2. Pass the token as the credential
    // 3. Set redirect to true and point to your reset page
    await signIn('magic-link', {
      token: token,
      callbackUrl: '/reset',
      redirect: true
    })
  } catch (error) {
    toast.add({ title: 'Error', description: 'Invalid or expired link', color: 'error' })
  }
})
</script>

<template>
  <div class="flex flex-col items-center justify-center h-screen">
    <UIcon name="i-heroicons-arrow-path" class="animate-spin w-10 h-10" />
    <p class="mt-4 text-gray-500">Verifying your link...</p>
  </div>
</template>
