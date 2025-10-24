<template>
  <Setting />
  <!-- wrapper generale a tutta altezza -->
  <div class="flex flex-col h-[calc(100vh-160px)]">
    <!-- blocco superiore (terminal monitor + impostazioni) -->
    <div class="grid grid-cols-3 gap-2 flex-grow overflow-hidden">
      <!-- Area terminale -->
      <div class="col-span-2 flex flex-col">
        <textarea
          id="serialData"
          class="form-textarea resize-none flex-grow w-full mb-1"
          readonly
        ></textarea>
      </div>

      <!-- Pannello impostazioni -->
      <div class="panel flex flex-col gap-2 mb-1 overflow-y-auto">
        <div>
          <label for="portName" class="mb-0">Port</label>
          <div class="flex gap-2">
            <select
              id="portName"
              class="form-select flex-1 disabled:pointer-events-none disabled:bg-[#eee] dark:disabled:bg-[#1b2e4b]"
              v-model="selectedPort"
              v-bind:disabled="portOpened"
            >
              <option v-for="item in ports" :key="item" :value="item">
                {{ item }}
              </option>
            </select>
            <button type="button" class="btn btn-primary" @click="listPorts">
              Refresh
            </button>
          </div>
        </div>

        <div>
          <label for="baudrate" class="mb-0">Baudrate</label>
          <div class="flex gap-2">
            <select
              id="baudrate"
              class="form-select flex-1 disabled:pointer-events-none disabled:bg-[#eee] dark:disabled:bg-[#1b2e4b]"
              v-model="selectedBaud"
              v-bind:disabled="portOpened"
            >
              <option v-for="item in baudrate" :key="item" :value="item">
                {{ item }}
              </option>
            </select>
          </div>
        </div>

        <div>
          <label for="data" class="mb-0">Data</label>
          <div class="flex gap-2">
            <select
              id="data"
              class="form-select flex-1 disabled:pointer-events-none disabled:bg-[#eee] dark:disabled:bg-[#1b2e4b]"
              v-model="selectedDataBit"
              v-bind:disabled="portOpened"
            >
              <option v-for="item in dataBit" :key="item" :value="item">
                {{ item }}
              </option>
            </select>
          </div>
        </div>

        <div>
          <label for="parity" class="mb-0">Parity</label>
          <div class="flex gap-2">
            <select
              id="parity"
              class="form-select flex-1 disabled:pointer-events-none disabled:bg-[#eee] dark:disabled:bg-[#1b2e4b]"
              v-model="selectedParity"
              v-bind:disabled="portOpened"
            >
              <option v-for="item in parity" :key="item" :value="item">
                {{ item }}
              </option>
            </select>
          </div>
        </div>

        <div>
          <label for="stopBit" class="mb-0">Stop</label>
          <div class="flex gap-2">
            <select
              id="stopBit"
              class="form-select flex-1 disabled:pointer-events-none disabled:bg-[#eee] dark:disabled:bg-[#1b2e4b]"
              v-model="selectedStopBit"
              v-bind:disabled="portOpened"
            >
              <option v-for="stop in stopBit" :key="stop" :value="stop">
                {{ stop }}
              </option>
            </select>
          </div>
        </div>

        <div>
          <label for="flowControl" class="mb-0">Flow Control</label>
          <div class="flex gap-2">
            <select
              id="flowControl"
              class="form-select flex-1 disabled:pointer-events-none disabled:bg-[#eee] dark:disabled:bg-[#1b2e4b]"
              v-model="selectedFlow"
              v-bind:disabled="portOpened"
            >
              <option v-for="flow in flowControl" :key="flow" :value="flow">
                {{ flow }}
              </option>
            </select>
          </div>
        </div>

        <button
          type="button"
          class="btn"
          :class="portOpened ? 'btn-warning' : 'btn-primary'"
          @click="openPort"
        >
          <span v-if="portOpened">Disconnect</span>
          <span v-else>Connect</span>
        </button>
      </div>
    </div>

    <!-- blocco inferiore -->
    <div class="grid grid-cols-3 gap-2 mt-2">
      <div class="col-span-2 panel">
        <div class="flex flex-col gap-2">
          <textarea class="form-textarea resize-none" rows="1"></textarea>
        </div>
      </div>
      <div class="panel">asd</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Setting from "@/views/utilities/serial/settings.vue";
import "@suadelabs/vue3-multiselect/dist/vue3-multiselect.css";

const portOpened = ref(false);
const ports = ref<string[]>([]);
const selectedPort = ref();

const baudrate = [9600, 19200];
const selectedBaud = ref(9600);

const dataBit = [5, 6, 7, 8];
const selectedDataBit = ref(8);

const parity = ["None", "Even", "Odd"];
const selectedParity = ref("None");

const stopBit = [1, 2];
const selectedStopBit = ref(1);

const flowControl = ["None", "Software", "Hardware"];
const selectedFlow = ref("None");

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
