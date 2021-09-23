import React from "react";
import { Spinner } from "@blueprintjs/core";

interface Props {}

const Loading: React.FC<Props> = () => {
  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Spinner />
    </div>
  );
};

export default Loading;
