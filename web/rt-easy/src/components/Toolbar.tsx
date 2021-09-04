import React, { useContext } from "react";
import {
  Button,
  Classes,
  Popover,
  Position,
  MenuDivider,
  MenuItem,
  Menu,
  Text,
} from "@blueprintjs/core";

import { GlobalContext } from "../global/context";

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

  const fileMenu = (
    <Menu>
      <MenuItem icon="document-open" text="Open File..." />
      <MenuItem icon="download" text="Save File..." />
    </Menu>
  );

  const editMenu = (
    <Menu>
      <MenuItem icon="undo" text="Undo" label="Ctrl+Z" />
      <MenuItem icon="redo" text="Redo" label="Ctrl+Y" />

      <MenuDivider />
      <MenuItem icon="cut" text="Cut" label="Ctrl+X" />
      <MenuItem icon="duplicate" text="Copy" label="Ctrl+C" />
      <MenuItem icon="clipboard" text="Paste" label="Ctrl+V" />

      <MenuDivider />
      <MenuItem icon="search" text="Find" label="Ctrl+F" />
      <MenuItem icon="multi-select" text="Replace" label="Ctrl+H" />
    </Menu>
  );

  return (
    <div
      style={{
        boxSizing: "border-box",
        height: "100%",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <div
        style={{
          display: "flex",
          borderBottom: "1px solid #ccc",
        }}
      >
        <Popover content={fileMenu} position={Position.BOTTOM_RIGHT} minimal>
          <Button className={Classes.MINIMAL} text="File" />
        </Popover>
        <Popover content={editMenu} position={Position.BOTTOM_RIGHT} minimal>
          <Button className={Classes.MINIMAL} text="Edit" />
        </Popover>
        <Button className={Classes.MINIMAL} text="Run" />
        <Button className={Classes.MINIMAL} text="Help" />
        <div style={{ margin: "auto" }}>
          <Text>RTeasy-Online</Text>
        </div>
      </div>

      <div
        style={{
          flex: "1",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Button
          icon={globalModel.tag === "Edit" ? "build" : "code"}
          onClick={toggleMode}
          style={{ marginLeft: "8px", marginRight: "16px" }}
          intent="primary"
          minimal
          small
        />
        <Button
          icon={
            globalModel.tag === "Run" && globalModel.isRunning()
              ? "stop"
              : "play"
          }
          onClick={() => {
            if (globalModel.tag === "Run") globalModel.runStop();
          }}
          style={{ marginRight: "16px" }}
          intent={
            globalModel.tag === "Run" && globalModel.isRunning()
              ? "danger"
              : "success"
          }
          minimal
          small
          disabled={globalModel.tag === "Edit"}
        />
        <Button
          icon="reset"
          onClick={() => {
            if (globalModel.tag === "Run") globalModel.reset();
          }}
          style={{ marginRight: "16px" }}
          intent="success"
          minimal
          small
          disabled={globalModel.tag === "Edit"}
        />
        <Button
          icon="step-forward"
          onClick={() => {
            if (globalModel.tag === "Run") globalModel.step();
          }}
          style={{ marginRight: "16px" }}
          intent="none"
          minimal
          small
          disabled={globalModel.tag === "Edit"}
        />
        <Button
          icon="caret-right"
          onClick={() => {
            if (globalModel.tag === "Run") globalModel.microStep();
          }}
          intent="none"
          minimal
          small
          disabled={globalModel.tag === "Edit"}
        />
      </div>
    </div>
  );
};

export default Toolbar;
