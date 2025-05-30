export function useLoginToken() {
  return useCookie<string | undefined>("token", {
    maxAge: 60 * 60 * 24 * 30, //1 month
    sameSite: true,
  });
}
