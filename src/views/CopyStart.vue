<script setup lang="ts">
import { copyStore } from '../store/copyStore.js'
</script>

<template>
	<h1>Copy</h1>
	<div><v-text-field label="Source" v-model="copyStore.sourcePath"></v-text-field></div>
	<div><v-text-field label="Destination" v-model="copyStore.destinationPath"></v-text-field></div>
</template>