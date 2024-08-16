<script setup>
import { RouterView } from "vue-router";
</script>

<template>
    <div class="app">
        <RouterView />
    </div>
</template>

<style>
html,
body,
#app,
.app {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100vh;
    max-height: 100vh;
}
</style>
