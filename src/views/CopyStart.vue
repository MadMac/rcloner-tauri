<script setup lang="ts">
import { copyStore } from "../store/copyStore.js";
import { useRouter } from "vue-router";
const router = useRouter();

const startCopy = () => {
  // TODO: Validate Paths
  router.push("/copy/dry-run");
};
</script>

<template>
  <v-container>
    <v-row class="ma-4">
      <v-btn class="ma-2" @click="$router.push('/')"
        ><v-icon>mdi-arrow-left</v-icon></v-btn
      >
      <h1>Copy</h1>
    </v-row>
    <v-row class="ma-4">
      <v-text-field
        label="Source"
        v-model="copyStore.sourcePath"
      ></v-text-field>
    </v-row>
    <v-row class="ma-4">
      <v-text-field
        label="Destination"
        v-model="copyStore.destinationPath"
      ></v-text-field>
    </v-row>
    <v-row class="ma-4">
      <v-btn color="primary" @click="startCopy"> Start Copy </v-btn>
    </v-row>
  </v-container>
</template>
