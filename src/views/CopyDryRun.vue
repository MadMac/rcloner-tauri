<script setup lang="ts">
import { copyStore } from "../store/copyStore.js";

const startDryRun = () => {
  console.log("Starting dry-run...");
};
</script>

<template>
  <v-container>
    <v-row class="ma-4">
      <v-btn class="ma-2" variant="flat" @click="$router.push('/copy/start')"
        ><v-icon>mdi-arrow-left</v-icon></v-btn
      >
      <h1>Copy Dry-run</h1>
    </v-row>
    <v-row class="ma-4">
      <v-text-field
        label="Source"
        v-model="copyStore.dryRun.command"
        disabled
      ></v-text-field>
    </v-row>
    <v-row class="ma-4">
      <v-btn variant="flat" color="primary" @click="startDryRun()">
        Run dry-run
      </v-btn>
    </v-row>
    <v-row class="ma-4">
      <v-textarea
        label="Dry-run output"
        v-model="copyStore.dryRun.output"
        disabled
      ></v-textarea>
    </v-row>
  </v-container>
</template>
