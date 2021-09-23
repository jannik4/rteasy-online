import React, { useContext, useState } from "react";
import { Classes, Slider, Dialog, Button } from "@blueprintjs/core";

import { GlobalContext, clockRateValues } from "../global/context";
import { Storage } from "../storage";

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

const OptionsDialog: React.FC<Props> = ({ isOpen, onClose }) => {
  const globalModel = useContext(GlobalContext);
  const [reloadRequired, setReloadRequired] = useState(false);

  return (
    <Dialog title="Options" onClose={onClose} isOpen={isOpen}>
      <div className={Classes.DIALOG_BODY}>
        <div style={{ marginBottom: 16 }}>
          <strong>Clock rate</strong>
        </div>
        <div style={{ maxWidth: 300 }}>
          <Slider
            min={0}
            max={clockRateValues.length - 1}
            onChange={(idx) => globalModel.setClockRate(clockRateValues[idx])}
            labelRenderer={(idx) => {
              const value = clockRateValues[idx];
              if (value === "Max") return value;
              return value + "Hz";
            }}
            showTrackFill={false}
            value={clockRateValues.indexOf(globalModel.clockRate)}
          />
        </div>
        <div style={{ margin: "16px 0" }}>
          <strong>Layout</strong>
        </div>
        <Button
          small
          onClick={() => {
            Storage.removeAllLayoutModels();
            setReloadRequired(true);
          }}
        >
          Reset Layout
        </Button>
        {reloadRequired ? (
          <span
            style={{
              marginLeft: 8,
              textDecorationLine: "underline",
              cursor: "pointer",
            }}
            onClick={() => window.location.reload()}
          >
            (reload required)
          </span>
        ) : null}
      </div>
    </Dialog>
  );
};

export default OptionsDialog;
