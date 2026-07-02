<script setup lang="ts">
import DraggableDomino from "./components/DraggableDomino.vue";
import PipsGrid from "./components/PipsGrid.vue";
import { ref } from "vue";
import {
  RegionType,
  Rotation,
  type Domino,
  type DominoPosition,
  type Position,
  type Region,
} from "@/components/types/domino.ts";

// const DOMINO_SIZE = 100;

const dominoes = ref<DominoPosition[]>([]);
// let index = 0;
// for (let i = 0; i <= 9; i++) {
//   for (let j = 0; j <= i; j++) {
//     const newDomino: Domino = {
//       id: index,
//       left: i,
//       right: j,
//       rotation: Rotation.Right,
//     };
//     const newPosition: Position = {
//       x: 100 + (index % 8) * 200,
//       y: 100 + 100 * Math.floor(index / 8),
//     };
//     dominoes.value.push({ domino: newDomino, position: newPosition, isValid: true });
//     index++;
//   }
// }
// while (dominoes.value.length > 5) {
//   dominoes.value.splice(Math.round(Math.random() * dominoes.value.length), 1);
// }

dominoes.value.push({
  domino: { id: 1, left: 0, right: 3, rotation: Rotation.Right },
  position: { x: 300, y: 700 },
  isValid: true,
});

dominoes.value.push({
  domino: { id: 2, left: 5, right: 6, rotation: Rotation.Right },
  position: { x: 500, y: 700 },
  isValid: true,
});

dominoes.value.push({
  domino: { id: 3, left: 3, right: 6, rotation: Rotation.Right },
  position: { x: 700, y: 700 },
  isValid: true,
});

dominoes.value.push({
  domino: { id: 4, left: 4, right: 4, rotation: Rotation.Right },
  position: { x: 900, y: 700 },
  isValid: true,
});

dominoes.value.push({
  domino: { id: 5, left: 3, right: 3, rotation: Rotation.Right },
  position: { x: 1100, y: 700 },
  isValid: true,
});

function hslToHex(h: number, s: number, l: number): string {
  s /= 100;
  l /= 100;

  const a = s * Math.min(l, 1 - l);
  const f = (n: number) => {
    const k = (n + h / 30) % 12;
    const color = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
    return Math.round(255 * color)
      .toString(16)
      .padStart(2, "0");
  };

  return `#${f(0)}${f(8)}${f(4)}`;
}

function generateRandomMutedHex(): string {
  const hue = Math.floor(Math.random() * 361);
  const saturation = Math.floor(Math.random() * 21) + 20; // 20% to 40%
  const lightness = Math.floor(Math.random() * 21) + 45; // 45% to 65%

  return hslToHex(hue, saturation, lightness);
}

const colors: string[] = [];
for (let hue = 0; hue < 360; hue += 15) {
  colors.push(hslToHex(hue, 50, 50));
}

const popRandomColor = (): string => {
  const poppedArray = colors.splice(Math.floor(Math.random() * colors.length), 1);
  return poppedArray[0]!;
};

const regions = ref<Region[]>([]);
regions.value.push({
  squares: [{ x: 2, y: 0 }],
  regionType: RegionType.SumsTo,
  regionValue: 3,
  regionColor: popRandomColor(),
});

regions.value.push({
  squares: [{ x: 1, y: 0 }],
  regionType: RegionType.LessThan,
  regionValue: 6,
  regionColor: popRandomColor(),
});

regions.value.push({
  squares: [
    { x: 0, y: 0 },
    { x: 3, y: 0 },
  ],
  regionType: RegionType.Blank,
  regionValue: -1,
  regionColor: "#555555",
});

regions.value.push({
  squares: [
    { x: 0, y: 1 },
    { x: 0, y: 2 },
    { x: 1, y: 2 },
  ],
  regionType: RegionType.AllEqual,
  regionValue: -1,
  regionColor: popRandomColor(),
});

regions.value.push({
  squares: [
    { x: 2, y: 2 },
    { x: 3, y: 2 },
    { x: 3, y: 3 },
  ],
  regionType: RegionType.SumsTo,
  regionValue: 8,
  regionColor: popRandomColor(),
});

