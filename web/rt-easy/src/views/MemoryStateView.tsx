import React, { useState, useContext, useMemo } from "react";
import { HTMLTable, Text, InputGroup, Button } from "@blueprintjs/core";

import { useFilePicker } from "../hooks/useFilePicker";
import { downloadFile } from "../util/downloadFile";
import { GlobalContext } from "../global/context";

interface Props {
  memory: string;
}

interface InputValue {
  key: string;
  value: string;
  valueNext: string | null;
  onChanged: (value: string) => void;
}

const MemoryStateView: React.FC<Props> = ({ memory }) => {
  // Context and state
  const globalModel = useContext(GlobalContext);
  const [pageNr, setPageNr] = useState("1");
  const [focused, setFocused] = useState<InputValue | null>(null);

  // Page count
  const pageCount = useMemo(() => {
    if (globalModel.tag === "Edit") return "";
    return globalModel.memoryPageCount(memory);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [globalModel.tag, memory]);

  // File picker
  const openLoadFromSaveFilePicker = useFilePicker({
    accept: [".rtmem"],
    onChange: (_name, content) => {
      if (globalModel.tag === "Edit") return;
      console.log("ccc");
      globalModel.memoryLoadFromSave(memory, content);
    },
  });

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  const inputValue = (inputValue: InputValue) => {
    let value: string;
    if (focused?.key === inputValue.key) {
      value = focused.value;
    } else {
      value = inputValue.value;
      if (inputValue.valueNext !== null) {
        value += ` \u2794 ${inputValue.valueNext}`;
      }
    }

    return (
      <InputGroup
        small
        value={value}
        onChange={(e) => setFocused({ ...inputValue, value: e.target.value })}
        onFocus={() => setFocused(inputValue)}
        onBlur={() => {
          if (focused?.value !== inputValue.value) {
            focused?.onChanged(focused.value);
          }
          setFocused(null);
        }}
        onKeyDown={(e) => {
          if (e.key === "Enter") e.currentTarget.blur();
        }}
      />
    );
  };

  return (
    <div style={{ height: "100%", padding: "0 8px" /*, overflow: "hidden"*/ }}>
      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          alignItems: "center",
        }}
      >
        <Button onClick={() => openLoadFromSaveFilePicker()} small>
          Load
        </Button>
        <div style={{ width: 8 }} />
        <Button
          onClick={() =>
            downloadFile(
              `memory-${memory}.rtmem`,
              globalModel.memorySave(memory)
            )
          }
          small
        >
          Save
        </Button>
      </div>

      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <Button
          icon="arrow-left"
          onClick={() => {
            const pageNrPrev = globalModel.memoryPagePrev(memory, pageNr);
            if (pageNrPrev !== null) setPageNr(pageNrPrev);
          }}
          minimal
          small
        />
        <Text>
          {pageNr} / {pageCount}
        </Text>
        <Button
          icon="arrow-right"
          onClick={() => {
            const pageNrNext = globalModel.memoryPageNext(memory, pageNr);
            if (pageNrNext !== null) setPageNr(pageNrNext);
          }}
          minimal
          small
        />
      </div>

      <div style={{ height: 16 }} />

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Address</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {globalModel
            .memoryPage(memory, pageNr, globalModel.base)
            .map((row) => (
              <tr key={row.address}>
                <td>{row.address}</td>
                <td>
                  {inputValue({
                    key: row.address,
                    value: row.value,
                    valueNext: null,
                    onChanged: (value) =>
                      globalModel.writeIntoMemory(
                        memory,
                        row.address,
                        value,
                        globalModel.base
                      ),
                  })}
                </td>
              </tr>
            ))}
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default MemoryStateView;
