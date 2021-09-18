import React, { useContext, useCallback, useEffect } from "react";
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

import { useFilePicker } from "../hooks/useFilePicker";
import { GlobalContext, GlobalModel } from "../global/context";

interface Props {}

const Toolbar: React.FC<Props> = () => {
  const globalModel = useContext(GlobalContext);
  const openFilePicker = useFilePicker({
    accept: [".rt", ".txt"],
    onChange: (_name, content) => {
      switch (globalModel.tag) {
        case "Edit":
          globalModel.setSourceCode(content);
          break;
        case "Run":
          globalModel.goToEditMode(content);
          break;
      }
    },
  });
  const handleUserKeyPressCallback = useCallback(
    (event: KeyboardEvent) =>
      handleUserKeyPress(event, globalModel, openFilePicker),
    [globalModel, openFilePicker]
  );
  useEffect(() => {
    window.addEventListener("keydown", handleUserKeyPressCallback);
    return () => {
      window.removeEventListener("keydown", handleUserKeyPressCallback);
    };
  }, [handleUserKeyPressCallback]);

  const fileMenu = (
    <Menu>
      <MenuItem
        icon="document-open"
        text="Open File..."
        label="Ctrl+O"
        onClick={openFilePicker}
      />
      <MenuItem icon="download" text="Save File..." label="Ctrl+S" />
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

  const runMenu = (
    <Menu>
      <MenuItem
        icon={globalModel.tag === "Edit" ? "build" : "code"}
        text={globalModel.tag === "Edit" ? "Build" : "Code"}
        label="F5"
        intent="primary"
        onClick={() => globalModel.toggleMode()}
      />
      <MenuItem
        icon={
          globalModel.tag === "Run" && globalModel.isRunning() ? "stop" : "play"
        }
        text={
          globalModel.tag === "Run" && globalModel.isRunning() ? "Stop" : "Run"
        }
        label="F6"
        intent={
          globalModel.tag === "Run" && globalModel.isRunning()
            ? "danger"
            : "success"
        }
        disabled={globalModel.tag === "Edit"}
        onClick={() => {
          if (globalModel.tag === "Run") globalModel.runStop();
        }}
      />
      <MenuItem
        icon="reset"
        text="Reset"
        label="F7"
        intent="success"
        disabled={globalModel.tag === "Edit"}
        onClick={() => {
          if (globalModel.tag === "Run") globalModel.reset();
        }}
      />
      <MenuItem
        icon="step-forward"
        text="Step"
        label="F8"
        intent="none"
        disabled={globalModel.tag === "Edit"}
        onClick={() => {
          if (globalModel.tag === "Run") globalModel.step();
        }}
      />
      <MenuItem
        icon="caret-right"
        text="Micro Step"
        label="F9"
        intent="none"
        disabled={globalModel.tag === "Edit"}
        onClick={() => {
          if (globalModel.tag === "Run") globalModel.microStep();
        }}
      />
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
        <Popover content={runMenu} position={Position.BOTTOM_RIGHT} minimal>
          <Button className={Classes.MINIMAL} text="Run" />
        </Popover>
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
          onClick={() => globalModel.toggleMode()}
          className="noFocus"
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
          className="noFocus"
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
          className="noFocus"
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
          className="noFocus"
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
          className="noFocus"
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

function handleUserKeyPress(
  event: KeyboardEvent,
  globalModel: GlobalModel,
  openFilePicker: () => void
) {
  switch (event.key) {
    case "o":
      if (ctrlKeyPressed(event)) {
        event.preventDefault();
        openFilePicker();
      }
      break;
    case "F5":
      event.preventDefault();
      globalModel.toggleMode();
      break;
    case "F6":
      event.preventDefault();
      if (globalModel.tag === "Run") globalModel.runStop();
      break;
    case "F7":
      event.preventDefault();
      if (globalModel.tag === "Run") globalModel.reset();
      break;
    case "F8":
      event.preventDefault();
      if (globalModel.tag === "Run") globalModel.step();
      break;
    case "F9":
      event.preventDefault();
      if (globalModel.tag === "Run") globalModel.microStep();
      break;
  }
}

function ctrlKeyPressed(event: KeyboardEvent): boolean {
  return isMac() ? event.metaKey : event.ctrlKey;
}

function isMac(): boolean {
  return navigator.userAgent.includes("Mac");
}
