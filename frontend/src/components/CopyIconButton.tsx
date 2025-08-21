import { ActionIcon, CopyButton } from "@mantine/core"
import { IconClipboard, IconClipboardCheck } from "@tabler/icons-react"

export default function CopyIconButton({ value }: { value: string }) {
  return (
    <CopyButton value={value}>
      {({ copied, copy }) => (
        <ActionIcon onClick={copy} color="gray" size="lg">
          {copied ? <IconClipboardCheck /> : <IconClipboard />}
        </ActionIcon>
      )}
    </CopyButton>
  )
}
