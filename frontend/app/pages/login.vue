<script setup lang="ts">
const user = ref("");
const pw = ref("");

const config = useRuntimeConfig();
const token = useLoginToken();

async function onLogin() {
  try {
    const response = await $fetch<{ token: string }>("/auth", {
      baseURL: config.public.apiURL,
      method: "POST",
      body: {
        user: user.value,
        pw: pw.value,
      },
    });

    token.value = response.token;
    navigateTo("/");
  } catch (e) {
    alert(e);
  }
}
</script>

<template>
  <div class="w-dvw h-dvh flex items-center justify-center">
    <div class="border-1 border-neutral-600 p-4 shadow-md">
      <form class="flex flex-col gap-2 w-48" @submit.prevent="onLogin">
        <label class="flex flex-col">
          <span class="text-xl"> User </span>
          <input
            v-model="user"
            class="border-1 border-neutral-600 text-xl pl-1"
          />
        </label>
        <label class="flex flex-col">
          <span class="text-xl"> Password </span>
          <input
            v-model="pw"
            class="border-1 border-neutral-600 text-xl pl-1"
            type="password"
          />
        </label>
        <button
          class="text-xl flex gap-2 items-center justify-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 shadow-md px-1 disabled:bg-neutral-800 data-[loading=true]:text-neutral-500"
        >
          Login
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped></style>
