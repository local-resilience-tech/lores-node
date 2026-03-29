import { Stack, Card, Text, Group, Badge, ThemeIcon } from "@mantine/core"
import {
  IconAlertCircle,
  IconCircleCheck,
  IconClock,
  IconHelpCircle,
} from "@tabler/icons-react"
import { Anchor } from "../../../components"
import { RegionNodeDetails } from "../../../api/Api"
import { nodeName } from "../../../store/my_regions"

const IpLink = ({ ip }: { ip: string | undefined | null }) => {
  if (!ip) return <Text c="dimmed">unknown</Text>

  return (
    <Anchor href={`https://${ip}`} newWindow>
      {ip}
    </Anchor>
  )
}

interface NodeCardProps {
  node: RegionNodeDetails
  isRegionCreator?: boolean
}

function stateMeta(state?: string | null): {
  label: string
  color: "red" | "yellow" | "green" | "gray"
  Icon: typeof IconHelpCircle
} {
  const label = state?.trim() || "unknown"
  const normalized = label.toLowerCase()

  if (
    normalized.includes("error") ||
    normalized.includes("fail") ||
    normalized.includes("offline") ||
    normalized.includes("down") ||
    normalized.includes("reject")
  ) {
    return { label, color: "red", Icon: IconAlertCircle }
  }

  if (
    normalized.includes("pending") ||
    normalized.includes("join") ||
    normalized.includes("starting") ||
    normalized.includes("sync") ||
    normalized.includes("wait")
  ) {
    return { label, color: "yellow", Icon: IconClock }
  }

  if (
    normalized.includes("online") ||
    normalized.includes("active") ||
    normalized.includes("ready") ||
    normalized.includes("ok") ||
    normalized.includes("healthy") ||
    normalized.includes("connected")
  ) {
    return { label, color: "green", Icon: IconCircleCheck }
  }

  return { label, color: "gray", Icon: IconHelpCircle }
}

export default function NodeCard({ node, isRegionCreator }: NodeCardProps) {
  const { label: stateLabel, color: stateColor, Icon } = stateMeta(node.state)
  const message = node.status_text?.trim()

  return (
    <Card key={node.id} withBorder>
      <Stack>
        <Group justify="space-between">
          <Stack gap={0}>
            <Text fw={500} size="lg">
              {nodeName(node)}
            </Text>
            <Text size="sm" ff="monospace">
              {node.node_id}
            </Text>
          </Stack>
          {isRegionCreator && <Badge>Admin</Badge>}
        </Group>
        <Card.Section bg="dark.7" px="md" py="sm">
          <Group gap="xs" wrap="nowrap">
            <ThemeIcon variant="light" color={stateColor} size="sm" radius="xl">
              <Icon size={14} />
            </ThemeIcon>
            <Text size="sm">
              <Text span fw={500}>
                {stateLabel}
              </Text>
              {message ? (
                <Text span c="dimmed">
                  : {message}
                </Text>
              ) : null}
            </Text>
          </Group>
        </Card.Section>
      </Stack>
    </Card>
  )
}