const rows =
  regions.value.reduce(
    (maxVal, region) => Math.max(maxVal, ...region.squares.map((square) => square.y)),
    0,
  ) + 1;
const cols =
  regions.value.reduce(
    (maxVal, region) => Math.max(maxVal, ...region.squares.map((square) => square.x)),
    0,
  ) + 1;

const grid = ref<number[][]>(Array.from({ length: rows }, () => new Array<number>(cols).fill(-1)));
const dominoPlacements: { [key: number]: DominoCoordinate } = {};

const pipsGrid = ref<InstanceType<typeof PipsGrid> | null>(null);

interface DominoCoordinate {
  left: Position;
  right: Position;
}

const calculateGridSquares = (
  centerPosition: Position,
  rotation: Rotation,
  dominoSize: number,
): DominoCoordinate => {
  const gridRect = getGridBoundingBox();
  if (!gridRect) {
    return {
      left: { x: 0, y: 0 },
      right: { x: 0, y: 0 },
    };
  }
  const { x: gridX, y: gridY } = gridRect;
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

const validPosition = (coordinates: Position): boolean => {
  const { x, y } = coordinates;
  return x >= 0 && x < cols && y >= 0 && y < rows;
};

const getGridBoundingBox = (): DOMRect | undefined =>
  pipsGrid.value?.gridContainer?.getBoundingClientRect();

function placeDomino(gridSquares: DominoCoordinate, domino: Domino): boolean {
  const { left, right } = gridSquares;
  if (
    !grid.value ||
    !validPosition(left) ||
    !validPosition(right) ||
    grid.value[left.y]![left.x] != -1 ||
    grid.value[right.y]![right.x] != -1
  ) {
    return true;
  }
  grid.value[left.y]![left.x] = domino.left;
  grid.value[right.y]![right.x] = domino.right;
  return false;
}

const validateRegions = (): boolean => {
  let regionsValid = true;
  for (const region of regions.value.filter((region) => region.regionType != RegionType.Blank)) {
    const values = region.squares
      .map(({ x, y }) => grid.value[y]![x]!)
      .filter((value) => value != -1);
    if (values.length != region.squares.length) {
      region.isValid = false;
      regionsValid = false;
      continue;
    }
    switch (region.regionType) {
      case RegionType.AllDifferent: {
        region.isValid = values.length === new Set(values).size;
        break;
      }
      case RegionType.AllEqual: {
        region.isValid = values.every((value) => value === values[0]);
        break;
      }
      case RegionType.GreaterThan: {
        region.isValid = values.reduce((acc, value) => acc + value, 0) > region.regionValue;
        break;
      }
      case RegionType.LessThan: {
        region.isValid = values.reduce((acc, value) => acc + value, 0) < region.regionValue;
        break;
      }
      case RegionType.SumsTo: {
        region.isValid = values.reduce((acc, value) => acc + value, 0) === region.regionValue;
        break;
      }
    }
    regionsValid = region.isValid || false;
  }
  return regionsValid;
};

function dominoChangedCallback(center: Position, dominoSize: number, dominoIdx: number) {
  const movedDominoPos = dominoes.value[dominoIdx];
  const movedDomino = movedDominoPos?.domino;
  const newPosition = movedDominoPos?.position;
  if (!movedDomino || !newPosition) {
    return;
  }
  const gridSquares = calculateGridSquares(center, movedDomino.rotation, dominoSize);
  const currentDominoPlacement = dominoPlacements[dominoIdx];
  if (currentDominoPlacement) {
    removeDomino(currentDominoPlacement);
  }
  const intersectsAnyOther = placeDomino(gridSquares, movedDomino);
  if (!intersectsAnyOther) {
    dominoPlacements[dominoIdx] = gridSquares;
  }
  movedDominoPos.isValid = !intersectsAnyOther;
  if (validateRegions()) alert("YOU WIN");
}
</script>

<template>
  <header></header>

  <main>
    <DraggableDomino
      v-for="(domino, index) in dominoes"
      v-model="domino!"
      :key="domino.domino.id"
      @dominoChanged="(center, size) => dominoChangedCallback(center, size, index)"
    />
    <PipsGrid ref="pipsGrid" v-model:grid="grid" v-model:regions="regions" />
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
