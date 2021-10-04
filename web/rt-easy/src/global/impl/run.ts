import React from "react";
import { editor } from "monaco-editor";

import { calcRange } from "../../util/convertRangeSpan";
import { RtEasy, Simulator } from "../../wasm";
import {
  GlobalModelRun,
  SimState,
  SimStateMarker,
  BaseInherit,
} from "../context";
import { model as modelCommon } from "./common";
import { State, StateRun } from "../state";

export function model(
  _rtEasy: RtEasy,
  state: StateRun,
  setState: React.Dispatch<React.SetStateAction<State>>,
  editorModel: editor.IModel,
  inheritBasesStorage: React.MutableRefObject<Map<string, BaseInherit>>
): GlobalModelRun {
  const goToEditMode = () => {
    if (state.timerId !== null) clearInterval(state.timerId);
    state.simulator.free();
    setState({
      tag: "Edit",
      editor: state.editor,
      base: state.base,
      clockRate: state.clockRate,
    });
  };

  return {
    ...modelCommon(state, setState, editorModel, goToEditMode),
    tag: "Run",
    goToEditMode,
    reset: () => {
      if (state.timerId !== null) clearInterval(state.timerId);
      state.simulator.reset();
      setState({ ...state, simState: null, timerId: null });
    },
    isFinished: () => state.simulator.is_finished(),
    microStep: () => {
      const simState = simulatorStep({
        simulator: state.simulator,
        currSimState: state.simState,
        micro: true,
        stopOnBreakpoint: false,
        sourceCode: editorModel.getValue(),
      });
      setState({ ...state, simState });
    },
    step: () => {
      const simState = simulatorStep({
        simulator: state.simulator,
        currSimState: state.simState,
        micro: false,
        stopOnBreakpoint: false,
        sourceCode: editorModel.getValue(),
      });
      setState({ ...state, simState });
    },
    simState: state.simState,

    signals: () => {
      const signalsWasm = state.simulator.signals();
      const signals = {
        conditionSignals: signalsWasm.condition_signals(),
        controlSignals: signalsWasm.control_signals(),
      };
      signalsWasm.free();
      return signals;
    },
    statementRange: (statement: number) => {
      const span = state.simulator.statement_span(statement);
      if (span === undefined) return null;
      const range = calcRange(editorModel.getValue(), span);
      span.free();
      return range;
    },
    addBreakpoint: (statement: number) => {
      state.simulator.add_breakpoint(statement);
      setState({ ...state });
    },
    removeBreakpoint: (statement: number) => {
      state.simulator.remove_breakpoint(statement);
      setState({ ...state });
    },
    breakpoints: () => Array.from(state.simulator.breakpoints()),
    runStop: () => {
      if (state.timerId === null) {
        if (state.simulator.is_finished()) return;
        const intervalSleep =
          state.clockRate === "Max" ? 10 : 1000 / state.clockRate;
        const timerId = setInterval(() => {
          // Next sim state
          let simState = state.simState;

          // Run
          if (state.clockRate === "Max") {
            // Run for _ ms
            let start = performance.now();
            const MS = 5;
            while (true) {
              simState = simulatorStep({
                simulator: state.simulator,
                currSimState: simState,
                micro: false,
                stopOnBreakpoint: true,
                sourceCode: editorModel.getValue(),
              });

              if (simState?.isAtBreakpoint) break;
              if (performance.now() - start > MS) break;
            }
          } else {
            // Run one step
            simState = simulatorStep({
              simulator: state.simulator,
              currSimState: simState,
              micro: false,
              stopOnBreakpoint: true,
              sourceCode: editorModel.getValue(),
            });
          }

          // Check finished or breakpoint
          if (state.simulator.is_finished() || simState?.isAtBreakpoint) {
            clearInterval(timerId);
            setState((prev) => {
              return { ...prev, timerId: null, simState };
            });
            return;
          }

          // Update state
          setState((prev) => {
            return { ...prev, simState };
          });
        }, intervalSleep);
        setState({ ...state, timerId });
      } else {
        clearInterval(state.timerId);
        setState({ ...state, timerId: null });
      }
    },
    isRunning: () => state.timerId !== null,

    cycleCount: () => state.simulator.cycle_count(),
    registers: (kind: "Intern" | "Output") => state.simulator.registers(kind),
    registerValue: (name: string, base: string) =>
      state.simulator.register_value(name, base),
    registerValueNext: (name: string, base: string) =>
      state.simulator.register_value_next(name, base) ?? null,
    writeRegister: (name: string, value: string, base: string) => {
      try {
        state.simulator.write_register(name, value, base);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },
    buses: (kind: "Intern" | "Input") => state.simulator.buses(kind),
    busValue: (name: string, base: string) =>
      state.simulator.bus_value(name, base),
    writeBus: (name: string, value: string, base: string) => {
      try {
        state.simulator.write_bus(name, value, base);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },
    registerArrays: () => state.simulator.register_arrays(),
    registerArrayPageCount: (name: string) =>
      state.simulator.register_array_page_count(name),
    registerArrayPage: (name: string, pageNr: number, base: string) => {
      // Page returned from wasm is in the form:
      // [idx, value, idx, value, ...]
      const pageRaw = state.simulator.register_array_page(name, pageNr, base);

      // Map to [{idx, value}, ...] form
      let page: { idx: number; value: string }[] = [];
      for (let i = 0; i < pageRaw.length; i += 2) {
        page.push({ idx: pageRaw[i], value: pageRaw[i + 1] });
      }

      return page;
    },
    writeRegisterArray: (
      name: string,
      idx: number,
      value: string,
      base: string
    ) => {
      try {
        state.simulator.write_register_array(name, idx, value, base);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },
    memories: () => state.simulator.memories(),
    memoryPageCount: (name: string) => state.simulator.memory_page_count(name),
    memoryPagePrev: (name: string, pageNr: string) =>
      state.simulator.memory_page_prev(name, pageNr) ?? null,
    memoryPageNext: (name: string, pageNr: string) =>
      state.simulator.memory_page_next(name, pageNr) ?? null,
    memoryPage: (name: string, pageNr: string, base: string) => {
      // Page returned from wasm is in the form:
      // [addr, value, addr, value, ...]
      const pageRaw = state.simulator.memory_page(name, pageNr, base);

      // Map to [{addr, value}, ...] form
      let page: { address: string; value: string }[] = [];
      for (let i = 0; i < pageRaw.length; i += 2) {
        page.push({ address: pageRaw[i], value: pageRaw[i + 1] });
      }

      return page;
    },
    writeMemory: (
      name: string,
      address: string,
      value: string,
      base: string
    ) => {
      try {
        state.simulator.write_memory(name, address, value, base);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },
    memorySave: (name: string) => state.simulator.save_memory(name),
    memoryLoadFromSave: (name: string, save: string) => {
      try {
        state.simulator.load_memory_from_save(name, save);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },

    inheritBasesStorage,
  };
}

interface SimulatorStepParams {
  simulator: Simulator;
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
