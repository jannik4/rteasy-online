export const Storage = {
  getSourceCode: () => localStorage.getItem("source-code"),
  setSourceCode: (sourceCode: string) =>
    localStorage.setItem("source-code", sourceCode),

  getLayoutModel: (id: string) => localStorage.getItem("layout-model-" + id),
  setLayoutModel: (id: string, layoutModel: string) =>
    localStorage.setItem("layout-model-" + id, layoutModel),
};
