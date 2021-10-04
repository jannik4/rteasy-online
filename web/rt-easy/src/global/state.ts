import { editor } from "monaco-editor";
import { ClockRate } from "./context";
import { Simulator, Base } from "../wasm";
import { Storage } from "../storage";

export type State = StateEdit | StateRun;

export interface StateCommon {
  editor: editor.IStandaloneCodeEditor | null;
  base: Base;
  clockRate: ClockRate;
}

export interface StateEdit extends StateCommon {
  tag: "Edit";
}

export interface StateRun extends StateCommon {
  tag: "Run";
  simulator: Simulator;
}

export function initialState(): State {
  return {
    tag: "Edit",
    editor: null,
    base: Storage.getBase() || "DEC",
    clockRate: Storage.getClockRate() || 100,
  };
}
