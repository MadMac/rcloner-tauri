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
  <p><button @click="goToCopy">COPY</button></p>
  <p><button disabled>SYNC</button></p>
</template>