export default defineNuxtConfig({
  modules: [
    "@nuxt/eslint",
    "@nuxt/ui",
    "@nuxt/hints",
    "@nuxt/image",
    "@nuxt/test-utils",
    "@sidebase/nuxt-auth",
  ],

  devtools: {
    enabled: true,
  },

  css: ["~/assets/css/main.css"],

  routeRules: {
    "/": { prerender: true },
  },

  colorMode: {
    preference: 'light'
  },

  runtimeConfig: {
    baseURL: '/api/auth',
    apiEndpoint: process.env.AUTH_ORIGIN,
    authOrigin: process.env.AUTH_ORIGIN,
    public: {
      baseURL: '/api/auth',
      apiEndpoint: process.env.AUTH_ORIGIN
    }
  },

  auth: {
    isEnabled: true,
    disableServerSideAuth: false,
    // originEnvKey: 'AUTH_ORIGIN',
    // baseUrl: process.env.AUTH_ORIGIN,
    baseURL: '/api/auth',
    globalAppMiddleware: true,
    provider: {
      type: 'local',
      endpoints: {
        signIn: { path: 'api/auth/login', method: 'post' },
        signOut: { path: '/api/auth/logout', method: 'post' },
        getSession: { path: '/api/user/current', method: 'get' },
        signUp: false,
        // getSession: { path: '/user/current', method: 'get' },
      },
      token: {
        signInResponseTokenPointer: '/token',
        type: 'Bearer',
        cookieName: 'auth.token',
        headerName: 'Authorization',
        maxAgeInSeconds: 1800,
        sameSiteAttribute: 'lax',
        //cookieDomain: 'localhost',
        secureCookieAttribute: false,
        httpOnlyCookieAttribute: false,
      },
      sessionRefresh: {
        enablePeriodically: true,
        enableOnWindowFocus: true,
      },
      pages: {
        login: '/login'
      }
    }
  },

  compatibilityDate: "2025-01-15",

  eslint: {
    config: {
      stylistic: {
        commaDangle: "never",
        braceStyle: "1tbs",
      },
    },
  },
});
