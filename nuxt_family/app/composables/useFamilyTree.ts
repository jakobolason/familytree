import * as d3 from 'd3';

export const useFamilyTree = () => {
  const nodes = ref([]);
  const links = ref([]);

  const { data: treeData, status, error, refresh } = useFetch('/api/tree', {
    lazy: true,
    default: () => null // Start as null so we can check for existence
  });

  // The Layout Configuration
  // [50, 100] means: 50px vertical space between siblings, 100px horizontal space between generations.
  const treeLayout = d3.tree().nodeSize([50, 100]);

  // Whenever data arrives from the backend, recalculate the D3 layout automatically.
  watch(treeData, (newData) => {
    if (!newData) return;

    // should not be array
    const rawData = Array.isArray(newData) ? newData[0] : newData;

    if (!rawData) {
      console.warn("Tree data is empty");
      return;
    }

    const root = d3.hierarchy(rawData);
    treeLayout(root);

    nodes.value = root.descendants();
    links.value = root.links().map(link => ({
      pathD: d3.linkHorizontal()
        .x(d => d.y)
        .y(d => d.x)(link),
      targetId: link.target.data.id || link.target.data.pid
    }));

  }, { immediate: true });

  return {
    treeData,
    status,
    error,
    nodes,
    links,
    refresh
  };
};
