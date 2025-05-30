import { useLoginToken } from "~/composables/useLoginToken";

export default defineNuxtRouteMiddleware(() => {
  const token = useLoginToken();

  if (!token.value) {
    return navigateTo("/login");
  }
});
