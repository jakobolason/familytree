

export default defineEventHandler(async (event) => {
  const session = await requireUserSession(event)
  const sessionToken = session.secure?.apiToken

  if (!sessionToken) {
    throw createError({ statusCode: 401, statusMessage: 'No token found' })
  }


  const body = await readBody(event)
  const config = useRuntimeConfig()

  return $fetch(`${config.authOrigin}/api/reset`, {
    method: 'POST',
    body: body,
    headers: {
      Authorization: `Bearer ${sessionToken}`
    }
  })

})
