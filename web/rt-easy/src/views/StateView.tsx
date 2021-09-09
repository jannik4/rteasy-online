import React, { useState, useContext } from "react";
import {
  HTMLTable,
  HTMLSelect,
  Text,
  H5,
  InputGroup,
  Button,
} from "@blueprintjs/core";
import { DockLocation, Orientation } from "flexlayout-react";

import { GlobalContext } from "../global/context";
import { LayoutModelContext, LayoutModel } from "../layout/context";
import * as consts from "../layout/consts";

interface Props {}

interface InputValue {
  key: string;
  value: string;
  valueNext: string | null;
  onChanged: (value: string) => void;
}

const StateView: React.FC<Props> = () => {
  const [focused, setFocused] = useState<InputValue | null>(null);
  const globalModel = useContext(GlobalContext);
  const layoutModel = useContext(LayoutModelContext);

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
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <HTMLSelect
          value={globalModel.base}
          onChange={(e) => {
            if (
              e.target.value !== "BIN" &&
              e.target.value !== "DEC" &&
              e.target.value !== "HEX"
            )
              throw new Error("invalid value");
            globalModel.setBase(e.target.value);
          }}
          minimal
        >
          <option value="BIN">BIN</option>
          <option value="DEC">DEC</option>
          <option value="HEX">HEX</option>
        </HTMLSelect>
        <Text>Cycle count: {globalModel.cycleCount()}</Text>
      </div>

      <div style={{ height: 16 }} />
      <H5>Registers</H5>

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Identifier</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {
            // TODO: Only get names one time!!! (on simulator create)
            globalModel.registers().map((register) => (
              <tr key={register}>
                <td>{register}</td>
                <td>
                  {inputValue({
                    key: register,
                    value: globalModel.registerValue(
                      register,
                      globalModel.base
                    ),
                    valueNext: globalModel.registerValueNext(
                      register,
                      globalModel.base
                    ),
                    onChanged: (value) =>
                      globalModel.writeIntoRegister(
                        register,
                        value,
                        globalModel.base
                      ),
                  })}
                </td>
              </tr>
            ))
          }
        </tbody>
      </HTMLTable>

      <div style={{ height: 16 }} />
      <H5>Buses</H5>

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Identifier</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {
            // TODO: Only get names one time!!! (on simulator create)
            globalModel.buses().map((bus) => (
              <tr key={bus}>
                <td>{bus}</td>
                <td>
                  {inputValue({
                    key: bus,
                    value: globalModel.busValue(bus, globalModel.base),
                    valueNext: null,
                    onChanged: (value) =>
                      globalModel.writeIntoBus(bus, value, globalModel.base),
                  })}
                </td>
              </tr>
            ))
          }
        </tbody>
      </HTMLTable>

      <div style={{ height: 16 }} />
      <H5>Memories</H5>

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Identifier</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {
            // TODO: Only get names one time!!! (on simulator create)
            globalModel.memories().map((memory) => (
              <tr key={memory}>
                <td>{memory}</td>
                <td>
                  <Button
                    small
                    onClick={() => {
                      // Select if exists
                      const memoryStateId = consts.ID_TAB_STATE_MEMORY(memory);
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
          }
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
