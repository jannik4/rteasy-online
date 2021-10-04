import {
  Position,
  OverlayToaster,
  ToastProps,
  Intent,
} from "@blueprintjs/core";

const AppToaster = OverlayToaster.create({
  position: Position.TOP,
});

export function showErrorToast(props: ToastProps, key?: string): string {
  return AppToaster.show(
    { intent: Intent.DANGER, timeout: 2000, ...props },
    key
  );
}
