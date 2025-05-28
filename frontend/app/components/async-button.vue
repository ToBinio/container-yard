<script setup lang="ts">
const emits = defineEmits<{ click: [] }>();
defineProps<{ loading: boolean }>();

function onClick() {
  emits("click");
}
</script>

<template>
  <button
    class="flex gap-2 items-center justify-center border-1 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 shadow-xl px-1"
    @click="onClick"
  >
    <slot />
    <Icon v-if="loading" name="codex:loader" />
  </button>
</template>

<style scoped></style>
