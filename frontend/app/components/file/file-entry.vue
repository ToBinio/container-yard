<script setup lang="ts">
import { FileDelete } from "#components";
import FileEdit from "./file-edit.vue";

const props = defineProps<{ projectName: string; fileName: string }>();
</script>

<template>
  <div class="flex gap-2">
    <div>
      {{ props.fileName }}
    </div>
    <FileEdit :file-name="props.fileName" :project-name="props.projectName" />
    <FileDelete :file-name="props.fileName" :project-name="props.projectName" />
  </div>
</template>

<style scoped></style>
