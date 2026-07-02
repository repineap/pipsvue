<script setup lang="ts">
import { computed, ref } from "vue";
import { regionToString, RegionType, type Position, type Region } from "./types/domino";

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

const labelMap = computed(() => {
  const map: { [key: string]: string } = {};

  for (const region of regions.value.filter((region) => region.regionType != RegionType.Blank)) {
    const sorted = region.squares.sort((a, b) => Number(a.x > b.x && a.y > b.y));
    const maxSquare = sorted[sorted.length - 1];
    if (maxSquare) {
      map[`${maxSquare!.x}-${maxSquare!.y}`] = regionToString(region);
    }
  }
  console.log(map);
  return map;
});

const gridContainer = ref<HTMLElement | null>(null);

const getRandomHexColor = (): string => {
  const val = ["#ffffff", "#555555"][Math.floor(Math.random() * 2)];
  return val ? val : "#3523324";
};

const inSameRegion = (x: number, y: number, color: string): boolean => {
  return (colorMap.value[y]?.[x] || "EMPTY") !== color;
};

const getDynamicBorder = (x: number, y: number) => {
  const color = colorMap.value[y]?.[x] || "EMPTY";
  if (color === "EMPTY" || color === "#555555") {
    return {};
  }
  const borderStyle = `5px dashed ${color}`;
  return {
    borderTop: inSameRegion(x, y - 1, color) ? borderStyle : undefined,
    borderBottom: inSameRegion(x, y + 1, color) ? borderStyle : undefined,
    borderLeft: inSameRegion(x - 1, y, color) ? borderStyle : undefined,
    borderRight: inSameRegion(x + 1, y, color) ? borderStyle : undefined,
  };
};

defineExpose({
  gridContainer,
});
</script>

<template>
  <div class="grid-container" :style="{ '--cols': columns, '--rows': rows }" ref="gridContainer">
    <template v-for="(row, rowIndex) in grid">
      <template v-for="(col, colIndex) in row" :key="`${rowIndex}-${colIndex}`">
        <div
          v-if="colorMap[rowIndex]?.[colIndex] !== 'EMPTY'"
          class="box"
          :style="[
            {
              '--x': colIndex,
              '--y': rowIndex,
            },
          ]"
        >
          <div
            v-if="labelMap[`${colIndex}-${rowIndex}`]"
            class="region-label"
            :style="{
              background: `${colorMap[rowIndex]?.[colIndex]}`,
            }"
          >
            {{ labelMap[`${colIndex}-${rowIndex}`] }}
          </div>
        </div>
        <div
          v-if="colorMap[rowIndex]?.[colIndex] !== 'EMPTY'"
          class="inner-box"
          :style="[
            { '--x': colIndex, '--y': rowIndex, background: `${colorMap[rowIndex]?.[colIndex]}40` },
            getDynamicBorder(colIndex, rowIndex),
          ]"
        ></div>
      </template>
    </template>
  </div>
</template>

<style scoped>
.grid-container {
  position: absolute;
  top: round(nearest, 40% - (var(--rows) / 2 * var(--domino-size)), var(--domino-size));
  left: round(nearest, 50% - (var(--cols) / 2 * var(--domino-size)), var(--domino-size));
  z-index: -1;
}

.box {
  --padding-size: 10px;
  position: absolute;
  left: calc(var(--x) * var(--domino-size) - var(--padding-size));
  top: calc(var(--y) * var(--domino-size) - var(--padding-size));

  width: calc(var(--domino-size) + 2 * var(--padding-size));
  height: calc(var(--domino-size) + 2 * var(--padding-size));
  background: lightgray;
}

.inner-box {
  position: absolute;

  left: calc(var(--x) * var(--domino-size));
  top: calc(var(--y) * var(--domino-size));

  width: calc(var(--domino-size));
  height: calc(var(--domino-size));

  box-sizing: border-box;
  z-index: 1;
}

.region-label {
  position: relative;
  top: 80%;
  left: 80%;
  width: 20%;
  height: 20%;

  text-align: center;
  color: black;
  font-weight: bold;
  border-radius: 50%;
  z-index: 100;
}
</style>
