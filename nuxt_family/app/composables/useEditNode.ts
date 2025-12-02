export const useEditNode = () => {
  const isOpen = useState("editNodeOpen", () => false);
  const nodeData = useState("editNodeData", () => null);

  const openEditNode = (data?: any) => {
    // nodeData.value = data;
    isOpen.value = true;
  };

  const closeEditNode = () => {
    // isOpen.value = false;
    nodeData.value = null;
  };

  return {
    isOpen,
    nodeData,
    openEditNode,
    closeEditNode,
  };
};
