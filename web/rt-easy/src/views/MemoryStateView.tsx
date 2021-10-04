import React, { useState, useContext, useMemo, useRef, useEffect } from "react";
import { HTMLTable, Text, Button } from "@blueprintjs/core";
import { showErrorToast } from "../toaster";

import {
  InputValue,
  Focused,
  BaseInheritSelect,
  GotoDialog,
} from "../components";
import { useFilePicker } from "../hooks/useFilePicker";
import { downloadFile } from "../util/downloadFile";
import { GlobalContext, BaseInherit } from "../global/context";

interface Props {
  memory: string;
}

const MemoryStateView: React.FC<Props> = ({ memory }) => {
  // Context and state
  const [showGotoDialog, setShowGotoDialog] = useState(false);
  const [gotoAddress, setGotoAddress] = useState<[string | null]>([null]); // Use array here => gotoAddress object is always different => effect to scroll into view runs every time
  const gotoElementRef = useRef<HTMLTableCellElement | null>(null);

  const globalModel = useContext(GlobalContext);
  const [pageNr, setPageNr] = useState("1");
  const [focused, setFocused] = useState<Focused | null>(null);
  const [baseInherit, setBaseInherit] = useState<BaseInherit>(() => {
    if (globalModel.tag === "Run") {
      return globalModel.inheritBasesStorage.current.get(memory) ?? "Inherit";
    }
    return "Inherit";
  });
  const base = baseInherit === "Inherit" ? globalModel.base : baseInherit;

  // Page count
  const pageCount = useMemo(() => {
    if (globalModel.tag === "Edit") return "";
    return globalModel.simulator.memoryPageCount(memory);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [globalModel.tag, memory]);

  // File picker
  const openLoadFromSaveFilePicker = useFilePicker({
    accept: [".rtmem"],
    onChange: (_name, content) => {
      if (globalModel.tag === "Edit") return;
      globalModel.simulator.memoryLoadFromSave(memory, content);
    },
  });

  useEffect(() => {
    if (gotoElementRef.current) gotoElementRef.current.scrollIntoView();
  }, [gotoAddress]);

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  const valueNext = globalModel.simulator.memoryValueNext(memory, base);

  return (
    <div style={{ height: "100%", overflow: "auto" }}>
      <GotoDialog
        isOpen={showGotoDialog}
        onClose={() => setShowGotoDialog(false)}
        onGoto={(address) => {
          try {
            const pageNr = globalModel.simulator.memoryPageNrOfAddress(
              memory,
              address
            );
            if (pageNr === null) {
              showErrorToast({ message: "Address out of range" });
            } else {
              setGotoAddress([address.toUpperCase()]);
              setPageNr(pageNr);
              setShowGotoDialog(false);
            }
          } catch (e) {
            showErrorToast({ message: e as string });
          }
        }}
      />
      <div
        style={{ height: "100%", padding: "0 8px" /*, overflow: "hidden"*/ }}
      >
        <div style={{ height: 16 }} />

        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
          }}
        >
          <BaseInheritSelect
            value={baseInherit}
            onChange={(baseInherit) => {
              if (globalModel.tag === "Run") {
                globalModel.inheritBasesStorage.current.set(
                  memory,
                  baseInherit
                );
              }
              setBaseInherit(baseInherit);
            }}
          />
          <Button onClick={() => setShowGotoDialog(true)} small>
            Goto
          </Button>
          <div
            style={{
              display: "flex",
              alignItems: "center",
              // marginLeft: "auto",
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
                  globalModel.simulator.memorySave(memory)
                )
              }
              small
            >
              Save
            </Button>
          </div>
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
              const pageNrPrev = globalModel.simulator.memoryPagePrev(
                memory,
                pageNr
              );
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
              const pageNrNext = globalModel.simulator.memoryPageNext(
                memory,
                pageNr
              );
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
            {globalModel.simulator
              .memoryPage(memory, pageNr, base)
              .map((row) => (
                <tr key={row.address}>
                  <td>{row.address}</td>
                  <td
                    ref={
                      row.address === gotoAddress[0]
                        ? gotoElementRef
                        : undefined
                    }
                  >
                    <InputValue
                      focused={focused}
                      setFocused={setFocused}
                      inputKey={row.address}
                      highlight={
                        globalModel.simulator
                          .getSimState()
                          ?.changed?.memories.get(memory) === row.address
                      }
                      value={() => row.value}
                      valueNext={() =>
                        row.address === valueNext?.address
                          ? valueNext.value
                          : null
                      }
                      onChanged={(value: string) =>
                        globalModel.simulator.writeMemory(
                          memory,
                          row.address,
                          value,
                          base
                        )
                      }
                    />
                  </td>
                </tr>
              ))}
          </tbody>
        </HTMLTable>
      </div>
    </div>
  );
};

export default MemoryStateView;
