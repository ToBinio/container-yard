<script setup lang="ts">
import {
  PopoverRoot,
  PopoverTrigger,
  PopoverPortal,
  PopoverContent,
  PopoverArrow,
} from "reka-ui";

const { $api } = useNuxtApp();

const projectsStore = useProjectsStore();

const open = ref(false);

const name = ref("");

const loading = ref(false);
async function onAddNewProject() {
  loading.value = true;
  try {
    await $api<{ content: string }>(`/projects/create/${name.value}`, {
      method: "POST",
    });

    name.value = "";

    open.value = false;

    projectsStore.fetch();
  } catch (e) {
    alert(e);
  }

  loading.value = false;
}
</script>

<template>
  <PopoverRoot v-model:open="open">
    <PopoverTrigger
      class="flex items-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 shadow-md"
    >
      <Icon name="material-symbols:add-rounded" size="24" />
    </PopoverTrigger>
    <PopoverPortal>
      <PopoverContent
        class="bg-neutral-600 z-50 mx-2 flex flex-col gap-2 p-1 drop-shadow-lg/30"
        side="top"
        :side-offset="5"
      >
        <input v-model="name" placeholder="Filename" />
        <button class="h-6 cursor-pointer" @click="onAddNewProject">
          <Icon name="material-symbols:add-rounded" size="24" />
        </button>

        <PopoverArrow class="fill-neutral-600" :height="5" :width="10" />
      </PopoverContent>
    </PopoverPortal>
  </PopoverRoot>
</template>

<style scoped></style>
