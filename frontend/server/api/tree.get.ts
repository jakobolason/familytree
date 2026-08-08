// The client does now have access to the api key, so this is necessary
export default defineEventHandler(async (event) => {
  const session = await requireUserSession(event)
  const sessionToken = session.secure?.apiToken

  if (!sessionToken) {
    throw createError({ statusCode: 401, statusMessage: 'No token found' })
  }
  const config = useRuntimeConfig()

  try {
    const treeData = await $fetch(`${config.authOrigin}/api/user/tree`, {
      headers: {
        Authorization: `Bearer ${sessionToken}`
      }
    })

    return treeData
  } catch (error: any) {
    throw createError({
      statusCode: error.response?.status || 500,
      statusMessage: 'Failed to fetch tree'
    })
  }
})
