import { ClockRate, isClockRate } from "./global/context";
import { Base, isBase } from "./wasm";

const VERSION = "0.0.1";

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

  getLayoutModel: (id: string) =>
    localStorage.getItem("layout-model-" + VERSION + "-" + id),
  setLayoutModel: (id: string, layoutModel: string) =>
    localStorage.setItem("layout-model-" + VERSION + "-" + id, layoutModel),
  removeAllLayoutModels: () => {
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)!;
      if (key.startsWith("layout-model-")) {
        localStorage.removeItem(key);
      }
    }
  },
};

function isNumeric(val: string) {
  return /^-?\d+$/.test(val);
}
