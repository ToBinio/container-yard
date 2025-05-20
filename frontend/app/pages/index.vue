<script setup lang="ts">
const config = useRuntimeConfig();
const { data } = useFetch<{ name: string }>("/name", {
  baseURL: config.public.baseURL,
});
</script>

<template>
  <div>hi, {{ data?.name ?? "..." }}</div>
</template>

<style scoped></style>
