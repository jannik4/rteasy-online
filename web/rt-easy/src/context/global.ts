import React from "react";

import { Span } from "../wasm";

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
  isFinished: () => boolean;
  microStep: () => void;
  step: () => void;
  currSpan: () => Span | null;

  runStop: () => void;
  isRunning: () => boolean;

  cycleCount: () => number;
  registers: () => string[];
  registerValue: (name: string, base: string) => string;
  buses: () => string[];
  busValue: (name: string, base: string) => string;
  writeIntoBus: (name: string, value: string, base: string) => void;
}

export const GlobalContext = React.createContext<GlobalModel>({
  tag: "Edit",
  sourceCode: "",
  base: "DEC",
  setBase: () => {},
  log: "",
  setSourceCode: () => {},
  build: () => {},
});
