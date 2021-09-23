import React, { useState, useEffect, useContext } from "react";
import { Monaco } from "@monaco-editor/react";

import { Toolbar } from "./";
import { EditPage, RunPage } from "../layout";

import { RtEasyContext } from "../wasm/context";
import { GlobalContext, GlobalModel } from "../global/context";
import { State, initialState } from "../global/state";
import { model as modelEdit } from "../global/impl/edit";
import { model as modelRun } from "../global/impl/run";
import { Storage } from "../storage";

interface Props {
  monaco: Monaco;
}

const Scaffold: React.FC<Props> = ({ monaco }) => {
  const rtEasy = useContext(RtEasyContext);
  const [state, setState] = useState<State>(() => initialState(monaco));

  // Update storage and call setState if model content changed
  useEffect(() => {
    state.editorModel.onDidChangeContent(() => {
      Storage.setSourceCode(state.editorModel.getValue());
      setState({ ...state });
    });
    // Run effect only once! (We only need one listener)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  let globalModel: GlobalModel;
  let page: React.ReactNode;
  switch (state.tag) {
    case "Edit":
      globalModel = modelEdit(rtEasy, state, setState);
      page = <EditPage />;
      break;
    case "Run":
      globalModel = modelRun(rtEasy, state, setState);
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
