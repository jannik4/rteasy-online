import { Monaco } from "@monaco-editor/react";
import { editor } from "monaco-editor";

import { Base, ClockRate, SimState } from "./context";
import { Simulator } from "../wasm";
import { Storage } from "../storage";

export type State = StateEdit | StateRun;

export interface StateCommon {
  editorModel: editor.IModel;
  base: Base;
  clockRate: ClockRate;
}

export interface StateEdit extends StateCommon {
  tag: "Edit";
}

export interface StateRun extends StateCommon {
  tag: "Run";
  simState: SimState | null;
  simulator: Simulator;
  timerId: NodeJS.Timeout | null;
}

export function initialState(monaco: Monaco): State {
  const sourceCode = Storage.getSourceCode() || "";

  return {
    tag: "Edit",
    editorModel: monaco.editor.createModel(sourceCode, "rt-easy"),
    base: Storage.getBase() || "DEC",
    clockRate: Storage.getClockRate() || "Max",
  };
}
