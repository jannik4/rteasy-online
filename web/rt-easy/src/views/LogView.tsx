import React, { useContext } from "react";
import Anser from "anser";

import { GlobalContext } from "../global/context";

interface Props {}

const LogView: React.FC<Props> = () => {
  const globalModel = useContext(GlobalContext);

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
        dangerouslySetInnerHTML={{ __html: Anser.ansiToHtml(globalModel.log) }}
        style={{ margin: 0 }}
      ></pre>
    </div>
  );
};

export default LogView;
