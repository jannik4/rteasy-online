import React from "react";
import { Range } from "monaco-editor";

import { RtEasy, Span, StepResult } from "../../wasm";
import { GlobalModelRun, SimState } from "../context";
import { State, StateRun } from "../state";

export function model(
  _rtEasy: RtEasy,
  state: StateRun,
  setState: React.Dispatch<React.SetStateAction<State>>
): GlobalModelRun {
  const goToEditMode = (sourceCode?: string) => {
    if (state.timerId !== null) clearInterval(state.timerId);
    state.simulator.free();
    setState({
      tag: "Edit",
      sourceCode: sourceCode ?? state.sourceCode,
      base: state.base,
      log: "",
    });
  };

  return {
    tag: "Run",
    sourceCode: state.sourceCode,
    toggleMode: () => goToEditMode(),
    base: state.base,
    setBase: (base) => setState({ ...state, base }),
    goToEditMode,
    reset: () => {
      if (state.timerId !== null) clearInterval(state.timerId);
      state.simulator.reset();
      setState({ ...state, simState: null, timerId: null });
    },
    isFinished: () => state.simulator.is_finished(),
    microStep: () => {
      const stepResult = state.simulator.micro_step() ?? null;
      const simState = calcNextSimState(
        state.sourceCode,
        state.simState,
        stepResult
      );
      setState({ ...state, simState });
    },
    step: () => {
      const stepResult = state.simulator.step() ?? null;
      const simState = calcNextSimState(
        state.sourceCode,
        state.simState,
        stepResult
      );
      setState({ ...state, simState });
    },
    simState: state.simState,

    runStop: () => {
      if (state.timerId === null) {
        const timerId = setInterval(() => {
          if (state.simulator.is_finished()) {
            clearInterval(timerId);
            setState((prev) => {
              return { ...prev, timerId: null, simState: null };
            });
            return;
          }

          // Run for x ms
          let simState = state.simState;
          let start = performance.now();
          const MS = 5;
          while (true) {
            const stepResult = state.simulator.step() ?? null;
            simState = calcNextSimState(state.sourceCode, simState, stepResult);

            if (performance.now() - start > MS) break;
          }

          setState((prev) => {
            return { ...prev, simState };
          });
        }, 10);
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
    writeIntoRegister: (name: string, value: string, base: string) => {
      try {
        state.simulator.write_into_register(name, value, base);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },
    buses: (kind: "Intern" | "Input") => state.simulator.buses(kind),
    busValue: (name: string, base: string) =>
      state.simulator.bus_value(name, base),
    writeIntoBus: (name: string, value: string, base: string) => {
      try {
        state.simulator.write_into_bus(name, value, base);
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
    writeIntoMemory: (
      name: string,
      address: string,
      value: string,
      base: string
    ) => {
      try {
        state.simulator.write_into_memory(name, address, value, base);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },
    memorySave: (name: string) => state.simulator.memory_save(name),
    memoryLoadFromSave: (name: string, save: string) => {
      try {
        state.simulator.memory_load_from_save(name, save);
        setState({ ...state }); // Force state update
      } catch (e) {
        console.log(e); // TODO: ???
      }
    },
  };
}

function calcNextSimState(
  sourceCode: string,
  currSimState: SimState | null,
  stepResult: StepResult | null
): SimState | null {
  if (stepResult === null) return null;

  const nextSimState = {
    span: calcRange(sourceCode, stepResult.span),
    currCondition: stepResult.condition
      ? {
          value: stepResult.condition.value,
          span: calcRange(sourceCode, stepResult.condition.span),
        }
      : null,
    conditions: stepResult.is_at_statement_start
      ? []
      : currSimState?.currCondition
      ? [...currSimState.conditions, currSimState.currCondition]
      : currSimState?.conditions ?? [],
  };

  stepResult.free();
  return nextSimState;
}

function calcRange(sourceCode: string, span: Span): Range {
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
