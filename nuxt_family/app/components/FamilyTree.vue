<template>
  <div> states: {{ status }}: , error: {{ error }} </div>
  <div v-if="status === 'pending'">Loadfing tree .. </div>
  <div v-else-if="error">ERROR: see above </div>
  <div v-else class="w-full font-sans">
    <svg :width="config.width" :height="config.height">
      <g :transform="`translate(${config.margin.left},${config.margin.top})`">
        <!-- Links -->
        <TransitionGroup name="link-fade" tag="g">
          <path
            v-for="link in links"
            :key="link.targetId"
            :d="link.pathD"
            class="fill-none stroke-gray-300 stroke-[1.5px]"
          />
        </TransitionGroup>

        <!-- Nodes -->
        <TransitionGroup
          name="node-transition"
          tag="g"
          @before-enter="onNodeBeforeEnter"
          @enter="onNodeEnter"
          @leave="onNodeLeave"
        >
          <g
            v-for="node in nodes"
            :key="node.data.id"
            class="cursor-pointer"
            :data-x="node.x"
            :data-y="node.y"
            :transform="`translate(${node.y},${node.x})`"
          >
            <circle
              :r="config.nodeRadius"
              class="stroke-blue-500 stroke-[1.5px] transition-all hover:stroke-black hover:stroke-2"
              :class="node._children ? 'fill-blue-200' : 'fill-white'"
              @click.stop="openEditNode(node)"
            />

            <text
              dy="0.35em"
              :x="node.children || node._children ? -13 : 13"
              :text-anchor="node.children || node._children ? 'end' : 'start'"
              class="text-xs font-medium cursor-pointer hover:fill-blue-600 hover:underline"
              @click.stop="openEditNode(node)"
            >
              {{ node.data.name }}
            </text>
          </g>
        </TransitionGroup>
      </g>
    </svg>
  </div>
</template>

<script setup lang="ts">
const { openEditNode } = useEditNode();
const {
  treeData,
  status,
  error,
  nodes,
  links,
  config,
  calculateTreeLayout,
  onNodeBeforeEnter,
  onNodeEnter,
  onNodeLeave,
} = useFamilyTree();


</script>

<style scoped>
/* Keep minimal CSS for transitions that Tailwind can't handle */
.link-fade-enter-active,
.link-fade-leave-active {
  transition: opacity 0.4s;
}
.link-fade-enter-from,
.link-fade-leave-to {
  opacity: 0;
}
</style>
