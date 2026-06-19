<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { Region } from "./types/domino";

const grid = defineModel<number[][]>("grid", { required: true });
const regions = defineModel<Region[]>("regions", { required: true });

const rows = computed(() => grid.value.length);
const columns = computed(() => (!grid.value[0] ? 0 : grid.value[0].length));

const colorMap = computed(() => {
  const map = Array.from({ length: rows.value }, () =>
    new Array<string>(columns.value).fill("EMPTY"),
  );

  for (const region of regions.value) {
    for (const square of region.squares) {
      if (map[square.y]) {
        map[square.y]![square.x] = region.regionColor;
      }
    }
  }
  return map;
});

const gridContainer = ref<HTMLElement | null>(null);

const getRandomHexColor = (): string => {
  const val = ["#ffffff", "#555555"][Math.floor(Math.random() * 2)];
  return val ? val : "#3523324";
};

defineExpose({
  gridContainer,
});
</script>

<template>
  <div class="grid-container" :style="{ '--cols': columns, '--rows': rows }" ref="gridContainer">
    <template v-for="(row, rowIndex) in grid">
      <div
        v-for="(col, colIndex) in row"
        :key="`${rowIndex}-${colIndex}`"
        class="box"
        :style="{
          'background-color': colorMap[rowIndex]?.[colIndex] || 'EMPTY',
          visibility:
            (colorMap[rowIndex]?.[colIndex] || 'EMPTY') !== 'EMPTY' ? 'visible' : 'hidden',
        }"
      ></div>
    </template>
  </div>
</template>

<style scoped>
.grid-container {
  position: absolute;
  top: round(nearest, 40% - (var(--rows) / 2 * var(--domino-size)), var(--domino-size));
  left: round(nearest, 50% - (var(--cols) / 2 * var(--domino-size)), var(--domino-size));
  display: grid;
  grid-template-columns: repeat(var(--cols), 1fr);
  grid-template-rows: repeat(var(--rows), 1fr);
  z-index: -1;
}

.box {
  /*background-color: var(--color-background-soft);*/
  color: white;
  width: var(--domino-size);
  height: var(--domino-size);
  border: 5px solid lightgray;
  text-align: center;
  border-radius: 10%;
  box-sizing: border-box;
}
</style>
