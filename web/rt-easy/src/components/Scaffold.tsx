import React, { useState, useContext /*, useReducer*/ } from "react";

import { RtEasyContext, GlobalContext, GlobalModel } from "../context";
import { Toolbar } from "./";
import { EditPage, RunPage } from "../pages";
import { Simulator, Span } from "../wasm";

interface Props {}

const Scaffold: React.FC<Props> = () => {
  const rtEasy = useContext(RtEasyContext);
  const [state, setState] = useState<State>({
    tag: "Edit",
    sourceCode: localStorage.getItem("source-code") || "",
    log: "--- ok ---",
  });
  // const [, forceUpdate] = useReducer((x) => x + 1, 0);

  let globalModel: GlobalModel;
  let page: React.ReactNode;

  switch (state.tag) {
    case "Edit":
      globalModel = {
        tag: "Edit",
        sourceCode: state.sourceCode,
        log: state.log,
        setSourceCode: (sourceCode) => {
          let log: string;
          try {
            rtEasy.check(sourceCode);
            log = "--- ok ---";
          } catch (e) {
            log = e;
          }

          localStorage.setItem("source-code", sourceCode);
          setState({ ...state, sourceCode, log });
        },
        build: () => {
          try {
            const simulator = rtEasy.build(state.sourceCode);
            setState({
              tag: "Run",
              sourceCode: state.sourceCode,
              simulator,
              currSpan: null,
              timerId: null,
            });
          } catch (e) {
            alert(e);
          }
        },
      };
      page = <EditPage />;
      break;
    case "Run":
      globalModel = {
        tag: "Run",
        sourceCode: state.sourceCode,
        goToEditMode: () => {
          if (state.timerId !== null) clearInterval(state.timerId);
          state.simulator.free();
          setState({
            tag: "Edit",
            sourceCode: state.sourceCode,
            log: "",
          });
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
                  return { ...prev, timerId: null, currSpan: null };
                });
                return;
              }

              const currSpan = state.simulator.step() ?? null;
              setState((prev) => {
                (prev as any).currSpan?.free();
                return { ...prev, currSpan };
              });
            }, 300);
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
        buses: () => state.simulator.buses(),
        busValue: (name: string, base: string) =>
          state.simulator.bus_value(name, base),
      };
      page = <RunPage />;
      break;
  }

  return (
    <GlobalContext.Provider value={globalModel}>
      <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
        <div style={{ flex: "0 0 32px", overflow: "hidden" }}>
          <Toolbar />
        </div>
        <div style={{ flex: "1", position: "relative" }}>{page}</div>
      </div>
    </GlobalContext.Provider>
  );
};

export default Scaffold;

type State = StateEdit | StateRun;

interface StateEdit {
  tag: "Edit";
  sourceCode: string;
  log: string;
}

interface StateRun {
  tag: "Run";
  sourceCode: string;
  currSpan: Span | null;
  simulator: Simulator;
  timerId: NodeJS.Timeout | null;
}
