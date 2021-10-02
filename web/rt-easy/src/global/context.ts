import React from "react";
import { editor, Range } from "monaco-editor";

export const GlobalContext = React.createContext<GlobalModel>({
  tag: "Edit",
  editor: null,
  setEditor: () => {},
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

export const baseInheritValues = ["Inherit", "BIN", "DEC", "HEX"] as const;
export type BaseInherit = typeof baseInheritValues[number];
export const isBaseInherit = (x: any): x is BaseInherit =>
  baseInheritValues.includes(x);

export const clockRateValues = [1, 2, 4, 8, 100, "Max"] as const;
export type ClockRate = typeof clockRateValues[number];
export const isClockRate = (x: any): x is ClockRate =>
  clockRateValues.includes(x);

export interface Signals {
  conditionSignals: String[];
  controlSignals: String[];
}

export interface GlobalModelCommon {
  editor: editor.IStandaloneCodeEditor | null;
  setEditor: (editor: editor.IStandaloneCodeEditor | null) => void;
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

  signals: () => Signals;
  statementRange: (statement: number) => Range | null;
  addBreakpoint: (statement: number) => void;
  removeBreakpoint: (statement: number) => void;
  breakpoints: () => number[];

  runStop: () => void;
  isRunning: () => boolean;

  cycleCount: () => number;
  registers: (kind: "Intern" | "Output") => string[];
  registerValue: (name: string, base: Base) => string;
  registerValueNext: (name: string, base: Base) => string | null;
  writeRegister: (name: string, value: string, base: Base) => void;
  buses: (kind: "Intern" | "Input") => string[];
  busValue: (name: string, base: Base) => string;
  writeBus: (name: string, value: string, base: Base) => void;
  registerArrays: () => string[];
  registerArrayPageCount: (name: string) => number;
  registerArrayPage: (
    name: string,
    pageNr: number,
    base: Base
  ) => { idx: number; value: string }[];
  writeRegisterArray: (
    name: string,
    idx: number,
    value: string,
    base: Base
  ) => void;
  memories: () => string[];
  memoryPageCount: (name: string) => string;
  memoryPagePrev: (name: string, pageNr: string) => string | null;
  memoryPageNext: (name: string, pageNr: string) => string | null;
  memoryPage: (
    name: string,
    pageNr: string,
    base: Base
  ) => { address: string; value: string }[];
  writeMemory: (
    name: string,
    address: string,
    value: string,
    base: Base
  ) => void;
  memorySave: (name: string) => string;
  memoryLoadFromSave: (name: string, save: string) => void;

  inheritBasesStorage: React.MutableRefObject<Map<string, BaseInherit>>;
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
