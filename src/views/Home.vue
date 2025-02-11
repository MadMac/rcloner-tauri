<script>
export default {
  methods: {
    goToCopy() {
      this.$router.push('/copy/start')
    },
  },
}
</script>

<template>
  <h1>Choose a command to run</h1>
  <p><v-btn @click="goToCopy">COPY</v-btn></p>
  <p><v-btn disabled>SYNC</v-btn></p>
</template>