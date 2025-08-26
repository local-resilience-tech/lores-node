import { ActionIcon, CopyButton, Group, Text } from "@mantine/core"
import { IconClipboard, IconClipboardCheck } from "@tabler/icons-react"

interface CopyIconButtonProps {
  value: string
  prompt?: string
  successText?: string
}

export default function CopyIconButton({
  value,
  prompt,
  successText,
}: CopyIconButtonProps) {
  return (
    <CopyButton value={value}>
      {({ copied, copy }) => (
        <Group gap="sm">
          <ActionIcon onClick={copy} color="gray" size="lg">
            {copied ? <IconClipboardCheck /> : <IconClipboard />}
          </ActionIcon>
          {prompt && !copied && <Text c="dimmed">{prompt}</Text>}
          {successText && copied && <Text c="dimmed">{successText}</Text>}
        </Group>
      )}
    </CopyButton>
  )
}
