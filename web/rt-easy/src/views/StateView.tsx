import React, { useState, useMemo, useContext } from "react";
import { HTMLTable, HTMLSelect, Text, Button } from "@blueprintjs/core";
import { DockLocation, Orientation } from "flexlayout-react";

import { InputValue, Focused } from "../components";
import { GlobalContext } from "../global/context";
import { LayoutModelContext, LayoutModel } from "../layout/context";
import * as consts from "../layout/consts";
import { baseValues, isBase } from "../wasm";

interface Props {}

const StateView: React.FC<Props> = () => {
  // Context and state
  const [focused, setFocused] = useState<Focused | null>(null);
  const globalModel = useContext(GlobalContext);
  const layoutModel = useContext(LayoutModelContext);

  // Names
  const [inputs, outputs, registers, buses, registerArrays, memories] =
    useMemo(() => {
      if (globalModel.tag === "Edit") return [[], [], [], [], [], []];
      return [
        globalModel.simulator.buses("Input"),
        globalModel.simulator.registers("Output"),
        globalModel.simulator.registers("Intern"),
        globalModel.simulator.buses("Intern"),
        globalModel.simulator.registerArrays(),
        globalModel.simulator.memories(),
      ];
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [globalModel.tag]);

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  const headerRow = (title: string) => (
    <tr style={{ backgroundColor: "#f2f2f2" }}>
      <td colSpan={2}>{title}:</td>
    </tr>
  );
  const dividerRow = (
    <tr key="dividerRow">
      <td colSpan={2}>&nbsp;</td>
    </tr>
  );

  const registerRows = (names: string[]) =>
    names.length !== 0
      ? names.map((name) => (
          <tr key={name}>
            <td>{name}</td>
            <td>
              <InputValue
                withBaseSelect
                focused={focused}
                setFocused={setFocused}
                inputKey={name}
                highlight={
                  globalModel.simulator
                    .getSimState()
                    ?.changed?.registers.has(name) ?? false
                }
                value={(base) =>
                  globalModel.simulator.registerValue(name, base)
                }
                valueNext={(base) =>
                  globalModel.simulator.registerValueNext(name, base)
                }
                onChanged={(value, base) =>
                  globalModel.simulator.writeRegister(name, value, base)
                }
              />
            </td>
          </tr>
        ))
      : [dividerRow];

  const busRows = (names: string[]) =>
    names.length !== 0
      ? names.map((name) => (
          <tr key={name}>
            <td>{name}</td>
            <td>
              <InputValue
                withBaseSelect
                focused={focused}
                setFocused={setFocused}
                inputKey={name}
                highlight={false}
                value={(base) => globalModel.simulator.busValue(name, base)}
                valueNext={null}
                onChanged={(value, base) =>
                  globalModel.simulator.writeBus(name, value, base)
                }
              />
            </td>
          </tr>
        ))
      : [dividerRow];

  return (
    <div style={{ padding: "16px 8px" /*, overflow: "hidden"*/ }}>
      <div
        style={{
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <HTMLSelect
          value={globalModel.base}
          onChange={(e) => {
            if (!isBase(e.target.value)) throw new Error("invalid value");
            globalModel.setBase(e.target.value);
          }}
          minimal
        >
          {baseValues.map((base) => (
            <option key={base} value={base}>
              {base}
            </option>
          ))}
        </HTMLSelect>
        <Text>Cycle count: {globalModel.simulator.cycleCount()}</Text>
      </div>

      <div style={{ height: 16 }} />

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Identifier</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {headerRow("Inputs")}
          {busRows(inputs)}

          {headerRow("Outputs")}
          {registerRows(outputs)}

          {headerRow("Registers")}
          {registerRows(registers)}

          {headerRow("Buses")}
          {busRows(buses)}

          {headerRow("Register arrays")}
          {registerArrays.length !== 0
            ? registerArrays.map((registerArray) => (
                <tr key={registerArray}>
                  <td>{registerArray}</td>
                  <td>
                    <Button
                      small
                      onClick={() => {
                        // Select if exists
                        const registerArrayStateId =
                          consts.ID_TAB_STATE_REGISTER_ARRAY(registerArray);
                        if (layoutModel.selectTab(registerArrayStateId)) {
                          return;
                        }

                        // Find position
                        const position = findPosition(layoutModel);
                        if (position === null) return;

                        // Create tab
                        layoutModel.createTab(
                          registerArrayStateId,
                          `Register array (${registerArray})`,
                          `register-array-${registerArray}`,
                          position.toNodeId,
                          position.location
                        );
                      }}
                    >
                      Content
                    </Button>
                  </td>
                </tr>
              ))
            : [dividerRow]}

          {headerRow("Memories")}
          {memories.length !== 0
            ? memories.map((memory) => (
                <tr key={memory}>
                  <td>{memory}</td>
                  <td>
                    <Button
                      small
                      onClick={() => {
                        // Select if exists
                        const memoryStateId =
                          consts.ID_TAB_STATE_MEMORY(memory);
                        if (layoutModel.selectTab(memoryStateId)) {
                          return;
                        }

                        // Find position
                        const position = findPosition(layoutModel);
                        if (position === null) return;

                        // Create tab
                        layoutModel.createTab(
                          memoryStateId,
                          `Memory (${memory})`,
                          `memory-${memory}`,
                          position.toNodeId,
                          position.location
                        );
                      }}
                    >
                      Content
                    </Button>
                  </td>
                </tr>
              ))
            : [dividerRow]}
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default StateView;

function findPosition(
  layoutModel: LayoutModel
): { toNodeId: string; location: DockLocation } | null {
  // Try to find "state extra tab"
  const stateExtraTabParentId = layoutModel
    .findOne((n) => n.getId().includes(consts.MARKER_STATE_EXTRA))
    ?.getParent()
    ?.getId();
  if (stateExtraTabParentId !== undefined) {
    return { toNodeId: stateExtraTabParentId, location: DockLocation.CENTER };
  }

  // Get state tab and parent
  const stateTab = layoutModel.getNodeById(consts.ID_TAB_STATE);
  if (stateTab === undefined) return null;
  const stateTabParent = stateTab.getParent();
  if (stateTabParent === undefined) return null;

  return {
    toNodeId: stateTabParent.getId(),
    location:
      stateTabParent.getOrientation() === Orientation.HORZ
        ? DockLocation.RIGHT
        : DockLocation.BOTTOM,
  };
}
