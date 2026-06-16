<script setup lang="ts">
import { ref } from "vue";

const columns = ref(15);
const rows = ref(8);

const gridModel = defineModel<number[][]>({ required: true });

const getRandomHexColor = (): string => {
  const val = ["#ffffff", "#555555"][Math.floor(Math.random() * 2)];
  return val ? val : "#3523324";
};
</script>

<template>
  <div
    class="grid-container"
    :style="{ '--cols': gridModel[0]!.length, '--rows': gridModel!.length }"
  >
    <template v-for="(row, rowIndex) in gridModel">
      <div
        v-for="(col, colIndex) in row"
        :key="`${rowIndex}-${colIndex}`"
        class="box"
        :style="{
          'background-color': '#555555',
          visibility: col === -1 ? 'visible' : 'hidden',
        }"
      ></div>
    </template>
  </div>
</template>

<style scoped>
.grid-container {
  position: absolute;
  top: 100px;
  left: 100px;
  display: grid;
  grid-template-columns: repeat(var(--cols), 1fr);
  grid-template-rows: repeat(var(--rows), 1fr);
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
