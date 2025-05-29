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

const loadingContent = ref(false);
watch(open, async () => {
  if (!open.value) return;

  loadingContent.value = true;
  try {
    const response = await $fetch<{ content: string }>(
      `/projects/${project}?file=${props.name}`,
      {
        baseURL: config.public.apiURL,
      },
    );

    content.value = response.content;
  } catch (e) {
    alert(e);
  }

  loadingContent.value = false;
});

const loadingUpdate = ref(false);
async function onSaveChanges() {
  loadingUpdate.value = true;
  try {
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
  } catch (e) {
    alert(e);
  }

  loadingUpdate.value = false;
}
</script>

<template>
  <div class="flex gap-2">
    <div>
      {{ props.name }}
    </div>
    <DialogRoot v-model:open="open">
      <DialogTrigger
        class="flex gap-2 items-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 shadow-md px-1"
      >
        Edit
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
              <DialogTitle class="text-xl">{{ name }}</DialogTitle>
              <DialogClose
                class="aspect-square justify-center flex gap-2 items-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 px-1"
                aria-label="Close"
              >
                X
              </DialogClose>
            </div>
            <VisuallyHidden>
              <DialogDescription>{{ name }}</DialogDescription>
            </VisuallyHidden>
            <div
              class="border-neutral-600 w-full flex-1 border-1 pl-1 flex items-center justify-center"
            >
              <textarea
                v-model="content"
                :data-loading="loadingContent"
                class="w-full h-full resize-none data-[loading=true]:blur-[1px] data-[loading=true]:text-neutral-400"
              />
              <Icon
                v-if="loadingContent"
                name="codex:loader"
                class="absolute text-white"
                size="50"
              />
            </div>
            <AsyncButton
              :loading="loadingUpdate"
              :disabled="loadingUpdate"
              @click="onSaveChanges"
            >
              Save
            </AsyncButton>
          </div>
        </DialogContent>
      </DialogPortal>
    </DialogRoot>
  </div>
</template>

<style scoped></style>
