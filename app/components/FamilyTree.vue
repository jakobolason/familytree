<template>
  <div class="d3-container">
    <svg :width="width" :height="height">
      <g :transform="`translate(${margin.left},${margin.top})`">
        <TransitionGroup name="link-fade" tag="g">
          <path
            v-for="link in links"
            :key="link.targetId"
            :d="link.pathD"
            class="link"
          />
        </TransitionGroup>

        <TransitionGroup
          name="node-transition"
          tag="g"
          @before-enter="onNodeBeforeEnter"
          @enter="onNodeEnter"
          @before-leave="onNodeBeforeLeave"
          @leave="onNodeLeave"
        >
          <g
            v-for="node in nodes"
            :key="node.data.id"
            class="node"
            :data-x="node.x"
            :data-y="node.y"
            :transform="`translate(${node.y},${node.x})`"
          >
            <circle
              :r="nodeRadius"
              stroke="steelblue"
              stroke-width="1.5"
              :fill="node._children ? 'lightsteelblue' : '#fff'"
              @click.stop="openNode(node)"
              class="node-circle"
            />

            <text
              dy="0.35em"
              :x="node.children || node._children ? -13 : 13"
              :text-anchor="node.children || node._children ? 'end' : 'start'"
              @click.stop="openNode(node)"
              class="node-text"
            >
              {{ node.data.name }}
            </text>
          </g>
        </TransitionGroup>
      </g>
    </svg>
  </div>
</template>

<script setup>
import * as d3 from "d3";
import { ref, onMounted } from "vue";
import { familyTreeData } from "../../shared/lib/treeData.js";

// --- STATE ---
const rawData = ref(structuredClone(familyTreeData)); // Deep copy to prevent HMR issues
const nodes = ref([]);
const links = ref([]);
const selectedNode = ref(null); // The node currently being edited

// Config
const width = 1000;
const height = 600;
const margin = { top: 50, right: 90, bottom: 50, left: 90 };
const nodeRadius = 10;
const duration = 400;

// Internal D3 helpers
const treeLayout = d3
  .tree()
  .size([
    height - margin.top - margin.bottom,
    width - margin.left - margin.right - 100,
  ]);
const diagonal = d3
  .linkHorizontal()
  .x((d) => d.y)
  .y((d) => d.x);
const previousPositions = new Map();
let nodeIdCounter = 0;

// --- ACTIONS ---

// 1. Open Side Panel
function openNode(node) {
  selectedNode.value = node;
}

// 2. Close Side Panel
function closeNode() {
  selectedNode.value = null;
}

// 3. Save (Since we v-model directly to .data, it updates visually instantly)
function saveAndClose() {
  // Here you would typically trigger an API call to save to backend
  console.log("Saving:", selectedNode.value.data);
  closeNode();
}

// --- CORE LAYOUT ---
function calculateTreeLayout() {
  const root = d3.hierarchy(rawData.value[0]);

  // Ensure IDs
  root.descendants().forEach((d) => {
    if (!d.data.id) d.data.id = `gen-${nodeIdCounter++}`;
  });

  treeLayout(root);

  nodes.value.forEach((n) =>
    previousPositions.set(n.data.id, { x: n.x, y: n.y }),
  );

  nodes.value = root.descendants();
  links.value = root.links().map((link) => ({
    pathD: diagonal(link),
    targetId: link.target.data.id,
  }));
}

// --- TRANSITION HOOKS (Minimised for brevity) ---
function onNodeBeforeEnter(el) {
  const id = el.getAttribute("key"); // Vue passes key as attribute usually
  // Find parent's previous position to sprout from
  const node = nodes.value.find((n) => n.data.id == id);
  let x = margin.top,
    y = margin.left;

  if (node && node.parent) {
    const prev = previousPositions.get(node.parent.data.id);
    if (prev) {
      x = prev.x;
      y = prev.y;
    }
  }
  d3.select(el).attr("transform", `translate(${y},${x})`);
}

function onNodeEnter(el, done) {
  d3.select(el)
    .transition()
    .duration(duration)
    .attr("transform", `translate(${el.dataset.y},${el.dataset.x})`)
    .on("end", done);
}

function onNodeBeforeLeave(el) {
  // No-op needed usually, handled in leave
}

function onNodeLeave(el, done) {
  const id = el.getAttribute("key"); // If using Vue 3 key might need to be accessed differently depending on setup
  // Find where to shrink to (the parent's new position)
  // Simply fading out at current position is often smoother and less code than calculating parent target
  d3.select(el)
    .transition()
    .duration(duration)
    .style("opacity", 0)
    .on("end", done);
}

onMounted(() => {
  calculateTreeLayout();
});
</script>

<style scoped>
.tree-wrapper {
  position: relative;
  height: 600px;
  overflow: hidden;
  font-family: sans-serif;
}
.d3-container {
  width: 100%;
  height: 100%;
}

/* Node Styling */
.node {
  cursor: pointer;
}
.node-circle {
  transition: fill 0.3s;
}
.node-circle:hover {
  stroke: #000;
  stroke-width: 2px;
}
.node-text {
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
}
.node-text:hover {
  fill: blue;
  text-decoration: underline;
}

/* Links */
.link {
  fill: none;
  stroke: #ccc;
  stroke-width: 1.5px;
}
.link-fade-enter-active,
.link-fade-leave-active {
  transition: opacity 0.4s;
}
.link-fade-enter-from,
.link-fade-leave-to {
  opacity: 0;
}

/* Side Panel Styling */
.side-panel {
  position: absolute;
  top: 0;
  right: 0;
  width: 300px;
  height: 100%;
  background: white;
  box-shadow: -2px 0 10px rgba(0, 0, 0, 0.1);
  padding: 20px;
  z-index: 10;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
.panel-content input {
  width: 100%;
  padding: 8px;
  margin-bottom: 15px;
  border: 1px solid #ddd;
  border-radius: 4px;
}
.panel-content label {
  display: block;
  margin-bottom: 5px;
  font-size: 0.9em;
  color: #666;
}
.save-btn {
  background: steelblue;
  color: white;
  border: none;
  padding: 10px;
  width: 100%;
  border-radius: 4px;
  cursor: pointer;
}
.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
}

/* Slide Animation for Panel */
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.3s ease;
}
.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(100%);
  opacity: 0;
}
</style>
