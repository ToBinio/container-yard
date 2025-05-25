<script setup lang="ts">
const config = useRuntimeConfig();
const { data } = useFetch<Project[]>("/projects", {
  baseURL: config.public.apiURL,
});
</script>

<template>
  <div>
    <h2 class="text-2xl">Projects</h2>
    <div v-for="project in data" :key="project.name">
      <a :href="project.name" class="border-1 px-1 rounded hover:bg-gray-300">
        {{ project.name }} - {{ project.status }}
      </a>
    </div>
  </div>
</template>

<style scoped></style>
