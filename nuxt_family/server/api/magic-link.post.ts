export default defineEventHandler(async (event) => {
  const body = await readBody(event)
  const config = useRuntimeConfig()

  try {
    const magicResponse = await $fetch(`${config.authOrigin}/api/magic-link`, {
      method: 'POST',
      body: body
    })

    return { ok: true }
  } catch (error: any) {
    console.error('Magic link creation error:', error)
    throw createError({
      statusCode: 403,
      statusMessage: 'Magic link creation failed'
    })
  }
})
