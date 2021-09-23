import React, { useRef } from "react";

export function useLazyRef<T>(
  initialValue: () => T
): React.MutableRefObject<T> {
  const ref = useRef<T | null>(null);
  const initialized = useRef(false);

  // Initialize once
  if (!initialized.current) {
    ref.current = initialValue();
    initialized.current = true;
  }

  // Ref must be initialized
  return ref as React.MutableRefObject<T>;
}
