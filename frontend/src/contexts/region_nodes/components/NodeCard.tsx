import { Stack, Card, Text, Group, Badge, ThemeIcon } from "@mantine/core"
import { type ReactNode } from "react"
import {
  IconAlertCircle,
  IconCircleCheck,
  IconCircleFilled,
  IconClock,
  IconHelpCircle,
} from "@tabler/icons-react"
import { Anchor } from "../../../components"
import { NodeState, RegionNodeDetails } from "../../../api/Api"
import { nodeName } from "../../../store/my_regions"
import { NodeHeartbeatDisplay } from "../../../hooks/useNodeHeartbeats"

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
  rightSection?: ReactNode
  isThisNode?: boolean
  nodeHeartbeatDisplay?: NodeHeartbeatDisplay
}

interface NodeStatusProps {
  state?: NodeState | null
  statusText?: string | null
  isThisNode?: boolean
  nodeHeartbeatDisplay?: NodeHeartbeatDisplay
}

function NodeStatus({
  state,
  statusText,
  isThisNode,
  nodeHeartbeatDisplay,
}: NodeStatusProps) {
  const message = statusText?.trim() || undefined

  if (!state && !message) return null

  const {
    label: stateLabel,
    color: stateColor,
    Icon,
  } = (() => {
    switch (state) {
      case NodeState.Active:
        return { label: "Active", color: "green", Icon: IconCircleCheck }
      case NodeState.Inactive:
        return { label: "Inactive", color: "red", Icon: IconAlertCircle }
      case NodeState.Maintenance:
        return { label: "Maintenance", color: "yellow", Icon: IconClock }
      case NodeState.Development:
        return { label: "Development", color: "gray", Icon: IconHelpCircle }
      default:
        return { label: "Unknown", color: "gray", Icon: IconHelpCircle }
    }
  })()

  return (
    <Group gap="sm" justify="space-between">
      <Group>
        <Group gap={1} wrap="nowrap">
          <ThemeIcon variant="light" color={stateColor} size="sm" radius="xl">
            <Icon size={22} />
          </ThemeIcon>

          <Text span fw={500} size="sm" c={stateColor}>
            {stateLabel}
          </Text>
        </Group>

        {message ? (
          <Text span size="sm">
            {message}
          </Text>
        ) : null}
      </Group>
      <Group gap={3}>
        {isThisNode ? (
          <Badge color="gray">This node</Badge>
        ) : nodeHeartbeatDisplay ? (
          <>
            <IconCircleFilled color={nodeHeartbeatDisplay.color} />
            <Text span size="sm">
              {nodeHeartbeatDisplay.label}
            </Text>
          </>
        ) : null}
      </Group>
    </Group>
  )
}

export default function NodeCard({
  node,
  isRegionCreator,
  rightSection,
  isThisNode,
  nodeHeartbeatDisplay,
}: NodeCardProps) {
  return (
    <Card key={node.id} withBorder>
      <Stack>
        <Group justify="space-between" wrap="nowrap" align="flex-start">
          <Stack gap={0} style={{ flex: 1, minWidth: 0 }}>
            <Text fw={500} size="lg">
              {nodeName(node)}
            </Text>
            <Text
              size="sm"
              ff="monospace"
              maw="90%"
              truncate="end"
              title={node.node_id}
            >
              {node.node_id}
            </Text>
          </Stack>
          {(isRegionCreator || rightSection) && (
            <Group gap="xs" wrap="nowrap" align="center">
              {isRegionCreator && <Badge>Admin</Badge>}
              {rightSection}
            </Group>
          )}
        </Group>

        <NodeStatus
          state={node.state}
          statusText={node.status_text}
          isThisNode={isThisNode}
          nodeHeartbeatDisplay={nodeHeartbeatDisplay}
        />
      </Stack>
    </Card>
  )
}
