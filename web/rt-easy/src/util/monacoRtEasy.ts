import { Monaco } from "@monaco-editor/react";
import { languages, Position, editor } from "monaco-editor";

export function setUpRtEasyLang(monaco: Monaco) {
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
      "assert",
    ],
    typeKeywords: ["input", "output", "register", "bus", "memory", "array"],
    operators: [
      "=",
      "<>",
      "<=",
      "<",
      ">=",
      ">",
      "+",
      "-",
      "and",
      "nand",
      "or",
      "nor",
      "xor",
      "neg",
      "not",
      "sxt",
    ],

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
          /[a-zA-Z_][\w]*/,
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
        [/(\$|0[xX])[0-9a-fA-F]+/, "number.hex"], // hex
        [/(%|0[bB])[01]+/, "number.hex"], // binary: use hex style
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

      const completionIdents: languages.CompletionItem[] = [];
      for (const ident of findAllIdents(model, position)) {
        completionIdents.push({
          label: ident,
          kind: monaco.languages.CompletionItemKind.Variable,
          insertText: ident,
          range,
        });
      }

      const completionKeyWord = (
        keyword: string,
        more?: boolean
      ): languages.CompletionItem => {
        return {
          label: keyword,
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: keyword + (more ? " " : ""),
          range,
          command: more
            ? {
                id: "editor.action.triggerSuggest",
                title: "More",
              }
            : undefined,
        };
      };
      const completionOperator = (
        operator: string
      ): languages.CompletionItem => {
        return {
          label: operator,
          kind: monaco.languages.CompletionItemKind.Operator,
          insertText: operator,
          range,
        };
      };

      const suggestions: languages.CompletionItem[] = [
        // Idents
        ...completionIdents,

        // Declare
        completionKeyWord("declare", true),
        completionKeyWord("input"),
        completionKeyWord("output"),
        completionKeyWord("register"),
        completionKeyWord("bus"),
        completionKeyWord("memory"),
        completionKeyWord("register array"),

        // Other keywords
        completionKeyWord("goto"),
        completionKeyWord("nop"),
        completionKeyWord("read"),
        completionKeyWord("write"),
        completionKeyWord("if"),
        completionKeyWord("then"),
        completionKeyWord("else"),
        completionKeyWord("fi"),
        completionKeyWord("switch"),
        completionKeyWord("case"),
        completionKeyWord("default"),
        completionKeyWord("assert"),

        // Operators
        completionOperator("and"),
        completionOperator("nand"),
        completionOperator("or"),
        completionOperator("nor"),
        completionOperator("xor"),
        completionOperator("neg"),
        completionOperator("not"),
        completionOperator("sxt"),

        // Snippets
        {
          label: "if",
          kind: monaco.languages.CompletionItemKind.Snippet,
          // eslint-disable-next-line
          insertText: "if ${1:condition} then $0 fi",
          insertTextRules:
            monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          documentation: "If Statement",
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
        {
          label: "switch",
          kind: monaco.languages.CompletionItemKind.Snippet,
          insertText: [
            // eslint-disable-next-line
            "switch ${1:key} {",
            // eslint-disable-next-line
            "\tcase ${2:value}: $3",
            // eslint-disable-next-line
            "\tdefault: ${4:nop}",
            "}",
          ].join("\n"),
          insertTextRules:
            monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          documentation: "Switch Statement",
          range,
        },
      ];

      return { suggestions };
    },
  });
}

function findAllIdents(model: editor.ITextModel, position: Position): string[] {
  // Find all matches
  const matches = model.findMatches(
    "[A-Z_][A-Z0-9_]*(?![a-z])",
    false,
    true,
    true,
    null,
    true
  );

  // Collect into set
  const identsSet: Set<string> = new Set();
  for (const match of matches) {
    // Skip if cursor is inside match
    if (match.range.containsPosition(position)) continue;

    if (match.matches) identsSet.add(match.matches[0]);
  }

  // Collect and sort into array
  const idents: string[] = [];
  identsSet.forEach((e) => idents.push(e));
  idents.sort();

  return idents;
}
