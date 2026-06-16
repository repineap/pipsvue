<script setup lang="ts">
import DraggableDomino from "./components/DraggableDomino.vue";
import PipsGrid from "./components/PipsGrid.vue";
import { ref } from "vue";
import {
  Rotation,
  type Domino,
  type DominoPosition,
  type Position,
} from "@/components/types/domino.ts";

// const DOMINO_SIZE = 100;

const dominoes = ref<DominoPosition[]>([]);
let index = 0;
for (let i = 0; i <= 9; i++) {
  for (let j = 0; j <= i; j++) {
    const newDomino: Domino = {
      id: index,
      left: i,
      right: j,
      rotation: Rotation.Right,
    };
    const newPosition: Position = {
      x: 100 + (index % 8) * 200,
      y: 100 + 100 * Math.floor(index / 8),
    };
    dominoes.value.push({ domino: newDomino, position: newPosition, isValid: true });
    index++;
  }
}
while (dominoes.value.length > 10) {
  dominoes.value.splice(Math.round(Math.random() * dominoes.value.length), 1);
}

const rows = 5;
const cols = 10;

const grid = ref<number[][]>(Array.from({ length: rows }, () => new Array<number>(cols).fill(-1)));
const dominoPlacements: { [key: number]: DominoCoordinate } = {};

interface DominoCoordinate {
  left: Position;
  right: Position;
}

const calculateGridSquares = (
  centerPosition: Position,
  rotation: Rotation,
  dominoSize: number,
  gridTopLeft: Position,
): DominoCoordinate => {
  const { x: gridX, y: gridY } = gridTopLeft;
  const { x: centerX, y: centerY } = centerPosition;
  const left = {
    x: Math.floor((centerX - gridX) / dominoSize),
    y: Math.floor((centerY - gridY) / dominoSize),
  };
  const right = {
    x: rotation % 2 == 0 ? left.x + (rotation * -1 + 1) : left.x,
    y: rotation % 2 == 0 ? left.y : left.y + (rotation - 2) * -1,
  };
  return { left, right };
};

function removeDomino(gridSquares: DominoCoordinate) {
  const { left, right } = gridSquares;
  if (!grid.value) {
    return;
  }
  grid.value[left.y]![left.x] = -1;
  grid.value[right.y]![right.x] = -1;
}

function placeDomino(gridSquares: DominoCoordinate): boolean {
  const { left, right } = gridSquares;
  console.log(grid.value, grid.value[left.y]![left.x], grid.value[right.y]![right.x]);
  if (!grid.value || grid.value[left.y]![left.x] != -1 || grid.value[right.y]![right.x] != -1) {
    return true;
  }
  grid.value[left.y]![left.x] = 1;
  grid.value[right.y]![right.x] = 1;
  return false;
}

function dominoChangedCallback(center: Position, dominoSize: number, dominoIdx: number) {
  const movedDominoPos = dominoes.value[dominoIdx];
  const movedDomino = movedDominoPos?.domino;
  const newPosition = movedDominoPos?.position;
  if (!movedDomino || !newPosition) {
    return;
  }
  const gridSquares = calculateGridSquares(center, movedDomino.rotation, dominoSize, {
    x: 100,
    y: 100,
  });
  console.log(gridSquares);
  const currentDominoPlacement = dominoPlacements[dominoIdx];
  if (currentDominoPlacement) {
    removeDomino(currentDominoPlacement);
  }
  const intersectsAnyOther = placeDomino(gridSquares);
  if (!intersectsAnyOther) {
    dominoPlacements[dominoIdx] = gridSquares;
  }
  movedDominoPos.isValid = !intersectsAnyOther;
}
</script>

<template>
  <header>
    <!-- <img alt="Vue logo" class="logo" src="./assets/logo.svg" width="125" height="125" />

    <div class="wrapper">
      <HelloWorld msg="I did it lmao lol!" />
    </div> -->
  </header>

  <main>
    <DraggableDomino
      v-for="(domino, index) in dominoes"
      v-model="domino!"
      :key="domino.domino.id"
      @dominoChanged="(center, size) => dominoChangedCallback(center, size, index)"
    />
    <PipsGrid v-model="grid" />
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
