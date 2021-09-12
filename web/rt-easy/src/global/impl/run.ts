import React from "react";

import { RtEasy, Span } from "../../wasm";
import { GlobalModelRun } from "../context";
import { State, StateRun } from "../state";

export function model(
  _rtEasy: RtEasy,
  state: StateRun,
  setState: React.Dispatch<React.SetStateAction<State>>
): GlobalModelRun {
  return {
    tag: "Run",
    sourceCode: state.sourceCode,
    base: state.base,
    setBase: (base) => setState({ ...state, base }),
    goToEditMode: () => {
      if (state.timerId !== null) clearInterval(state.timerId);
      state.simulator.free();
      setState({
        tag: "Edit",
        sourceCode: state.sourceCode,
        base: state.base,
        log: "",
      });
    },
    reset: () => {
      if (state.timerId !== null) clearInterval(state.timerId);
      state.simulator.reset();
      state.currSpan?.free();
      setState({ ...state, currSpan: null, timerId: null });
    },
    isFinished: () => state.simulator.is_finished(),
    microStep: () => {
      const currSpan = state.simulator.micro_step() ?? null;
      state.currSpan?.free();
      setState({ ...state, currSpan });
    },
    step: () => {
      const currSpan = state.simulator.step() ?? null;
      state.currSpan?.free();
      setState({ ...state, currSpan });
    },
    currSpan: () => state.currSpan,

    runStop: () => {
      if (state.timerId === null) {
        const timerId = setInterval(() => {
          if (state.simulator.is_finished()) {
            clearInterval(timerId);
            setState((prev) => {
              if (prev.tag === "Run") prev.currSpan?.free();
              return { ...prev, timerId: null, currSpan: null };
            });
            return;
          }

          // Run for x ms
          let currSpan: Span | null = null;
          let start = performance.now();
          const MS = 5;
          while (true) {
            currSpan?.free();
            currSpan = state.simulator.step() ?? null;

            if (performance.now() - start > MS) break;
          }

          setState((prev) => {
            if (prev.tag === "Run") prev.currSpan?.free();
            return { ...prev, currSpan };
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
    registers: () => state.simulator.registers(),
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
    buses: () => state.simulator.buses(),
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
  };
}
