import React, { useState, useContext, useCallback, useEffect } from "react";
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

import { OptionsDialog } from "./";

import { useFilePicker } from "../hooks/useFilePicker";
import { downloadFile } from "../util/downloadFile";
import { GlobalContext, GlobalModel } from "../global/context";

const FILENAME = "code.rt";

interface Props {}

const Toolbar: React.FC<Props> = () => {
  const globalModel = useContext(GlobalContext);
  const [showOptionsDialog, setShowOptionsDialog] = useState(false);
  const openFilePicker = useFilePicker({
    accept: [".rt", ".txt"],
    onChange: (_name, content) => {
      // Use pushEditOperations instead of setValue to preserve undo/redo stack
      globalModel.editorModel.pushEditOperations(
        [],
        [
          {
            range: globalModel.editorModel.getFullModelRange(),
            text: content,
          },
        ],
        () => null
      );
      if (globalModel.tag === "Run") globalModel.goToEditMode();
    },
  });
  const handleUserKeyPressCallback = useCallback(
    (event: KeyboardEvent) =>
      handleUserKeyPress(
        event,
        globalModel,
        setShowOptionsDialog,
        openFilePicker
      ),
    [globalModel, setShowOptionsDialog, openFilePicker]
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
        label={ctrlKeyShortCut("O")}
        onClick={openFilePicker}
      />
      <MenuItem
        icon="download"
        text="Save File..."
        label={ctrlKeyShortCut("S")}
        onClick={() =>
          downloadFile(FILENAME, globalModel.editorModel.getValue())
        }
      />
      <MenuItem
        icon="cog"
        text="Options..."
        label={ctrlKeyShortCut(",")}
        onClick={() => setShowOptionsDialog(true)}
      />
    </Menu>
  );

  const editMenu = (
    <Menu>
      <MenuItem
        icon="undo"
        text="Undo"
        label={ctrlKeyShortCut("Z")}
        onClick={() => {
          globalModel.editor?.focus();
          globalModel.editor?.trigger("source", "undo", null);
        }}
      />
      <MenuItem
        icon="redo"
        text="Redo"
        label={ctrlKeyShortCut("Y")}
        onClick={() => {
          globalModel.editor?.focus();
          globalModel.editor?.trigger("source", "redo", null);
        }}
      />

      <MenuDivider />
      <MenuItem
        icon="cut"
        text="Cut"
        label={ctrlKeyShortCut("X")}
        onClick={() => {
          globalModel.editor?.focus();
          globalModel.editor?.trigger(
            "source",
            "editor.action.clipboardCutAction",
            null
          );
        }}
      />
      <MenuItem
        icon="duplicate"
        text="Copy"
        label={ctrlKeyShortCut("C")}
        onClick={() => {
          globalModel.editor?.focus();
          globalModel.editor?.trigger(
            "source",
            "editor.action.clipboardCopyWithSyntaxHighlightingAction",
            null
          );
        }}
      />
      <MenuItem
        icon="clipboard"
        text="Paste"
        label={ctrlKeyShortCut("V")}
        onClick={() => {
          globalModel.editor?.focus();
          navigator.clipboard.readText().then((text) => {
            globalModel.editor?.trigger("keyboard", "type", {
              text,
            });
          });
        }}
      />

      <MenuDivider />
      <MenuItem
        icon="search"
        text="Find"
        label={ctrlKeyShortCut("F")}
        onClick={() => {
          globalModel.editor?.focus();
          globalModel.editor?.trigger("source", "actions.find", null);
        }}
      />
      <MenuItem
        icon="multi-select"
        text="Replace"
        label={ctrlKeyShortCut("H")}
        onClick={() => {
          globalModel.editor?.focus();
          globalModel.editor?.trigger(
            "source",
            "editor.action.startFindReplaceAction",
            null
          );
        }}
      />
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
      <OptionsDialog
        isOpen={showOptionsDialog}
        onClose={() => setShowOptionsDialog(false)}
      />
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
  setShowOptionsDialog: (value: boolean) => void,
  openFilePicker: () => void
) {
  switch (event.key) {
    case "o":
      if (ctrlKeyPressed(event)) {
        event.preventDefault();
        if (event.repeat) return;
        openFilePicker();
      }
      break;
    case "s":
      if (ctrlKeyPressed(event)) {
        event.preventDefault();
        if (event.repeat) return;
        downloadFile(FILENAME, globalModel.editorModel.getValue());
      }
      break;
    case ",":
      if (ctrlKeyPressed(event)) {
        event.preventDefault();
        if (event.repeat) return;
        setShowOptionsDialog(true);
      }
      break;
    case "F5":
      event.preventDefault();
      if (event.repeat) return;
      globalModel.toggleMode();
      break;
    case "F6":
      event.preventDefault();
      if (event.repeat) return;
      if (globalModel.tag === "Run") globalModel.runStop();
      break;
    case "F7":
      event.preventDefault();
      if (event.repeat) return;
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

function ctrlKeyShortCut(char: string): string {
  return (isMac() ? "⌘+" : "Ctrl+") + char;
}

function isMac(): boolean {
  return navigator.userAgent.includes("Mac");
}
