import { Text } from "@mantine/core"
import { modals } from "@mantine/modals"

export function awaitConfirmModal(
  title?: string,
  children?: React.ReactNode
): Promise<boolean> {
  return new Promise((resolve) => {
    modals.openConfirmModal({
      title: title ?? "Please confirm your action",
      children: children ?? (
        <Text size="sm">
          This action is so important that you are required to confirm it with a
          modal. Please click one of these buttons to proceed.
        </Text>
      ),
      labels: { confirm: "Confirm", cancel: "Cancel" },
      onCancel: () => resolve(false),
      onConfirm: () => resolve(true),
    })
  })
}
