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

const route = useRoute();
const project = route.params.project;

const props = defineProps<{ name: string }>();
const config = useRuntimeConfig();

const open = ref(false);
const content = ref("");

watch(open, async () => {
  if (!open.value) return;

  const response = await $fetch<{ content: string }>(
    `/projects/${project}?file=${props.name}`,
    {
      baseURL: config.public.apiURL,
    },
  );

  content.value = response.content;
});

async function onSaveChanges() {
  const response = await $fetch<{ content: string }>(
    `/projects/${project}?file=${props.name}`,
    {
      body: {
        content: content.value,
      },
      baseURL: config.public.apiURL,
      method: "POST",
    },
  );

  content.value = response.content;
}
</script>

<template>
  <div class="flex gap-2">
    <div>
      {{ props.name }}
    </div>
    <DialogRoot v-model:open="open">
      <DialogTrigger class="border-1 px-1 rounded hover:bg-gray-300">
        Edit
      </DialogTrigger>
      <DialogPortal>
        <DialogOverlay
          class="fixed top-0 left-0 z-20 h-dvh w-dvw backdrop-blur-[1.5px]"
        />
        <DialogContent
          class="bg-gray-300 fixed top-1/2 left-1/2 z-30 h-96 w-9/10 max-w-128 -translate-x-1/2 -translate-y-1/2 rounded-lg p-1"
        >
          <div class="flex flex-col h-full gap-1">
            <div class="w-full flex justify-between">
              <DialogTitle class="text-xl">{{ name }}</DialogTitle>
              <DialogClose
                class="h-full bg-gray-400 rounded aspect-square"
                aria-label="Close"
              >
                X
              </DialogClose>
            </div>
            <VisuallyHidden>
              <DialogDescription>{{ name }}</DialogDescription>
            </VisuallyHidden>
            <textarea
              v-model="content"
              class="border-secondary w-full flex-1 resize-none rounded border-1 pl-1"
            />
            <button @click="onSaveChanges">Save</button>
          </div>
        </DialogContent>
      </DialogPortal>
    </DialogRoot>
  </div>
</template>

<style scoped></style>
