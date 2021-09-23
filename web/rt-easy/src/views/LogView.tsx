import React, { useContext, useMemo } from "react";
import Anser from "anser";

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
    try {
      rtEasy.check(debouncedSourceCode);
      return "--- ok ---";
    } catch (e) {
      return e as string;
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
          __html: Anser.ansiToHtml(Anser.escapeForHtml(log)),
        }}
        style={{ margin: 0 }}
      ></pre>
    </div>
  );
};

export default LogView;
