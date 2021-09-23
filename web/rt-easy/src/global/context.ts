import React from "react";
import { editor, Range } from "monaco-editor";

export const GlobalContext = React.createContext<GlobalModel>({
  tag: "Edit",
  editorModel: null as any,
  toggleMode: () => {},
  base: "DEC",
  setBase: () => {},
  clockRate: "Max",
  setClockRate: () => {},
  build: () => {},
});

export type GlobalModel = GlobalModelEdit | GlobalModelRun;

export const baseValues = ["BIN", "DEC", "HEX"] as const;
export type Base = typeof baseValues[number];
export const isBase = (x: any): x is Base => baseValues.includes(x);

export const clockRateValues = [1, 2, 4, 8, 100, "Max"] as const;
export type ClockRate = typeof clockRateValues[number];
export const isClockRate = (x: any): x is ClockRate =>
  clockRateValues.includes(x);

export interface GlobalModelCommon {
  editorModel: editor.IModel;
  toggleMode: () => void;
  base: Base;
  setBase: (base: Base) => void;
  clockRate: ClockRate;
  setClockRate: (clockRate: ClockRate) => void;
}

export interface GlobalModelEdit extends GlobalModelCommon {
  tag: "Edit";
  build: () => void;
}

export interface GlobalModelRun extends GlobalModelCommon {
  tag: "Run";
  goToEditMode: (sourceCode?: string) => void;
  reset: () => void;
  isFinished: () => boolean;
  microStep: () => void;
  step: () => void;
  simState: SimState | null;

  runStop: () => void;
  isRunning: () => boolean;

  cycleCount: () => number;
  registers: (kind: "Intern" | "Output") => string[];
  registerValue: (name: string, base: string) => string;
  registerValueNext: (name: string, base: string) => string | null;
  writeIntoRegister: (name: string, value: string, base: string) => void;
  buses: (kind: "Intern" | "Input") => string[];
  busValue: (name: string, base: string) => string;
  writeIntoBus: (name: string, value: string, base: string) => void;
  registerArrays: () => string[];
  registerArrayPageCount: (name: string) => number;
  registerArrayPage: (
    name: string,
    pageNr: number,
    base: string
  ) => { idx: number; value: string }[];
  writeIntoRegisterArray: (
    name: string,
    idx: number,
    value: string,
    base: string
  ) => void;
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
  memorySave: (name: string) => string;
  memoryLoadFromSave: (name: string, save: string) => void;
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
