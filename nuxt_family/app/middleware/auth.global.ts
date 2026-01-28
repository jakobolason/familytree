export default defineNuxtRouteMiddleware((to, from) => {
  const { loggedIn } = useUserSession()

  // 1. Check if the page is marked as public
  // We check 'to.meta.public' (we will define this property later)
  const isPublic = to.meta.public === true

  // 2. If the user is NOT logged in AND the page is NOT public
  if (!loggedIn.value && !isPublic) {
    return navigateTo('/login')
  }
})
