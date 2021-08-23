import React, { useContext } from "react";
import MonacoEditor from "@monaco-editor/react";

import { GlobalContext } from "../context";

interface Props {}

const StateView: React.FC<Props> = () => {
  const globalModel = useContext(GlobalContext);

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  const state = globalModel.state();

  return (
    <div style={{ height: "100%" /*, overflow: "hidden"*/ }}>
      <MonacoEditor
        value={state}
        options={{ readOnly: true, fixedOverflowWidgets: true }}
      />
    </div>
  );
};

export default StateView;
