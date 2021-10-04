import React, { useState } from "react";
import { Classes, Dialog, Button, InputGroup, Text } from "@blueprintjs/core";

interface Props {
  isOpen: boolean;
  onClose: () => void;
  onGoto: (address: string) => void;
}

const GotoDialog: React.FC<Props> = ({ isOpen, onClose, onGoto }) => {
  const [address, setAddress] = useState("");

  return (
    <Dialog
      title="Goto"
      onClose={onClose}
      isOpen={isOpen}
      usePortal={false}
      style={{ width: 250 }}
      enforceFocus={false}
    >
      <div className={Classes.DIALOG_BODY}>
        <div>
          <Text>Address (hex):</Text>
          <div style={{ height: 4 }} />
          <InputGroup
            small
            value={address}
            onChange={(e) => setAddress(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") onGoto(address);
            }}
          />
          <div style={{ height: 8 }} />
          <Button small onClick={() => onGoto(address)}>
            Goto
          </Button>
        </div>
      </div>
    </Dialog>
  );
};

export default GotoDialog;
