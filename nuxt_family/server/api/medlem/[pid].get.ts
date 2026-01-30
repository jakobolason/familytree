export default defineEventHandler(async (event) => {
  const pid = getRouterParam(event, 'pid') as string
  if (!pid) {
    throw createError({ statusCode: 400, statusMessage: 'No pid provided' })
  }

  const session = await requireUserSession(event)
  const sessionToken = session.secure?.apiToken
  if (!sessionToken) {
    throw createError({ statusCode: 401, statusMessage: 'No token found' })
  }
  const config = useRuntimeConfig()

  try {
    const medlem = await $fetch(`${config.authOrigin}/api/medlem/${pid}`, {
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
