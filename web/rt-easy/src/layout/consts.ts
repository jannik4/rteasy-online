export const ID_TAB_STATE = "TAB_STATE";

export const MARKER_STATE_EXTRA = "{{MARKER_STATE_EXTRA}}";
export const MARKER_TEMP = "{{MARKER_TEMP}}";

export function ID_TAB_STATE_REGISTER_ARRAY(name: string): string {
  return MARKER_TEMP + MARKER_STATE_EXTRA + "TAB_STATE_REGISTER_ARRAY_" + name;
}

export function ID_TAB_STATE_MEMORY(name: string): string {
  return MARKER_TEMP + MARKER_STATE_EXTRA + "TAB_STATE_MEMORY_" + name;
}
