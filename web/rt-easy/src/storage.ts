import { Base, isBase, ClockRate, isClockRate } from "./global/context";

export const Storage = {
  getSourceCode: () => localStorage.getItem("source-code"),
  setSourceCode: (sourceCode: string) =>
    localStorage.setItem("source-code", sourceCode),

  getBase: () => {
    const base = localStorage.getItem("value-base");
    if (base === null) return null;
    if (isBase(base)) return base;
    return null;
  },
  setBase: (base: Base) => localStorage.setItem("value-base", base.toString()),

  getClockRate: () => {
    let clockRate: string | number | null = localStorage.getItem("clock-rate");
    if (clockRate === null) return null;
    clockRate = isNumeric(clockRate) ? parseInt(clockRate) : clockRate;
    if (isClockRate(clockRate)) return clockRate;
    return null;
  },
  setClockRate: (clockRate: ClockRate) =>
    localStorage.setItem("clock-rate", clockRate.toString()),

  getLayoutModel: (id: string) => localStorage.getItem("layout-model-" + id),
  setLayoutModel: (id: string, layoutModel: string) =>
    localStorage.setItem("layout-model-" + id, layoutModel),
};

function isNumeric(val: string) {
  return /^-?\d+$/.test(val);
}
