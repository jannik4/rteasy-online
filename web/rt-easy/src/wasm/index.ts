import * as wasm from "./pkg";
import { Range } from "monaco-editor";

type RtEasyWasm = typeof import("./pkg");

export async function load(): Promise<RtEasy> {
  const rtEasyWasm = await import("./pkg");
  rtEasyWasm.setPanicHook();
  return new RtEasy(rtEasyWasm);
}

export class RtEasy {
  private rtEasyWasm: RtEasyWasm;

  constructor(rtEasyWasm: RtEasyWasm) {
    this.rtEasyWasm = rtEasyWasm;
  }

  check(code: string) {
    this.rtEasyWasm.check(code);
  }

  build(code: string, onChange?: () => void): Simulator {
    return new Simulator(this.rtEasyWasm.build(code), code, onChange);
  }
}

export class Simulator {
  private simulatorWasm: wasm.Simulator;
  private sourceCode: string;
  private signals: Signals;
  private simState: SimState | null;
  private runTimer: NodeJS.Timeout | null;

  private onChange: () => void;

  constructor(
    simulatorWasm: wasm.Simulator,
    sourceCode: string,
    onChange?: () => void
  ) {
    this.simulatorWasm = simulatorWasm;
    this.sourceCode = sourceCode;
    this.signals = getSignals(simulatorWasm);
    this.simState = null;
    this.runTimer = null;

    this.onChange = onChange ?? (() => {});
  }

  free = (): void => {
    if (this.runTimer !== null) clearInterval(this.runTimer);
    this.simulatorWasm.free();
  };

  reset = (): void => {
    this.stop();
    this.simulatorWasm.reset();
    this.simState = null;
    this.onChange();
  };
  cycleCount = (): number => this.simulatorWasm.cycle_count();
  isFinished = (): boolean => this.simulatorWasm.is_finished();
  getSimState = (): SimState | null => this.simState;
  getSignals = (): Signals => this.signals;

  statementRange = (statement: number): Range | null => {
    const span = this.simulatorWasm.statement_span(statement);
    if (span === undefined) return null;
    const range = calcRange(this.sourceCode, span);
    span.free();
    return range;
  };
  addBreakpoint = (statement: number): void => {
    this.simulatorWasm.add_breakpoint(statement);
    this.onChange();
  };
  removeBreakpoint = (statement: number): void => {
    this.simulatorWasm.remove_breakpoint(statement);
    this.onChange();
  };
  breakpoints = (): number[] => Array.from(this.simulatorWasm.breakpoints());

  microStep = (): void => {
    this.simState = simulatorStep({
      simulator: this.simulatorWasm,
      currSimState: this.simState,
      micro: true,
      stopOnBreakpoint: false,
      sourceCode: this.sourceCode,
    });
    this.onChange();
  };
  step = (): void => {
    this.simState = simulatorStep({
      simulator: this.simulatorWasm,
      currSimState: this.simState,
      micro: false,
      stopOnBreakpoint: false,
      sourceCode: this.sourceCode,
    });
    this.onChange();
  };

  run = (intervalMsOrMax: number | "Max"): void => {
    if (this.isRunning() || this.isFinished()) return;
    this.runTimer = setInterval(
      () => {
        // Run
        if (intervalMsOrMax === "Max") {
          // Run for MS ms
          const MS = 5;
          let start = performance.now();
          while (true) {
            this.simState = simulatorStep({
              simulator: this.simulatorWasm,
              currSimState: this.simState,
              micro: false,
              stopOnBreakpoint: true,
              sourceCode: this.sourceCode,
            });

            if (this.simState?.isAtBreakpoint) break;
            if (performance.now() - start > MS) break;
          }
        } else {
          // Run one step
          this.simState = simulatorStep({
            simulator: this.simulatorWasm,
            currSimState: this.simState,
            micro: false,
            stopOnBreakpoint: true,
            sourceCode: this.sourceCode,
          });
        }

        // Check finished or breakpoint
        if (this.isFinished() || this.simState?.isAtBreakpoint) {
          this.stop();
          return;
        }

        // Call on changed
        this.onChange();
      },
      intervalMsOrMax === "Max" ? 10 : intervalMsOrMax
    );
    this.onChange();
  };
  stop = (): void => {
    if (this.runTimer === null) return;
    clearInterval(this.runTimer);
    this.runTimer = null;
    this.onChange();
  };
  toggleRun = (intervalMsOrMax: number | "Max"): void => {
    this.isRunning() ? this.stop() : this.run(intervalMsOrMax);
  };
  isRunning = (): boolean => this.runTimer !== null;

