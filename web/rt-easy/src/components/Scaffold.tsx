import React, { useState, useRef, useContext } from "react";
import { Monaco } from "@monaco-editor/react";

import { Toolbar } from "./";
import { EditPage, RunPage } from "../layout";

import { useLazyRef } from "../hooks/useLazyRef";
import { RtEasyContext } from "../wasm/context";
import { GlobalContext, GlobalModel, BaseInherit } from "../global/context";
import { State, initialState } from "../global/state";
import { model as modelEdit } from "../global/impl/edit";
import { model as modelRun } from "../global/impl/run";
import { Storage } from "../storage";

interface Props {
  monaco: Monaco;
}

const Scaffold: React.FC<Props> = ({ monaco }) => {
  const rtEasy = useContext(RtEasyContext);
  const [state, setState] = useState<State>(() => initialState());
  const editorModelRef = useLazyRef(() => {
    // Load source code and create model
    const sourceCode = Storage.getSourceCode() || "";
    const editorModel = monaco.editor.createModel(sourceCode, "rt-easy");

    // Update storage and call setState if model content changed
    editorModel.onDidChangeContent(() => {
      Storage.setSourceCode(editorModel.getValue());
      setState((prev) => {
        return { ...prev };
      });
    });

    return editorModel;
  });
  const inheritBasesStorage = useRef<Map<string, BaseInherit>>(new Map());

  let globalModel: GlobalModel;
  let page: React.ReactNode;
  switch (state.tag) {
    case "Edit":
      globalModel = modelEdit(rtEasy, state, setState, editorModelRef.current);
      page = <EditPage />;
      break;
    case "Run":
      globalModel = modelRun(
        rtEasy,
        state,
        setState,
        editorModelRef.current,
        inheritBasesStorage
      );
      page = <RunPage />;
      break;
  }

  return (
    <GlobalContext.Provider value={globalModel}>
      <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
        <div style={{ flex: "0 0 64px", overflow: "hidden" }}>
          <Toolbar />
        </div>
        <div style={{ flex: "1", position: "relative" }}>{page}</div>
      </div>
    </GlobalContext.Provider>
  );
};

export default Scaffold;
