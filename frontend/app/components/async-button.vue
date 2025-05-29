<script setup lang="ts">
const emits = defineEmits<{ click: [] }>();
defineProps<{ loading: boolean; disabled: boolean }>();

function onClick() {
  emits("click");
}
</script>

<template>
  <button
    :disabled="disabled"
    :data-loading="loading"
    class="flex gap-2 items-center justify-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 shadow-md px-1 disabled:bg-neutral-800 data-[loading=true]:text-neutral-500"
    @click="onClick"
  >
    <slot />
    <Icon
      v-if="loading"
      name="codex:loader"
      class="absolute text-white"
      size="28"
    />
  </button>
</template>

<style scoped></style>
