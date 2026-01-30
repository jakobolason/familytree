<script setup lang="ts">
const { loggedIn, user, clear } = useUserSession()
const { openProfile } = useProfile()
</script>
<template>
  <div>
    <UHeader>
      <template #left>
        <NuxtLink to="/">
          <!-- <AppLogo class="w-auto h-6 shrink-0" /> -->
          <h1>Start Side</h1>
        </NuxtLink>
      </template>
      <template #right>
        <!--<UColorModeButton />-->
        <UButton to="https://github.com/jakobolason/familytree" target="_blank" icon="i-simple-icons-github"
          aria-label="GitHub" color="neutral" variant="ghost" />
        <!-- The following is for the profile page -->
        <div v-if="!loggedIn">
          <NuxtLink to="/login">
            <UButton color="primary" variant="solid">Log Ind</UButton>
          </NuxtLink>
        </div>
        <div v-else>
          <UAvatar :alt="user.name" size="md" @click.stop="openProfile()"
            class="cursor-pointer hover:ring-2 hover:ring-primary-500 transition-all" />
          <Profile />
        </div>
      </template>
      <template #toggle> <span /> </template>
    </UHeader>

    <slot />

    <USeparator icon="i-simple-icons-nuxtdotjs" />

    <UFooter>
      <template #left>
        <p class="text-sm text-muted">
          Built with Nuxt UI • © {{ new Date().getFullYear() }}
        </p>
      </template>
      <template #right>
        <UButton to="https://github.com/jakobolason" target="_blank" icon="i-simple-icons-github" aria-label="GitHub"
          color="neutral" variant="ghost" />
      </template>
    </UFooter>

    <EditNode />
  </div>
</template>
