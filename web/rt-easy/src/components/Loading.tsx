import React from "react";

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
      loading...
    </div>
  );
};

export default Loading;
