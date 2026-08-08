export const useMedlemNode = () => {
  const isOpen = useState("medlemNodeOpen", () => false);
  const nodeData = useState("medlemData", () => null);

  const openMedlemNode = (data?: any) => {
    nodeData.value = data.data;
    console.log('data: ', data.data);
    isOpen.value = true;
  };

  const closeMedlemNode = () => {
    // isOpen.value = false;
    nodeData.value = null;
  };

  return {
    isOpen,
    nodeData,
    openMedlemNode,
    closeMedlemNode,
  };
};
