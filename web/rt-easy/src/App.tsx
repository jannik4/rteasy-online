import React, { useState, useEffect } from "react";
import { useMonaco } from "@monaco-editor/react";

import { RtEasyContext } from "./wasm/context";
import { RtEasy, load } from "./wasm";
import { Loading, Scaffold } from "./components";
import { setUpRtEasyLang } from "./util/monacoRtEasy";

interface Props {}

const App: React.FC<Props> = () => {
  // Load rtEasy wasm
  const [rtEasy, setRtEasy] = useState<RtEasy | null>(null);
  useEffect(() => {
    load().then((rtEasy) => setRtEasy(rtEasy));
  }, []);

  // Load monaco editor
  const monaco = useMonaco();
  useEffect(() => {
    if (monaco) setUpRtEasyLang(monaco);
  }, [monaco]);

  if (rtEasy === null || monaco === null) {
    return <Loading />;
  }

  return (
    <RtEasyContext.Provider value={rtEasy}>
      <Scaffold monaco={monaco} />
    </RtEasyContext.Provider>
  );
};

export default App;
