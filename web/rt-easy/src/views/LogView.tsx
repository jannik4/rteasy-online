import React, { useContext } from "react";
import MonacoEditor from "@monaco-editor/react";

// import { RtEasyContext } from "../context";
import { GlobalContext } from "../context";

interface Props {}

const LogView: React.FC<Props> = () => {
  // const rtEasy = useContext(RtEasyContext);
  const globalModel = useContext(GlobalContext);

  if (globalModel.tag === "Run") {
    return <div>Err</div>;
  }

  // let result: string;
  // try {
  //   result = rtEasy.run(globalModel.sourceCode);
  // } catch (e) {
  //   result = e;
  // }

  return (
    <div style={{ height: "100%" /*, overflow: "hidden"*/ }}>
      <MonacoEditor
        value={globalModel.log}
        options={{ readOnly: true, fixedOverflowWidgets: true }}
      />
    </div>
  );
};

export default LogView;
