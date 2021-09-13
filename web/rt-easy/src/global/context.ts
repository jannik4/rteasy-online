import React from "react";
import { Range } from "monaco-editor";

export const GlobalContext = React.createContext<GlobalModel>({
  tag: "Edit",
  sourceCode: "",
  base: "DEC",
  setBase: () => {},
  log: "",
  setSourceCode: () => {},
  build: () => {},
});

export type GlobalModel = GlobalModelEdit | GlobalModelRun;

export type Base = "BIN" | "DEC" | "HEX";
export interface GlobalModelCommon {
  sourceCode: string;
  base: Base;
  setBase: (base: Base) => void;
}

export interface GlobalModelEdit extends GlobalModelCommon {
  tag: "Edit";
  log: string;
  setSourceCode: (sourceCode: string) => void;
  build: () => void;
}

export interface GlobalModelRun extends GlobalModelCommon {
  tag: "Run";
  goToEditMode: () => void;
  reset: () => void;
  isFinished: () => boolean;
  microStep: () => void;
  step: () => void;
  simState: SimState | null;

  runStop: () => void;
  isRunning: () => boolean;

  cycleCount: () => number;
  registers: () => string[];
  registerValue: (name: string, base: string) => string;
  registerValueNext: (name: string, base: string) => string | null;
  writeIntoRegister: (name: string, value: string, base: string) => void;
  buses: () => string[];
  busValue: (name: string, base: string) => string;
  writeIntoBus: (name: string, value: string, base: string) => void;
  memories: () => string[];
  memoryPageCount: (name: string) => string;
  memoryPagePrev: (name: string, pageNr: string) => string | null;
  memoryPageNext: (name: string, pageNr: string) => string | null;
  memoryPage: (
    name: string,
    pageNr: string,
    base: string
  ) => { address: string; value: string }[];
  writeIntoMemory: (
    name: string,
    address: string,
    value: string,
    base: string
  ) => void;
}

export interface SimState {
  span: Range;
  currCondition: SimStateCondition | null;
  conditions: SimStateCondition[];
}

export interface SimStateCondition {
  value: boolean;
  span: Range;
}
