<script setup lang="ts">
let config = useRuntimeConfig();
let {data} = useFetch<{name: string}>("/name", {baseURL: config.public.baseURL});
</script>

<template>
  hi, {{ data?.name ?? "..." }}
</template>

<style scoped>

</style>