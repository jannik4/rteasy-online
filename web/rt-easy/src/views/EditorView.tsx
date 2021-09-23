import React, { useContext, useRef } from "react";
import MonacoEditor from "@monaco-editor/react";
import * as monaco from "monaco-editor";

import { GlobalContext } from "../global/context";

interface Props {}

const EditorView: React.FC<Props> = () => {
  const oldDecorations = useRef<string[]>([]);
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const globalModel = useContext(GlobalContext);

  if (editorRef.current !== null && globalModel.tag === "Run") {
    const simState = globalModel.simState;
    if (simState) {
      let decorations = [];
      decorations.push({
        range: simState.span,
        options: {
          inlineClassName: "monacoInlineDecorationYellow",
        },
      });
      if (simState.currCondition) {
        decorations.push({
          range: simState.currCondition.span,
          options: {
            inlineClassName: simState.currCondition.value
              ? "monacoInlineDecorationGreen"
              : "monacoInlineDecorationRed",
          },
        });
      }
      for (const condition of simState.conditions) {
        decorations.push({
          range: condition.span,
          options: {
            inlineClassName: condition.value
              ? "monacoInlineDecorationLightGreen"
              : "monacoInlineDecorationLightRed",
          },
        });
      }

      oldDecorations.current = editorRef.current.deltaDecorations(
        oldDecorations.current,
        decorations
      );
    } else {
      oldDecorations.current = editorRef.current.deltaDecorations(
        oldDecorations.current,
        []
      );
    }
  }

  return (
    <div style={{ height: "100%" /*, overflow: "hidden"*/ }}>
      <MonacoEditor
        options={{
          fixedOverflowWidgets: true,
          readOnly: globalModel.tag === "Run",
        }}
        keepCurrentModel
        onMount={(editor, _monaco) => {
          editorRef.current = editor;
          editorRef.current.setModel(globalModel.editorModel);
        }}
      />
    </div>
  );
};

export default EditorView;
