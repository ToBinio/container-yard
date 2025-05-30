export default defineNuxtPlugin((nuxtApp) => {
  const token = useLoginToken();
  const config = useRuntimeConfig();

  const api = $fetch.create({
    baseURL: config.public.apiURL,
    onRequest({ options }) {
      if (token.value) {
        options.headers.set("Authorization", `Bearer ${token.value}`);
      }
    },
    async onResponseError({ response }) {
      if (response.status === 401) {
        token.value = undefined;
        await nuxtApp.runWithContext(() => navigateTo("/login"));
      }
    },
  });

  return {
    provide: {
      api,
    },
  };
});
