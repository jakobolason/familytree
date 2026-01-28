// server/routes/auth/verify.get.ts
export default defineEventHandler(async (event) => {
  const { token } = getQuery(event)
  const config = useRuntimeConfig()

  if (!token) {
    throw createError({ statusCode: 400, statusMessage: 'Missing token' })
  }

  try {
    // 1. Call Loco to verify the token and get the user
    // Adjust endpoint based on your Loco routes (e.g., /api/auth/verify)
    const backendResponse = await $fetch(`${config.locoUrl}/api/auth/login`, {
      method: 'POST',
      body: { token }
    })

    // 2. Setup the user session in Nuxt
    // 'backendResponse' should contain user info and ideally a persistent API Token/JWT from Loco
    await setUserSession(event, {
      user: {
        pid: backendResponse.pid,
        name: backendResponse.name
      },
      // Important: Store the Loco API Token in the secure session
      // so you can use it for future authenticated requests
      secure: {
        apiToken: backendResponse.token
      }
    })

    // 3. Redirect to dashboard
    return sendRedirect(event, '/dashboard')

  } catch (error) {
    // Redirect to login with error if token is invalid
    return sendRedirect(event, '/login?error=invalid_token')
  }
})
