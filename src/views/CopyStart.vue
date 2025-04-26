<script setup lang="ts">
import { onMounted } from "vue";
import { copyStore } from "../store/copyStore.js";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
const router = useRouter();

onMounted(() => {
    copyStore.sourceError = false;
    copyStore.destinationError = false;
});

const startCopy = () => {
    const sourcePromise = checkPathExists(copyStore.sourcePath);
    const destinationPromise = checkPathExists(copyStore.destinationPath);

    Promise.all([sourcePromise, destinationPromise])
        .then((results) => {
            if (results.every((result) => result === true)) {
                router.push("/copy/dry-run");
            } else {
                console.log("One or more paths do not exist", results);
                if (results[0]) {
                    copyStore.sourceError = false;
                } else {
                    copyStore.sourceError = true;
                }

                if (results[1]) {
                    copyStore.destinationError = false;
                } else {
                    copyStore.destinationError = true;
                }
            }
        })
        .catch((error) => {
            console.error(error);
        });
};

const checkPathExists = async (path: string) => {
    return invoke("check_if_path_exists", { path })
        .then((result) => {
            return result;
        })
        .catch((error) => {
            console.error(error);
            return false;
        });
};
</script>

<template>
    <v-container>
        <v-row class="ma-4">
            <v-btn class="ma-2" @click="router.push('/')"><v-icon>mdi-arrow-left</v-icon></v-btn>
            <h1>Copy</h1>
        </v-row>
        <v-row class="ma-4">
            <v-text-field
                label="Source"
                v-model="copyStore.sourcePath"
                :error="copyStore.sourceError"
                :error-messages="copyStore.sourceError ? ['Path does not exist'] : []"
            ></v-text-field>
        </v-row>
        <v-row class="ma-4">
            <v-text-field
                label="Destination"
                v-model="copyStore.destinationPath"
                :error="copyStore.destinationError"
                :error-messages="copyStore.destinationError ? ['Path does not exist'] : []"
            ></v-text-field>
        </v-row>
        <v-row class="ma-4">
            <v-btn color="primary" @click="startCopy"> Start Copy </v-btn>
        </v-row>
    </v-container>
</template>
