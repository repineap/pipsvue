<template>
  <svg viewBox="0 0 200 100" width="100%" height="100%">
    <rect
      x="0"
      y="0"
      width="200"
      height="100"
      class="bg"
      :class="`${isValid ? 'valid-bg' : 'invalid-bg'}`"
    />

    <line class="line" x1="100" y1="10" x2="100" y2="90" />

    <g class="pip">
      <circle
        v-for="(pip, index) in leftPipCoordinates"
        :key="'left-' + index"
        :cx="pip.cx"
        :cy="pip.cy"
        r="10"
      />
    </g>

    <g class="pip">
      <circle
        v-for="(pip, index) in rightPipCoordinates"
        :key="'right-' + index"
        :cx="pip.cx"
        :cy="pip.cy"
        r="10"
      />
    </g>
  </svg>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  left: number;
  right: number;
}>();

interface Coordinates {
  cx: number;
  cy: number;
}

const isValid = defineModel<boolean>();

const pipMap: Record<number, Coordinates[]> = {
  0: [],
  1: [{ cx: 50, cy: 50 }],
  2: [
    { cx: 25, cy: 25 },
    { cx: 75, cy: 75 },
  ],
  3: [
    { cx: 25, cy: 25 },
    { cx: 50, cy: 50 },
    { cx: 75, cy: 75 },
  ],
  4: [
    { cx: 25, cy: 25 },
    { cx: 25, cy: 75 },
    { cx: 75, cy: 25 },
    { cx: 75, cy: 75 },
  ],
  5: [
    { cx: 25, cy: 25 },
    { cx: 25, cy: 75 },
    { cx: 50, cy: 50 },
    { cx: 75, cy: 25 },
    { cx: 75, cy: 75 },
  ],
  6: [
    { cx: 25, cy: 25 },
    { cx: 25, cy: 50 },
    { cx: 25, cy: 75 },
    { cx: 75, cy: 25 },
    { cx: 75, cy: 50 },
    { cx: 75, cy: 75 },
  ],
  7: [
    { cx: 25, cy: 25 },
    { cx: 25, cy: 50 },
    { cx: 25, cy: 75 },
    { cx: 75, cy: 25 },
    { cx: 75, cy: 50 },
    { cx: 75, cy: 75 },
    { cx: 50, cy: 50 },
  ],
  8: [
    { cx: 25, cy: 25 },
    { cx: 25, cy: 50 },
    { cx: 25, cy: 75 },
    { cx: 75, cy: 25 },
    { cx: 75, cy: 50 },
    { cx: 75, cy: 75 },
    { cx: 50, cy: 25 },
    { cx: 50, cy: 75 },
  ],
  9: [
    { cx: 25, cy: 25 },
    { cx: 25, cy: 50 },
    { cx: 25, cy: 75 },
    { cx: 75, cy: 25 },
    { cx: 75, cy: 50 },
    { cx: 75, cy: 75 },
    { cx: 50, cy: 25 },
    { cx: 50, cy: 75 },
    { cx: 50, cy: 50 },
  ],
};

const leftPipCoordinates = computed(() => {
  return pipMap[props.left] || [];
});

const rightPipCoordinates = computed(() => {
  const pips = pipMap[props.right] || [];
  return pips.map((pip) => ({
    cx: pip.cx + 100,
    cy: pip.cy,
  }));
});
</script>

<style scoped>
.bg {
  opacity: 0.9;
  fill: #ffffff;
  stroke: #000000;
  stroke-width: 4;
  rx: 12;
}
.line {
  stroke: #000000;
  stroke-width: 2;
}
.pip {
  fill: #000000;
}

.valid-bg {
  stroke: #000000;
}

@media (prefers-color-scheme: light) {
  .bg {
    opacity: 0.9;
    fill: #202120;
    stroke: #d3d3d3;
    stroke-width: 4;
    rx: 12;
  }
  .line {
    stroke: #d3d3d3;
    stroke-width: 2;
  }
  .pip {
    fill: #d3d3d3;
  }

  .valid-bg {
    stroke: #d3d3d3;
  }
}

.invalid-bg {
  stroke: darkred;
}
</style>
