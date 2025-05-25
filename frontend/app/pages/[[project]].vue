<script setup lang="ts">
import { FileEntry } from "#components";

const route = useRoute();
const project = route.params.project;

const config = useRuntimeConfig();
const { data } = useFetch<ProjectDetails>("/projects/" + project, {
  baseURL: config.public.apiURL,
});

async function onStart() {
  const response = await $fetch<ProjectDetails>("/projects/start/" + project, {
    baseURL: config.public.apiURL,
    method: "POST",
  });

  data.value = response;
}
async function onStop() {
  const response = await $fetch<ProjectDetails>("/projects/stop/" + project, {
    baseURL: config.public.apiURL,
    method: "POST",
  });

  data.value = response;
}
async function onRestart() {
  const response = await $fetch<ProjectDetails>(
    "/projects/restart/" + project,
    {
      baseURL: config.public.apiURL,
      method: "POST",
    },
  );

  data.value = response;
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <div>
      <h2 class="text-xl">{{ data?.name }}</h2>
      {{ data?.status }}
    </div>

    <div class="flex gap-1">
      <button class="border-1 px-1 rounded hover:bg-gray-300" @click="onStart">
        Start
      </button>
      <button class="border-1 px-1 rounded hover:bg-gray-300" @click="onStop">
        Stop
      </button>
      <button
        class="border-1 px-1 rounded hover:bg-gray-300"
        @click="onRestart"
      >
        Restart
      </button>
    </div>

    <div class="flex flex-col">
      <div class="text-l">Files</div>

      <div v-for="file in data?.files" :key="file">
        <FileEntry :name="file" />
      </div>
    </div>
  </div>
</template>

<style scoped></style>
