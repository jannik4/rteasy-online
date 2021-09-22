import { Base, SimState } from "./context";
import { Simulator } from "../wasm";
import { Storage } from "../storage";

export type State = StateEdit | StateRun;

export interface StateCommon {
  sourceCode: string;
  base: Base;
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

export function initialState(): State {
  return {
    tag: "Edit",
    sourceCode: Storage.getSourceCode() || "",
    base: "DEC",
  };
}
