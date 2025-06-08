<script setup lang="ts">
import { AsyncButton, FileEntry } from "#components";

definePageMeta({
  middleware: ["auth"],
  layout: "sidebar",
});

const route = useRoute();
const projectName = route.params.project as string;

const projectsStore = useProjectsStore();
const project = computed(() => {
  return projectsStore.getByName(projectName);
});

onMounted(async () => {
  await projectsStore.fetchProject(projectName);
});

const { $api } = useNuxtApp();

const fetching = ref<"start" | "stop" | "restart" | undefined>(undefined);

async function onStart() {
  fetching.value = "start";

  try {
    const response = await $api<ProjectDetails>(
      "/projects/start/" + projectName,
      {
        method: "POST",
      },
    );

    projectsStore.setProjectDetails(response);
  } catch (e) {
    console.error(e);
  }

  fetching.value = undefined;
}
async function onStop() {
  fetching.value = "stop";
  try {
    const response = await $api<ProjectDetails>(
      "/projects/stop/" + projectName,
      {
        method: "POST",
      },
    );

    projectsStore.setProjectDetails(response);
  } catch (e) {
    console.error(e);
  }

  fetching.value = undefined;
}
async function onRestart() {
  fetching.value = "restart";
  try {
    const response = await $api<ProjectDetails>(
      "/projects/restart/" + projectName,
      {
        method: "POST",
      },
    );

    projectsStore.setProjectDetails(response);
  } catch (e) {
    console.error(e);
  }

  fetching.value = undefined;
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <div>
      <h2 class="text-4xl">{{ projectName }}</h2>

      <div class="flex items-center gap-1">
        <span
          class="inline-block w-4 aspect-square bg-neutral-600 rounded-full"
          :class="{ 'bg-emerald-700!': project?.status == 'running' }"
        />
        {{ project?.status ?? "stopped" }}
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

      <div v-for="file in project?.files" :key="file">
        <FileEntry :file-name="file" :project-name="projectName" />
      </div>
    </div>
  </div>
</template>

<style scoped></style>
