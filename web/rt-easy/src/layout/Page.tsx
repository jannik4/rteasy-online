import React, { useRef } from "react";
import FlexLayout, { TabNode, Model, IJsonModel, Node } from "flexlayout-react";

import { LayoutModelContext, LayoutModel } from "./context";
import {
  EditorView,
  LogView,
  StateView,
  MemoryStateView,
  RegisterArrayStateView,
  SignalsView,
} from "../views";
import * as consts from "./consts";
import { Storage } from "../storage";

interface Props {
  id: string;
  defaultModel: IJsonModel;
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
      case "signals":
        return <SignalsView />;
      default:
        const memory_prefix = "memory-";
        if (component?.startsWith(memory_prefix)) {
          return (
            <MemoryStateView memory={component.slice(memory_prefix.length)} />
          );
        }

        const register_array_prefix = "register-array-";
        if (component?.startsWith(register_array_prefix)) {
          return (
            <RegisterArrayStateView
              registerArray={component.slice(register_array_prefix.length)}
            />
          );
        }

        console.log("Missing component: " + component);
        return <div>missing component {component}</div>;
    }
  };

  return (
    <LayoutModelContext.Provider value={new LayoutModel(model.current)}>
      <FlexLayout.Layout
        model={model.current}
        factory={factory}
        onModelChange={(model) => saveModel(id, model)}
      />
    </LayoutModelContext.Provider>
  );
};

export default Page;

function loadModel(id: string, defaultModel: IJsonModel): Model {
  const saved = Storage.getLayoutModel(id);
  if (saved !== null) {
    try {
      const json = JSON.parse(saved);
      const model = Model.fromJson(json);

      // Find all temp nodes
      let nodes: Node[] = [];
      model.visitNodes((n, _l) => {
        if (n.getId().includes(consts.MARKER_TEMP)) nodes.push(n);
      });

      // Remove all temp nodes
      for (const node of nodes) {
        model.doAction(FlexLayout.Actions.deleteTab(node.getId()));
      }

      return model;
    } catch (e) {}
  }

  return Model.fromJson(defaultModel);
}

// TODO: debounce?
function saveModel(id: string, model: Model) {
  Storage.setLayoutModel(id, model.toString());
}
