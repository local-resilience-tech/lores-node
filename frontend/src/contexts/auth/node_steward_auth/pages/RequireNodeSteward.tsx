import { Stack, Title, Text } from "@mantine/core"
import { useAppSelector } from "../../../../store"
import { Anchor } from "../../../../components"

interface RequireNodeStewardProps {
  children: React.ReactNode
}

function NodeStewardMissing() {
  return (
    <Stack gap="lg">
      <Stack gap={0}>
        <Text size="xl" c="dimmed">
          403: Forbidden
        </Text>
        <Title order={1}>This page is for Node Stewards</Title>
      </Stack>
      <Text>
        You must be a Node Steward to view this page. If that's you, please{" "}
        <Anchor href="/auth/node_steward/login">log in</Anchor>.
      </Text>
    </Stack>
  )
}

export default function RequireNodeSteward({
  children,
}: RequireNodeStewardProps) {
  const me = useAppSelector((state) => state.me)
  if (!me) {
    return <NodeStewardMissing />
  }

  return <>{children}</>
}
