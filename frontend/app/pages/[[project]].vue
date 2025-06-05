<script setup lang="ts">
import { AsyncButton, FileEntry } from "#components";

definePageMeta({
  middleware: ["auth"],
  layout: "sidebar",
});

const route = useRoute();
const project = route.params.project;

const { $api } = useNuxtApp();
const { data } = useAPI<ProjectDetails>("/projects/" + project);

const fetching = ref<"start" | "stop" | "restart" | undefined>(undefined);

async function onStart() {
  fetching.value = "start";

  try {
    const response = await $api<ProjectDetails>("/projects/start/" + project, {
      method: "POST",
    });

    data.value = response;
  } catch (e) {
    console.error(e);
  }

  fetching.value = undefined;
}
async function onStop() {
  fetching.value = "stop";
  try {
    const response = await $api<ProjectDetails>("/projects/stop/" + project, {
      method: "POST",
    });

    data.value = response;
  } catch (e) {
    console.error(e);
  }

  fetching.value = undefined;
}
async function onRestart() {
  fetching.value = "restart";
  try {
    const response = await $api<ProjectDetails>(
      "/projects/restart/" + project,
      {
        method: "POST",
      },
    );

    data.value = response;
  } catch (e) {
    console.error(e);
  }

  fetching.value = undefined;
}
</script>

<template>
  <div class="flex flex-col gap-4 p-2">
    <div>
      <h2 class="text-4xl">{{ project }}</h2>

      <div class="flex items-center gap-1">
        <span
          class="inline-block w-4 aspect-square bg-neutral-600 rounded-full"
          :class="{ 'bg-emerald-700!': data?.status == 'running' }"
        />
        {{ data?.status ?? "stopped" }}
      </div>
    </div>

    <div class="flex gap-1">
      <AsyncButton
        :loading="fetching == 'start'"
        :disabled="!!fetching"
        @click="onStart"
      >
        Start
      </AsyncButton>
      <AsyncButton
        :loading="fetching == 'stop'"
        :disabled="!!fetching"
        @click="onStop"
      >
        Stop
      </AsyncButton>
      <AsyncButton
        :loading="fetching == 'restart'"
        :disabled="!!fetching"
        @click="onRestart"
      >
        Restart
      </AsyncButton>
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
