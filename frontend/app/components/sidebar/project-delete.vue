<script setup lang="ts">
import {
  PopoverRoot,
  PopoverTrigger,
  PopoverPortal,
  PopoverContent,
  PopoverArrow,
} from "reka-ui";

const props = defineProps<{ projectName: string }>();
const { $api } = useNuxtApp();

const projectsStore = useProjectsStore();

async function onDelete() {
  try {
    await $api<{ content: string }>(`/projects/${props.projectName}`, {
      method: "DELETE",
    });

    projectsStore.fetch();
  } catch (e) {
    alert(e);
  }
}
</script>

<template>
  <PopoverRoot>
    <PopoverTrigger
      class="flex items-center border-l-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 shadow-md"
    >
      <Icon name="material-symbols:delete-outline-rounded" size="24" />
    </PopoverTrigger>
    <PopoverPortal>
      <PopoverContent
        class="bg-neutral-600 z-50 mx-2 flex flex-col gap-2 p-1 drop-shadow-lg/30"
        side="top"
        :side-offset="5"
      >
        <button class="h-6 cursor-pointer" @click="onDelete">
          <Icon name="material-symbols:delete-outline-rounded" size="24" />
        </button>

        <PopoverArrow class="fill-neutral-600" :height="5" :width="10" />
      </PopoverContent>
    </PopoverPortal>
  </PopoverRoot>
</template>

<style scoped></style>
