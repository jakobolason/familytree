import * as d3 from "d3";
import { familyTreeData } from "./treeData";

export const useFamilyTree = () => {
  // State
  const rawData = ref(structuredClone(familyTreeData));
  const nodes = ref([]);
  const links = ref([]);

  // Config
  const config = {
    width: 1000,
    height: 600,
    margin: { top: 50, right: 90, bottom: 50, left: 150 },
    nodeRadius: 10,
    duration: 400,
  };

  // D3 setup
  const treeLayout = d3
    .tree()
    .size([
      config.height - config.margin.top - config.margin.bottom,
      config.width - config.margin.left - config.margin.right - 100,
    ]);

  const diagonal = d3
    .linkHorizontal()
    .x((d) => d.y)
    .y((d) => d.x);

  const previousPositions = new Map();
  let nodeIdCounter = 0;

  // Calculate tree layout
  const calculateTreeLayout = () => {
    const root = d3.hierarchy(rawData.value[0]);

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
  };

  // Transition hooks
  const onNodeBeforeEnter = (el: Element) => {
    const id = el.getAttribute("key");
    const node = nodes.value.find((n) => n.data.id == id);
    let x = config.margin.top;
    let y = config.margin.left;

    if (node && node.parent) {
      const prev = previousPositions.get(node.parent.data.id);
      if (prev) {
        x = prev.x;
        y = prev.y;
      }
    }
    d3.select(el).attr("transform", `translate(${y},${x})`);
  };

  const onNodeEnter = (el: Element, done: () => void) => {
    const dataset = (el as HTMLElement).dataset;
    d3.select(el)
      .transition()
      .duration(config.duration)
      .attr("transform", `translate(${dataset.y},${dataset.x})`)
      .on("end", done);
  };

  const onNodeLeave = (el: Element, done: () => void) => {
    d3.select(el)
      .transition()
      .duration(config.duration)
      .style("opacity", 0)
      .on("end", done);
  };

  return {
    rawData,
    nodes,
    links,
    config,
    calculateTreeLayout,
    onNodeBeforeEnter,
    onNodeEnter,
    onNodeLeave,
    previousPositions,
  };
};
