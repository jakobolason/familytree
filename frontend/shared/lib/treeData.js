export const familyTreeData = [
  {
    name: "Root: Grandparent A",
    // We use a property to track if children should be visible
    _children: null,
    children: [
      {
        name: "Parent 1",
        _children: null,
        children: [{ name: "Child 1-1" }, { name: "Child 1-2" }],
      },
      {
        name: "Parent 2",
        _children: null,
        children: [
          {
            name: "Child 2-1",
            _children: null,
            children: [
              {
                name: "Grandchild 2-1 1",
              },
              {
                name: "Grandchild 2-1 2",
              },
            ],
          },
          { name: "Child 2-2" },
        ],
      },
    ],
  },
];
