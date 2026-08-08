interface LoginResponse {
  token: string,
  pid: string,
  name: string
}

// This handles both email+password and magicl link
export default defineEventHandler(async (event) => {
  const body = await readBody(event)
  const config = useRuntimeConfig()

  try {
    const loginResponse: LoginResponse = await $fetch(`${config.authOrigin}/api/auth/login`, {
      method: 'POST',
      body: body
    })

    console.log('Login successful for user:', loginResponse.name)
    await setUserSession(event, {
      user: {
        pid: loginResponse.pid,
        name: loginResponse.name,
      },
      secure: {
        apiToken: loginResponse.token
      },
      loggedInAt: new Date(),
    })

    return { ok: true }
  } catch (error: any) {
    console.error('Login error:', error)
    throw createError({
      statusCode: 401,
      statusMessage: 'Invalid login credentials'
    })
  }
})
