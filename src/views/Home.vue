<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();

const goToCopy = () => {
  router.push("/copy/start");
};
</script>

<template>
  <v-container>
    <v-row class="ma-4">
      <h1>Choose a command to run</h1>
    </v-row>
    <v-row class="ma-4">
      <p><v-btn color="primary" @click="goToCopy">COPY</v-btn></p>
    </v-row>
    <v-row class="ma-4">
      <p><v-btn disabled>SYNC</v-btn></p>
    </v-row>
  </v-container>
</template>
