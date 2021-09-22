import React, { useContext } from "react";
import { Classes, Slider, Dialog } from "@blueprintjs/core";

import { GlobalContext, clockRateValues } from "../global/context";

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

const OptionsDialog: React.FC<Props> = ({ isOpen, onClose }) => {
  const globalModel = useContext(GlobalContext);

  return (
    <Dialog title="Options" onClose={onClose} isOpen={isOpen}>
      <div className={Classes.DIALOG_BODY}>
        <div style={{ height: 16 }} />
        <div style={{ display: "flex" }}>
          <strong>Clock rate</strong>
          <Slider
            min={0}
            max={clockRateValues.length - 1}
            onChange={(idx) => globalModel.setClockRate(clockRateValues[idx])}
            labelRenderer={(idx) => clockRateValues[idx].toString()}
            showTrackFill={false}
            value={clockRateValues.indexOf(globalModel.clockRate)}
          />
        </div>
      </div>
    </Dialog>
  );
};

export default OptionsDialog;
