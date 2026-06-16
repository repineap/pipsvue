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

const DOMINO_SIZE = 100;

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

interface BoundingBox {
  topLeft: Position;
  bottomRight: Position;
}

interface DominoCoordinate {
  left: Position;
  right: Position;
}

function generateBoundingBox(leftTopLeft: Position, rotation: Rotation): BoundingBox {
  const topLeft: Position = { x: 0, y: 0 };
  const bottomRight: Position = { x: 0, y: 0 };
  switch (rotation) {
    case Rotation.Left: {
      topLeft.x = leftTopLeft.x - DOMINO_SIZE;
      topLeft.y = leftTopLeft.y;
      bottomRight.x = leftTopLeft.x + DOMINO_SIZE;
      bottomRight.y = leftTopLeft.y + DOMINO_SIZE;
      break;
    }
    case Rotation.Up: {
      topLeft.x = leftTopLeft.x;
      topLeft.y = leftTopLeft.y - DOMINO_SIZE;
      bottomRight.x = leftTopLeft.x + DOMINO_SIZE;
      bottomRight.y = leftTopLeft.y + DOMINO_SIZE;
      break;
    }
    case Rotation.Right: {
      topLeft.x = leftTopLeft.x;
      topLeft.y = leftTopLeft.y;
      bottomRight.x = leftTopLeft.x + 2 * DOMINO_SIZE;
      bottomRight.y = leftTopLeft.y + DOMINO_SIZE;
      break;
    }
    case Rotation.Down: {
      topLeft.x = leftTopLeft.x;
      topLeft.y = leftTopLeft.y;
      bottomRight.x = leftTopLeft.x + DOMINO_SIZE;
      bottomRight.y = leftTopLeft.y + 2 * DOMINO_SIZE;
      break;
    }
  }
  return {
    topLeft,
    bottomRight,
  };
}

function boundingBoxIntersects(box1: BoundingBox, box2: BoundingBox): boolean {
  if (box1.bottomRight.x <= box2.topLeft.x) return false;
  if (box1.topLeft.x >= box2.bottomRight.x) return false;
  if (box1.bottomRight.y <= box2.topLeft.y) return false;
  if (box1.topLeft.y >= box2.bottomRight.y) return false;

  return true;
}

function intersectsAny(dominoId: number, newRotation: Rotation, newPosition: Position): boolean {
  const boundingBox = generateBoundingBox(newPosition, newRotation);
  for (const dominoPosition of dominoes.value) {
    if (dominoPosition.domino.id == dominoId) {
      continue;
    }
    const otherBoundingBox = generateBoundingBox(
      dominoPosition.position,
      dominoPosition.domino.rotation,
    );
    if (boundingBoxIntersects(boundingBox, otherBoundingBox)) {
      return true;
    }
  }
  return false;
}

function dominoPlacedCallback(dominoIdx: number, center: Position) {
  console.log(center);
  const movedDominoPos = dominoes.value[dominoIdx];
  const movedDomino = movedDominoPos?.domino;
  const newPosition = movedDominoPos?.position;
  if (!movedDomino || !newPosition) {
    return;
  }
  const intersectsAnyOther = intersectsAny(movedDomino.id, movedDomino.rotation, newPosition);
  if (intersectsAnyOther) {
    // switch (movedDomino.rotation) {
    //   case Rotation.Right:
    //     movedDominoPos.position = { x: newPosition.x - 25, y: newPosition.y - 12.5 };
    //     break;
    //   case Rotation.Down:
    //     movedDominoPos.position = { x: newPosition.x - 12.5, y: newPosition.y - 25 };
    //     break;
    //   case Rotation.Left:
    //     movedDominoPos.position = { x: newPosition.x, y: newPosition.y - 12.5 };
    //     break;
    //   case Rotation.Up:
    //     movedDominoPos.position = { x: newPosition.x - 12.5, y: newPosition.y };
    //     break;
    // }
  }
  movedDominoPos.isValid = !intersectsAnyOther;
}

function dominoRotatedCallback(dominoIdx: number, center: Position) {
  console.log(center);
  const movedDominoPos = dominoes.value[dominoIdx];
  const movedDomino = movedDominoPos?.domino;
  const newPosition = movedDominoPos?.position;
  if (!movedDomino || !newPosition) {
    return;
  }
  const intersectsAnyOther = intersectsAny(movedDomino.id, movedDomino.rotation, newPosition);
  if (intersectsAnyOther) {
    // switch (movedDomino.rotation) {
    //   case Rotation.Right:
    //     movedDominoPos.position = { x: newPosition.x - 25, y: newPosition.y - 12.5 };
    //     break;
    //   case Rotation.Down:
    //     movedDominoPos.position = { x: newPosition.x - 12.5, y: newPosition.y - 25 };
    //     break;
    //   case Rotation.Left:
    //     movedDominoPos.position = { x: newPosition.x, y: newPosition.y - 12.5 };
    //     break;
    //   case Rotation.Up:
    //     movedDominoPos.position = { x: newPosition.x - 12.5, y: newPosition.y };
    //     break;
    // }
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
      @dominoPlaced="dominoPlacedCallback(index, $event)"
      @dominoRotated="dominoRotatedCallback(index, $event)"
    />
    <!-- <PipsGrid /> -->
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
