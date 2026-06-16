<script setup lang="ts">
import { ref, computed, watch, nextTick } from "vue";
import { useDraggable } from "@vueuse/core";
import { Rotation, type DominoPosition, type Position } from "@/components/types/domino.ts";
import DominoSVG from "@/components/DominoSVG.vue";
const dominoModel = defineModel<DominoPosition>({
  required: true,
});

const domino = computed({
  get: () => dominoModel.value.domino,
  set: (val) => {
    dominoModel.value = { ...dominoModel.value, domino: val };
  },
});

const el = ref<HTMLElement | null>(null);
const centerDot = ref<HTMLElement | null>(null);
const rotationVal = ref(0);

const snap = (val: number) => Math.round(val / 100) * 100;

const wasDragged = ref(false);

const emit = defineEmits<{
  dominoPlaced: [Position];
  dominoRotated: [Position];
}>();

let startClientPosition: Position = {
  x: 0,
  y: 0,
};

const { style, position } = useDraggable(el, {
  initialValue: dominoModel.value.position,
  onStart(position, event) {
    startClientPosition = { x: event.clientX, y: event.clientY };
    wasDragged.value = false;
  },
  async onEnd(position, event) {
    const deltaX = Math.abs(event.clientX - startClientPosition.x);
    const deltaY = Math.abs(event.clientY - startClientPosition.y);

    if (deltaX > 2 || deltaY > 2) {
      wasDragged.value = true;
    } else {
      domino.value.rotation = (domino.value.rotation + 1) % 4;
      rotationVal.value += 1;
    }
    const snapped_position: Position = {
      x: snap(position.x),
      y: snap(position.y),
    };
    dominoModel.value.position = { ...snapped_position };
    await nextTick();
    if (wasDragged.value) {
      emit("dominoPlaced", calculateCenter());
    } else {
      emit("dominoRotated", calculateCenter());
    }
  },
});

watch(
  () => dominoModel.value.position,
  (newParentPos) => {
    position.value = { x: newParentPos.x, y: newParentPos.y };
  },
  { deep: true },
);

const calculateCenter = (): Position => {
  const center = {
    x: 0,
    y: 0,
  };
  if (!centerDot.value) {
    return center;
  }
  const { x, y } = centerDot.value.getBoundingClientRect();
  return {
    x,
    y,
  };
  // const { x, y } = position;
  // const { width, height } = dominoElement.getBoundingClientRect();
  // console.log(x, y, width, height);
  // if (domino.value.rotation == Rotation.Right || domino.value.rotation == Rotation.Left) {
  //   center.x = x + width / 4;
  //   center.y = x + height / 2;
  // } else {
  //   center.x = x + width / 4;
  //   center.y = x + height / 2;
  // }
};

const handleClick = () => {
  if (wasDragged.value) {
    wasDragged.value = false;
    return;
  }
};
</script>

<template>
  <div
    ref="el"
    :style="[
      style,
      `opacity: ${dominoModel.isValid ? 0.9 : 1}`,
      `z-index: ${dominoModel.isValid ? 0 : 9999}`,
    ]"
    @click="handleClick"
    class="draggable-wrapper"
  >
    <div
      ref="dominoDiv"
      class="domino"
      :class="{ 'invalid-domino': !dominoModel.isValid }"
      :style="{ transform: `rotate(${rotationVal * 90}deg)` }"
    >
      <DominoSVG
        :left="dominoModel.domino.left"
        :right="dominoModel.domino.right"
        v-model="dominoModel.isValid"
        alt="Domino"
        width="100%"
        height="100%"
        style="user-select: none; -webkit-user-drag: none"
      />
      <div ref="centerDot" style="position: fixed; top: 50%; left: 25%"></div>
    </div>
  </div>
</template>

<style scoped>
.domino {
  width: 200px;
  height: 100px;
  transform-origin: 25% 50%;
  transition: transform 0.2s ease;
  position: fixed;
  cursor: pointer;
  user-select: none;
}

.draggable-wrapper {
  position: absolute;
  cursor: grab;
  user-select: none;
  touch-action: none;
}
</style>
