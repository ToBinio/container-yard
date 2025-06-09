<script setup lang="ts">
import { NuxtLink } from "#components";
import ProjectCreate from "~/components/sidebar/project-create.vue";

const projectsStore = useProjectsStore();

await callOnce(async () => {
  await projectsStore.fetch();
});
</script>

<template>
  <div class="flex">
    <div class="bg-neutral-700 h-dvh p-1 min-w-48">
      <h2 class="text-2xl pb-2 flex gap-2 items-center">
        Projects
        <ProjectCreate />
      </h2>
      <div class="flex gap-2 flex-col">
        <div v-for="project in projectsStore.data" :key="project.name">
          <NuxtLink
            :to="project.name"
            class="flex gap-2 items-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 text-xl shadow-md px-1"
            active-class="bg-neutral-600!"
          >
            <span
              class="inline-block w-4 aspect-square bg-emerald-700 rounded-full"
              :class="{ 'bg-neutral-800!': project.status == 'stopped' }"
            />
            {{ project.name }}
          </NuxtLink>
        </div>
      </div>
    </div>
    <div class="p-2">
      <slot />
    </div>
  </div>
</template>

<style scoped></style>
