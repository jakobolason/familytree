export const useProfile = () => {
  const isOpen = useState("profile-open", () => false);
  // const nodeData = useState("editNodeData", () => null);

  const openProfile = () => {
    // nodeData.value = data.data;
    console.log('opening')
    isOpen.value = true;
  };

  const closeProfile = () => {
    console.log('closing')
    isOpen.value = false;
    // nodeData.value = null;
  };

  return {
    isOpen,
    // nodeData,
    openProfile,
    closeProfile,
  };
};
