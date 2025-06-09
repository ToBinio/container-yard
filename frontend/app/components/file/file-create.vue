<script setup lang="ts">
import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogOverlay,
  DialogPortal,
  DialogRoot,
  DialogTitle,
  DialogTrigger,
  VisuallyHidden,
} from "reka-ui";

const props = defineProps<{ projectName: string }>();
const { $api } = useNuxtApp();

const projectsStore = useProjectsStore();

const open = ref(false);

const content = ref("");
const name = ref("");

const loading = ref(false);
async function onSaveChanges() {
  loading.value = true;
  try {
    await $api<{ content: string }>(
      `/projects/${props.projectName}?file=${name.value}`,
      {
        body: {
          content: content.value,
        },
        method: "POST",
      },
    );

    content.value = "";
    name.value = "";

    open.value = false;

    projectsStore.fetchProject(props.projectName);
  } catch (e) {
    alert(e);
  }

  loading.value = false;
}
</script>

<template>
  <DialogRoot v-model:open="open">
    <DialogTrigger
      class="flex gap-2 items-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 shadow-md"
    >
      <Icon name="material-symbols:add-rounded" size="24" />
    </DialogTrigger>
    <DialogPortal>
      <DialogOverlay
        class="fixed top-0 left-0 z-20 h-dvh w-dvw backdrop-blur-[1.5px]"
      />
      <DialogContent
        class="bg-neutral-700 fixed top-1/2 left-1/2 z-30 h-96 w-9/10 max-w-128 -translate-x-1/2 -translate-y-1/2 p-1"
      >
        <div class="flex flex-col h-full gap-1">
          <div class="w-full flex justify-between">
            <DialogTitle class="text-xl">
              <input v-model="name" placeholder="Filename" />
            </DialogTitle>
            <DialogClose
              class="aspect-square justify-center flex gap-2 items-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 px-1"
              aria-label="Close"
            >
              X
            </DialogClose>
          </div>
          <VisuallyHidden>
            <DialogDescription>create file</DialogDescription>
          </VisuallyHidden>
          <div
            class="border-neutral-600 w-full flex-1 border-1 pl-1 flex items-center justify-center"
          >
            <textarea v-model="content" class="w-full h-full resize-none" />
          </div>
          <AsyncButton
            :loading="loading"
            :disabled="loading"
            @click="onSaveChanges"
          >
            Create
          </AsyncButton>
        </div>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>

<style scoped></style>
