<script setup lang="ts">
import { ref, onMounted } from "vue";
import { copyStore } from "../store/copyStore.js";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
const router = useRouter();

const timer = ref();

const startDryRun = () => {
    console.log("Starting dry-run...");
    copyStore.dryRun.output = "";
    copyStore.dryRun.finished = false;
    invoke("run_rclone", {
        sourcePath: copyStore.sourcePath,
        destinationPath: copyStore.destinationPath,
        dryRun: true,
    }).then((response) => {
        console.log(response);
        copyStore.dryRun.started = true;
    });
};

const getLogs = () => {
    if (copyStore.dryRun.started) {
        console.log("Getting logs...");
        invoke("get_logs").then((response) => {
            const responseString = response as string;
            console.log(responseString);
            if (responseString.length > 0) {
                console.log("add response");
                copyStore.dryRun.output = responseString + "\n" + copyStore.dryRun.output;
                if (responseString.includes("Rclone process exited with status")) {
                    console.log("Rclone process exited");
                    copyStore.dryRun.started = false;
                    copyStore.dryRun.finished = true;
                }
            }
        });
    }
};

const stopDryRun = () => {
    console.log("Stopping dry-run...");
    invoke("stop_rclone").then((response) => {
        console.log(response);
        copyStore.dryRun.started = false;
    });
};

const startCopyCommand = () => {
    router.push("/copy/start-run");
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
            <v-btn class="ma-2" variant="flat" @click="router.push('/copy/start')"><v-icon>mdi-arrow-left</v-icon></v-btn>
            <h1>Copy Dry-run</h1>
        </v-row>
        <v-row class="ma-4">
            <v-text-field :placeholder="copyStore.buildCopyDryRunCommand()" readonly></v-text-field>
        </v-row>
        <v-row class="ma-4">
            <v-btn variant="flat" color="primary" @click="startDryRun()" :disabled="copyStore.dryRun.started">Run dry-run</v-btn>
            <v-btn variant="flat" color="primary" @click="stopDryRun()" :disabled="!copyStore.dryRun.started" class="button-left-margin">Stop dry-run</v-btn>
            <v-btn variant="flat" color="green" @click="startCopyCommand()" :disabled="!copyStore.dryRun.finished" class="button-left-margin">Run Copy</v-btn>
        </v-row>
        <v-row class="ma-4 h-50">
            <v-textarea label="Dry-run output" v-model="copyStore.dryRun.output" readonly></v-textarea>
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
