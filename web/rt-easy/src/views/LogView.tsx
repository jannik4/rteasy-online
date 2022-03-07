import React, { useContext, useMemo } from "react";

import { RtEasyContext } from "../wasm/context";
import { GlobalContext } from "../global/context";
import { useDebounce } from "../hooks/useDebounce";

interface Props {}

const LogView: React.FC<Props> = () => {
  const rtEasy = useContext(RtEasyContext);
  const globalModel = useContext(GlobalContext);
  const debouncedSourceCode = useDebounce(
    globalModel.editorModel.getValue(),
    100
  );
  const log = useMemo(() => {
    const res = rtEasy.check(debouncedSourceCode);
    switch (res.tag) {
      case "Ok":
        return res.value;
      case "Error":
        return res.error_html;
    }
  }, [rtEasy, debouncedSourceCode]);

  if (globalModel.tag === "Run") {
    return <div>Err</div>;
  }

  return (
    <div
      style={{
        padding: 8,
      }}
    >
      <pre
        dangerouslySetInnerHTML={{
          __html: log,
        }}
        style={{ margin: 0 }}
      ></pre>
    </div>
  );
};

export default LogView;
