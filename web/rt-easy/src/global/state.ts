import { Base, SimState } from "./context";
import { Simulator } from "../wasm";

export type State = StateEdit | StateRun;

export interface StateCommon {
  sourceCode: string;
  base: Base;
}

export interface StateEdit extends StateCommon {
  tag: "Edit";
  log: string;
}

export interface StateRun extends StateCommon {
  tag: "Run";
  simState: SimState | null;
  simulator: Simulator;
  timerId: NodeJS.Timeout | null;
}

export function initialState(): State {
  return {
    tag: "Edit",
    sourceCode: localStorage.getItem("source-code") || "",
    base: "DEC",
    log: "--- ok ---",
  };
}
