import React, { useState, useEffect } from "react";

import { RtEasyContext } from "./wasm/context";
import { RtEasy } from "./wasm";
import { Loading, Scaffold } from "./components";

interface Props {}

const App: React.FC<Props> = () => {
  const [rtEasy, setRtEasy] = useState<RtEasy | null>(null);

  useEffect(() => {
    import("./wasm/pkg").then((rtEasy) => {
      rtEasy.setPanicHook();
      setRtEasy(rtEasy);
    });
  }, []);

  if (rtEasy === null) {
    return <Loading />;
  }

  return (
    <RtEasyContext.Provider value={rtEasy}>
      <Scaffold />
    </RtEasyContext.Provider>
  );
};

export default App;
