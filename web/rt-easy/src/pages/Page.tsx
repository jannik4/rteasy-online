import React, { useRef } from "react";
import FlexLayout, { TabNode, Model } from "flexlayout-react";

import { EditorView, LogView, StateView } from "../views";

interface Props {
  id: string;
  defaultModel: any;
}

const Page: React.FC<Props> = ({ id, defaultModel }) => {
  const model = useRef(loadModel(id, defaultModel));

  const factory = (node: TabNode) => {
    const component = node.getComponent();
    switch (component) {
      case "editor":
        return <EditorView />;
      case "log":
        return <LogView />;
      case "state":
        return <StateView />;
      default:
        return <div>missing component {component}</div>;
    }
  };

  return (
    <FlexLayout.Layout
      model={model.current}
      factory={factory}
      onModelChange={(model) => saveModel(id, model)}
    />
  );
};

export default Page;

function loadModel(id: string, defaultModel: any): Model {
  const saved = localStorage.getItem("layout-model-" + id);
  if (saved !== null) {
    try {
      const json = JSON.parse(saved);
      return Model.fromJson(json);
    } catch (e) {}
  }

  return Model.fromJson(defaultModel);
}

function saveModel(id: string, model: Model) {
  localStorage.setItem("layout-model-" + id, model.toString());
}
