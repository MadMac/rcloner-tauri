<script setup lang="ts">
import { ref, onMounted } from "vue";
import { copyStore } from "../store/copyStore.js";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
const router = useRouter();

const timer = ref();

const startRun = () => {
  console.log("Starting run...");
};

const getLogs = () => {
  if (copyStore.startRun.started) {
    console.log("Getting logs...");
    invoke("get_logs").then((response) => {
      const responseString = response as string;
      console.log(responseString);
      if (responseString.length > 0) {
        console.log("add response");
        copyStore.startRun.output =
          responseString + "\n" + copyStore.startRun.output;
        if (responseString.includes("Rclone process exited with status")) {
          console.log("Rclone process exited");
          copyStore.startRun.started = false;
          copyStore.startRun.finished = true;
        }
      }
    });
  }
};

const stopRun = () => {
  console.log("Stopping dry-run...");
  invoke("stop_rclone").then((response) => {
    console.log(response);
    copyStore.startRun.started = false;
  });
};

onMounted(() => {
  timer.value = setInterval(() => {
    getLogs();
  }, 500);
});
</script>

<template>
  <v-container class="h-screen">
    <v-row class="ma-4">
      <v-btn class="ma-2" variant="flat" @click="router.push('/copy/dry-run')"><v-icon>mdi-arrow-left</v-icon></v-btn>
      <h1>Copy Run</h1>
    </v-row>
    <v-row class="ma-4">
      <v-text-field :placeholder="copyStore.buildCopyRunCommand()" readonly></v-text-field>
    </v-row>
    <v-row class="ma-4">
      <v-btn variant="flat" color="red" @click="startRun()" :disabled="copyStore.startRun.started">
        Start
      </v-btn>
      <v-btn variant="flat" color="red" @click="stopRun()" :disabled="!copyStore.startRun.started"
        class="button-left-margin">
        Stop
      </v-btn>
    </v-row>
    <v-row class="ma-4 h-50">
      <v-textarea label="Dry-run output" v-model="copyStore.startRun.output" readonly></v-textarea>
    </v-row>
  </v-container>
</template>

<style scoped>
html,
body {
  padding: 0;
  margin: 0;
}

.button-left-margin {
  margin-left: 10px;
}
</style>