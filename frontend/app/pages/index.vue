<script setup lang="ts">
import { NuxtLink } from "#components";

const config = useRuntimeConfig();
const { data } = useFetch<Project[]>("/projects", {
  baseURL: config.public.apiURL,
});
</script>

<template>
  <div class="p-2">
    <h2 class="text-4xl pb-4">Projects</h2>
    <div class="flex gap-2 flex-row">
      <div v-for="project in data" :key="project.name">
        <NuxtLink
          :href="project.name"
          class="flex gap-2 items-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 text-xl shadow-xl px-1"
        >
          {{ project.name }}

          <span
            class="inline-block w-4 aspect-square bg-emerald-700 rounded-full"
            :class="{ 'bg-neutral-800!': project.status == 'stopped' }"
          />
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
