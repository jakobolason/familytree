export default defineEventHandler(async (event) => {
  const session = await requireUserSession(event)
  const sessionToken = session.secure?.apiToken
  if (!sessionToken) {
    throw createError({ statusCode: 401, statusMessage: 'No token found' })
  }
  const config = useRuntimeConfig()

  try {
    const medlem = await $fetch(`${config.authOrigin}/api/user/medlem_pids`, {
      headers: {
        Authorization: `Bearer ${sessionToken}`
      }
    })

    return medlem
  } catch (error: any) {
    console.error('Error fetching medlem:', error)
    throw createError({
      statusCode: error.response?.status || 500,
      statusMessage: 'Failed to fetch medlem'
    })
  }
})
