// The client does now have access to the api key, so this is necessary
export default defineEventHandler(async (event) => {
  const session = await getUserSession(event)

  if (!session.secure?.apiToken) {
    throw createError({ statusCode: 401, statusMessage: 'Unauthorized' })
  }

  const config = useRuntimeConfig()

  return $fetch(`${config.locoUrl}/api/user/current`, {
    headers: {
      Authorization: `Bearer ${session.secure.apiToken}`
    }
  })
})
