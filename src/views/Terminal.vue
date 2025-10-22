<template>
  Terminal asd
  <div v-for="port in ports">
    asd
    {{ port }}
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const ports = ref([]);

onMounted(async () => {
  ports.value = await invoke("list_ports");
});
</script>
