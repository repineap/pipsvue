<script setup lang="ts">
import { ref } from "vue";

const columns = ref(15);
const rows = ref(8);

const getRandomHexColor = (): string => {
  const val = ["#ffffff", "#555555"][Math.floor(Math.random() * 2)];
  return val ? val : "#3523324";
};

const getRandomVisibility = (): string => {
  return Math.random() <= 0.2 ? "hidden" : "visible";
};
</script>

<template>
  <div class="grid-container" :style="{ '--cols': columns }">
    <div
      v-for="i in columns * rows"
      :key="i"
      class="box"
      :style="{ 'background-color': getRandomHexColor(), visibility: getRandomVisibility() }"
    ></div>
  </div>
</template>

<style scoped>
.grid-container {
  position: absolute;
  top: 100px;
  left: 100px;
  display: grid; /* Initializes the grid context */
  grid-template-columns: repeat(var(--cols), 1fr); /* Creates 3 equal columns */
  z-index: -1;
}

.box {
  /*background-color: var(--color-background-soft);*/
  color: white;
  width: 100px;
  height: 100px;
  border: 5px solid lightgray;
  text-align: center;
  border-radius: 10%;
}
</style>
