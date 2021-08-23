import React, { useContext } from "react";

import { GlobalContext } from "../context";

interface Props {}

const Toolbar: React.FC<Props> = () => {
  const globalModel = useContext(GlobalContext);

  const toggleMode = () => {
    switch (globalModel.tag) {
      case "Edit":
        globalModel.build();
        break;
      case "Run":
        globalModel.goToEditMode();
        break;
    }
  };

  return (
    <div
      style={{
        boxSizing: "border-box",
        height: "100%",
        display: "flex",
        alignItems: "center",
        padding: "2px 8px",
        borderBottom: "1px solid gray",
      }}
    >
      <span>
        <b>rt-easy</b>
      </span>
      <div style={{ width: "20px" }}></div>
      <button onClick={toggleMode}>Mode</button>
      {globalModel.tag === "Run" ? (
        <>
          <div style={{ width: "20px" }}></div>
          <button onClick={() => globalModel.step()}>Step</button>
          <div style={{ width: "20px" }}></div>
          <button
            onClick={() => {
              for (let i = 0; i < 10; i++) globalModel.step();
            }}
          >
            Step 10
          </button>
          <div style={{ width: "20px" }}></div>
          <button onClick={() => globalModel.runStop()}>
            {globalModel.isRunning() ? "Stop" : "Run"}
          </button>
        </>
      ) : null}
    </div>
  );
};

export default Toolbar;
