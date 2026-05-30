import { Tooltip, Text } from "@mantine/core"

interface Props {
  id: string
}

export default function TruncatedId({ id }: Props) {
  return (
    <Tooltip label={id} withArrow>
      <Text span ff="monospace">
        {id.slice(0, 8)}
      </Text>
    </Tooltip>
  )
}
