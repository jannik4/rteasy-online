import React, { useContext, useRef } from "react";
import MonacoEditor, { Monaco } from "@monaco-editor/react";
import * as monaco from "monaco-editor";
import { languages } from "monaco-editor";

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
        value={globalModel.sourceCode}
        onChange={(value) => {
          if (globalModel.tag === "Edit") {
            globalModel.setSourceCode(value || "");
          }
        }}
        language={"rt-easy"}
        options={{
          fixedOverflowWidgets: true,
          readOnly: globalModel.tag === "Run",
        }}
        onMount={(editor, monaco) => {
          editorRef.current = editor;
          setUpLang(monaco);
        }}
      />
    </div>
  );
};

export default EditorView;

function setUpLang(monaco: Monaco) {
  monaco.languages.register({ id: "rt-easy" });
  monaco.languages.setMonarchTokensProvider("rt-easy", {
    keywords: [
      "declare",
      "goto",
      "nop",
      "read",
      "write",
      "if",
      "then",
      "else",
      "fi",
      "switch",
      "case",
      "default",
    ],
    typeKeywords: ["register", "bus", "memory", "array"],
    operators: ["=", "<>", "+", "-", "xor", "not", "sxt"],

    // C# style strings
    escapes:
      /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

    brackets: [
      { open: "[", close: "]", token: "delimiter.bracket" },
      { open: "(", close: ")", token: "delimiter.parenthesis" },
    ],

    tokenizer: {
      root: [
        // identifiers and keywords
        [
          /[a-z_$][\w$]*/,
          {
            cases: {
              "@typeKeywords": "keyword",
              "@keywords": "keyword",
              "@default": "identifier",
            },
          },
        ],

        [/[{}[]()]/, "@brackets"],

        // whitespace
        { include: "@whitespace" },

        // numbers
        [/0[xX][0-9a-fA-F]+/, "number.hex"], // hex
        [/0[bB][01]+/, "number.hex"], // binary: use hex style
        [/[0-9_]+/, "number"],

        // strings
        [/"([^"\\]|\\.)*$/, "string.invalid"], // non-teminated string
        [/"/, { token: "string.quote", next: "@string" }],

        // characters
        [/'[^\\']'/, "string"],
        [/(')(@escapes)(')/, ["string", "string.escape", "string"]],
        [/'/, "string.invalid"],
      ],

      string: [
        [/[^\\"]+/, "string"],
        [/@escapes/, "string.escape"],
        [/\\./, "string.escape.invalid"],
        [/"/, { token: "string.quote", next: "@pop" }],
      ],

      whitespace: [
        [/[ \t\r\n]+/, "white"],
        [/(^#.*$)/, "comment"],
      ],
    },
  });
  monaco.languages.registerCompletionItemProvider("rt-easy", {
    provideCompletionItems: (model, position) => {
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };

      const suggestions: languages.CompletionItem[] = [
        {
          label: "declare register",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: "declare register ",
          range,
        },
        {
          label: "declare bus",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: "declare bus ",
          range,
        },
        {
          label: "declare memory",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: "declare memory ",
          range,
        },
        {
          label: "ifelse",
          kind: monaco.languages.CompletionItemKind.Snippet,
          insertText: [
            // eslint-disable-next-line
            "if ${1:condition} then",
            "\t$0",
            "else",
            "\t",
            "fi",
          ].join("\n"),
          insertTextRules:
            monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          documentation: "If-Else Statement",
          range,
        },
      ];

      return { suggestions };
    },
  });
}
