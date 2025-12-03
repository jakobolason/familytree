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

  runtimeConfig: {
    baseURL: '/api/auth',
    public: {
      baseURL: '/api/auth'
    }
  },

  auth: {
    isEnabled: true,
    disableServerSideAuth: false,
    // originEnvKey: 'AUTH_ORIGIN',
    // baseUrl: process.env.AUTH_ORIGIN,
    // baseURL: 'http://localhost:8086/api',
    globalAppMiddleware: true,
    provider: {
      type: 'local',
      endpoints: {
        signIn: { path: '/login', method: 'post' },
        signOut: { path: '/logout', method: 'post' },
        getSession: { path: '/session', method: 'get' },
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
