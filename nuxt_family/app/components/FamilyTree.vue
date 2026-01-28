<template>
  <div v-if="status === 'pending'">Loading tree...</div>
  <div v-else-if="error">ERROR: {{ error }}</div>

  <div v-else ref="containerRef" class="w-full h-[600px] md:h-screen bg-gray-50 overflow-hidden relative cursor-move">

    <svg ref="svgRef" width="100%" height="100%">

      <g ref="gRef">

        <g class="links">
          <path v-for="link in links" :key="link.targetId" :d="link.pathD"
            class="fill-none stroke-gray-300 stroke-[1.5px]" />
        </g>

        <g class="nodes">
          <g v-for="node in nodes" :key="node.data.id" class="cursor-pointer"
            :transform="`translate(${node.y},${node.x})`" @click.stop="openEditNode(node)">
            <circle :r="10" class="stroke-blue-500 stroke-[1.5px] transition-colors hover:stroke-black"
              :class="node.data._children ? 'fill-blue-200' : 'fill-white'" />

            <text dy="0.35em" :x="node.children ? -13 : 13" :text-anchor="node.children ? 'end' : 'start'"
              class="text-xs font-medium pointer-events-none select-none">
              {{ node.data.name }}
            </text>
          </g>
        </g>
      </g>
    </svg>

    <UButton @click="resetZoom" class="absolute bottom-4 right-4 p-2 rounded shadow text-sm">
      Reset View
    </UButton>
  </div>
</template>

<script setup lang="ts">
import * as d3 from 'd3';

const { openEditNode } = useEditNode();
const { treeData, nodes, links, status, error } = useFamilyTree();

const containerRef = ref<HTMLElement | null>(null);
const svgRef = ref<SVGElement | null>(null);
const gRef = ref<SVGElement | null>(null);

// Define Zoom Behavior
const zoom = d3.zoom().on('zoom', (event) => {
  if (gRef.value) {
    d3.select(gRef.value).attr('transform', event.transform);
  }
});

function resetZoom() {
  if (!svgRef.value) return;
  const svg = d3.select(svgRef.value);
  svg.transition().duration(750).call(zoom.transform, d3.zoomIdentity.translate(100, 50).scale(1));
}

// When data loads, set up the zoom
watch(nodes, () => {
  if (nodes.value.length > 0 && svgRef.value) {
    const svg = d3.select(svgRef.value);

    // Attach zoom handler
    svg.call(zoom);

    // Initial Position: slightly offset so the root isn't cut off
    svg.call(zoom.transform, d3.zoomIdentity.translate(100, (containerRef.value?.clientHeight || 500) / 2).scale(1));
  }
}, { flush: 'post' }); // 'post' ensures DOM elements exist
</script>

<style scoped>
.link-fade-enter-active,
.link-fade-leave-active {
  transition: opacity 0.4s;
}

.link-fade-enter-from,
.link-fade-leave-to {
  opacity: 0;
}
</style>