  registers = (kind: "Intern" | "Output"): string[] =>
    this.simulatorWasm.registers(kind);
  registerValue = (name: string, base: Base): string =>
    this.simulatorWasm.register_value(name, base);
  registerValueNext = (name: string, base: Base): string | null =>
    this.simulatorWasm.register_value_next(name, base) ?? null;
  writeRegister = (name: string, value: string, base: Base): void => {
    try {
      this.simulatorWasm.write_register(name, value, base);
      this.onChange();
    } catch (e) {
      console.log(e); // TODO: ???
    }
  };

  buses = (kind: "Intern" | "Input"): string[] =>
    this.simulatorWasm.buses(kind);
  busValue = (name: string, base: Base): string =>
    this.simulatorWasm.bus_value(name, base);
  writeBus = (name: string, value: string, base: Base): void => {
    try {
      this.simulatorWasm.write_bus(name, value, base);
      this.onChange();
    } catch (e) {
      console.log(e); // TODO: ???
    }
  };

  registerArrays = (): string[] => this.simulatorWasm.register_arrays();
  registerArrayPageCount = (name: string): number =>
    this.simulatorWasm.register_array_page_count(name);
  registerArrayPage = (
    name: string,
    pageNr: number,
    base: Base
  ): { idx: number; value: string }[] => {
    // Page returned from wasm is in the form:
    // [idx, value, idx, value, ...]
    const pageRaw = this.simulatorWasm.register_array_page(name, pageNr, base);

    // Map to [{idx, value}, ...] form
    let page: { idx: number; value: string }[] = [];
    for (let i = 0; i < pageRaw.length; i += 2) {
      page.push({ idx: pageRaw[i], value: pageRaw[i + 1] });
    }

    return page;
  };
  writeRegisterArray = (
    name: string,
    idx: number,
    value: string,
    base: Base
  ): void => {
    try {
      this.simulatorWasm.write_register_array(name, idx, value, base);
      this.onChange();
    } catch (e) {
      console.log(e); // TODO: ???
    }
  };

  memories = (): string[] => this.simulatorWasm.memories();
  memoryPageCount = (name: string): string =>
    this.simulatorWasm.memory_page_count(name);
  memoryPagePrev = (name: string, pageNr: string): string | null =>
    this.simulatorWasm.memory_page_prev(name, pageNr) ?? null;
  memoryPageNext = (name: string, pageNr: string): string | null =>
    this.simulatorWasm.memory_page_next(name, pageNr) ?? null;
  memoryPage = (
    name: string,
    pageNr: string,
    base: Base
  ): { address: string; value: string }[] => {
    // Page returned from wasm is in the form:
    // [addr, value, addr, value, ...]
    const pageRaw = this.simulatorWasm.memory_page(name, pageNr, base);

    // Map to [{addr, value}, ...] form
    let page: { address: string; value: string }[] = [];
    for (let i = 0; i < pageRaw.length; i += 2) {
      page.push({ address: pageRaw[i], value: pageRaw[i + 1] });
    }

    return page;
  };
  writeMemory = (
    name: string,
    address: string,
    value: string,
    base: Base
  ): void => {
    try {
      this.simulatorWasm.write_memory(name, address, value, base);
      this.onChange();
    } catch (e) {
      console.log(e); // TODO: ???
    }
  };
  memorySave = (name: string): string => this.simulatorWasm.save_memory(name);
  memoryLoadFromSave = (name: string, save: string): void => {
    try {
      this.simulatorWasm.load_memory_from_save(name, save);
      this.onChange();
    } catch (e) {
      console.log(e); // TODO: ???
    }
  };
}

