

export default defineEventHandler(async (event) => {
  const body = await readBody(event)
  const config = useRuntimeConfig()

  return $fetch(`${config.authOrigin}/api/auth/reset`, {
    method: 'POST',
    body: body
  })

})
