import React from "react";
import { editor } from "monaco-editor";
import { Simulator, Base } from "../wasm";

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

export const baseInheritValues = ["Inherit", "BIN", "DEC", "HEX"] as const;
export type BaseInherit = typeof baseInheritValues[number];
export const isBaseInherit = (x: any): x is BaseInherit =>
  baseInheritValues.includes(x);

export const clockRateValues = [1, 2, 4, 8, 100, "Max"] as const;
export type ClockRate = typeof clockRateValues[number];
export const isClockRate = (x: any): x is ClockRate =>
  clockRateValues.includes(x);

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

  simulator: Simulator;
  toggleRun: () => void;
  inheritBasesStorage: React.MutableRefObject<Map<string, BaseInherit>>;
}