export const baseValues = ["BIN", "DEC", "HEX"] as const;
export type Base = typeof baseValues[number];
export const isBase = (x: any): x is Base => baseValues.includes(x);

export interface Signals {
  conditionSignals: String[];
  controlSignals: String[];
}

export interface SimState {
  statement: number;
  span: Range;
  isAtBreakpoint: boolean;
  isStatementEnd: boolean;
  markerCurrent: SimStateMarker | null;
  marker: SimStateMarker[];
}

export interface SimStateMarker {
  kind: "True" | "False" | "Breakpoint" | "AssertError";
  span: Range;
}

// ------------ HELPER ------------

function getSignals(simulatorWasm: wasm.Simulator): Signals {
  const signalsWasm = simulatorWasm.signals();
  const signals: Signals = {
    conditionSignals: signalsWasm.condition_signals(),
    controlSignals: signalsWasm.control_signals(),
  };
  signalsWasm.free();
  return signals;
}

function calcRange(sourceCode: string, span: wasm.Span): Range {
  let startLineNumber = 1;
  let startColumn = 1;
  let endLineNumber = 1;
  let endColumn = 1;

  for (let i = 0; i < sourceCode.length && i < span.end; i++) {
    if (sourceCode.charAt(i) === "\n") {
      if (i < span.start) {
        startLineNumber++;
        startColumn = 1;
        endColumn = 1;
      } else {
        endColumn = 1;
      }
      endLineNumber++;
    } else {
      if (i < span.start) startColumn++;
      endColumn++;
    }
  }

  return new Range(startLineNumber, startColumn, endLineNumber, endColumn);
}

interface SimulatorStepParams {
  simulator: wasm.Simulator;
  currSimState: SimState | null;
  micro: boolean;
  stopOnBreakpoint: boolean;
  sourceCode: string;
}

function simulatorStep({
  simulator,
  currSimState,
  micro,
  stopOnBreakpoint,
  sourceCode,
}: SimulatorStepParams): SimState | null {
  // Check is finished
  if (simulator.is_finished()) return currSimState;

  // Call step
  const stepResult = micro
    ? simulator.micro_step(stopOnBreakpoint) ?? null
    : simulator.step(stopOnBreakpoint) ?? null;
  if (stepResult === null) return null;

  // ...
  const span = calcRange(sourceCode, stepResult.span);
  let isAtBreakpoint = false;

  // Calc marker
  let markerCurrent: SimStateMarker | null = null;
  if (stepResult.is_condition()) {
    const stepResultCondition = stepResult.as_condition()!;
    markerCurrent = {
      kind: stepResultCondition.result ? "True" : "False",
      span: calcRange(sourceCode, stepResultCondition.span),
    };
  } else if (stepResult.is_breakpoint()) {
    isAtBreakpoint = true;
    markerCurrent = {
      kind: "Breakpoint",
      span,
    };
  } else if (stepResult.is_assert_error()) {
    markerCurrent = {
      kind: "AssertError",
      span,
    };
  }

  // Next sim state
  const nextSimState: SimState = {
    statement: stepResult.statement,
    span,
    isAtBreakpoint,
    isStatementEnd: stepResult.is_statement_end(),
    markerCurrent,
    marker: currSimState?.isStatementEnd
      ? []
      : currSimState?.markerCurrent
      ? [...currSimState.marker, currSimState.markerCurrent]
      : currSimState?.marker ?? [],
  };

  // Free wasm object
  stepResult.free();

  return nextSimState;
}
