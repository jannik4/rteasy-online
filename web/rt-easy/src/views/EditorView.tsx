import React, { useContext, useRef, useEffect, useCallback } from "react";
import MonacoEditor from "@monaco-editor/react";
import * as monaco from "monaco-editor";

import { GlobalContext, GlobalModelRun } from "../global/context";

interface Props {}

const EditorView: React.FC<Props> = () => {
  const oldDecorations = useRef<string[]>([]);
  const globalModel = useContext(GlobalContext);

  if (globalModel.editor !== null && globalModel.tag === "Run") {
    const simState = globalModel.simulator.getSimState();
    let decorations = [];

    // Breakpoints (TODO: Use memo to calc ranges)
    for (const breakpoint of globalModel.simulator.breakpoints()) {
      const statementRange = globalModel.simulator.statementRange(breakpoint);
      if (statementRange !== null) {
        decorations.push({
          range: new monaco.Range(
            statementRange.startLineNumber,
            1,
            statementRange.startLineNumber,
            1
          ),
          options: {
            glyphMarginClassName: "myGlyphMarginClass",
            glyphMarginHoverMessage: { value: "Breakpoint" },
          },
        });
      }
    }

    // Inline decorations from simState
    if (simState) {
      decorations.push({
        range: simState.span,
        options: {
          inlineClassName: "monacoInlineDecorationYellow",
        },
      });
      if (simState.markerCurrent) {
        decorations.push({
          range: simState.markerCurrent.span,
          options: {
            inlineClassName:
              "monacoInlineDecorationMarkerCurrent-" +
              simState.markerCurrent.kind,
          },
        });
      }
      for (const marker of simState.marker) {
        decorations.push({
          range: marker.span,
          options: {
            inlineClassName: "monacoInlineDecorationMarker-" + marker.kind,
          },
        });
      }
    }

    oldDecorations.current = globalModel.editor.deltaDecorations(
      oldDecorations.current,
      decorations
    );
  }

  // Listener: onMouseDown
  const onMouseDown = useCallback(
    (event: monaco.editor.IEditorMouseEvent) => {
      if (globalModel.tag === "Edit") return;

      const isBreakpointEvent =
        event.target.type ===
          monaco.editor.MouseTargetType.GUTTER_GLYPH_MARGIN ||
        event.target.type === monaco.editor.MouseTargetType.GUTTER_LINE_NUMBERS;
      const lineNumber = event.target.position?.lineNumber;

      if (isBreakpointEvent && lineNumber) {
        let breakpoint = calcBreakpoint(globalModel, lineNumber);

        if (breakpoint !== null) {
          // Toggle breakpoint
          if (globalModel.simulator.breakpoints().includes(breakpoint)) {
            globalModel.simulator.removeBreakpoint(breakpoint);
          } else {
            globalModel.simulator.addBreakpoint(breakpoint);
          }
        }
      }
    },
    [globalModel]
  );

  // Listener: onDidDispose
  const onDidDispose = useCallback(
    () => globalModel.setEditor(null),
    [globalModel]
  );

  // Add/Remove listeners
  useEffect(() => {
    if (globalModel.editor === null) return;

    // Add listeners
    const disposables = [
      globalModel.editor.onMouseDown(onMouseDown),
      globalModel.editor.onDidDispose(onDidDispose),
    ];

    // Remove listeners
    return () => {
      for (const disposable of disposables) {
        disposable.dispose();
      }
    };
  }, [globalModel.editor, onMouseDown, onDidDispose]);

  return (
    <div style={{ height: "100%" /*, overflow: "hidden"*/ }}>
      <MonacoEditor
        options={{
          fixedOverflowWidgets: true,
          glyphMargin: true,
          lineNumbersMinChars: 0,
          readOnly: globalModel.tag === "Run",
        }}
        keepCurrentModel
        onMount={(editor, _monaco) => globalModel.setEditor(editor)}
      />
    </div>
  );
};

export default EditorView;

function calcBreakpoint(
  globalModel: GlobalModelRun,
  lineNumber: number
): number | null {
  let breakpoint: number | null = null;
  for (let statement = 0; true; statement++) {
    // Get statement range
    const statementRange = globalModel.simulator.statementRange(statement);
    if (statementRange === null) break;

    // Check perfect match
    if (lineNumber === statementRange.startLineNumber) {
      breakpoint = statement;
      break;
    }

    // Check smaller than end line number
    // Only set if breakpoint is null, otherwise all subsequent statemens will overwrite this
    // Don't break, because there could still be a perfect matching statement
    if (lineNumber <= statementRange.endLineNumber && breakpoint === null) {
      breakpoint = statement;
    }
  }

  return breakpoint;
}
