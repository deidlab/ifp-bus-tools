<template>
  <div class="panel">
    <h1>Serial Terminal</h1>
  </div>

  <div class="flex gap-2 panel">
    <Multiselect
      v-model="selectedPort"
      :options="ports"
      class="custom-multiselect"
      :searchable="false"
      :preselect-first="false"
      :allow-empty="false"
      selected-label=""
      select-label=""
      deselect-label=""
    />
    <button
      type="button"
      class="btn btn-outline-primary"
      :class="portOpened ? 'btn-outline-warning' : 'btn-outline-primary'"
      @click="openPort"
    >
      <span v-if="portOpened">Disconnect</span>
      <span v-else>Connect</span>
    </button>
    <button type="button" class="btn btn-outline-primary" @click="listPorts">
      Refresh
    </button>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Multiselect from "@suadelabs/vue3-multiselect";
import "@suadelabs/vue3-multiselect/dist/vue3-multiselect.css";

const portOpened = ref(false);
const ports = ref<string[]>([]);
const selectedPort = ref();

onMounted(async () => {
  await listPorts();
});

const listPorts = async () => {
  ports.value = await invoke("list_ports");
};

const openPort = async () => {
  try {
    console.log(portOpened.value);
    if (portOpened.value === false) {
      await invoke("open_serial", { port: selectedPort.value, baud: 115200 });
    } else {
      await invoke("close_serial");
    }
    portOpened.value = !portOpened.value;
  } catch (e) {
    console.error(e);
  }
};
</script>
