import {
  Stack,
  Title,
  Text,
  Card,
  Group,
  ActionIcon,
  Button,
} from "@mantine/core"
import { useAppSelector } from "../../../store"
import ThisNodeDetails from "../components/ThisNodeDetails"
import { useNavigate } from "react-router-dom"
import { IconEdit } from "@tabler/icons-react"
import DisplayStatus from "../components/DisplayStatus"
import { getNodeById } from "../../../store/nodes"

export default function ThisNode() {
  const navigate = useNavigate()
  const node = useAppSelector((state) => state.thisNode)
  const nodeDetails = useAppSelector((state) =>
    getNodeById(state.nodes, node?.id)
  )

  if (!node) return null

  return (
    <Stack gap="lg">
      <Group justify="space-between">
        <Stack gap="xs">
          <Title order={1}>
            <Text span inherit c="dimmed">
              This Node:{" "}
            </Text>
            {node.name}
          </Title>
        </Stack>
        <ActionIcon size="lg" onClick={() => navigate("./edit")}>
          <IconEdit />
        </ActionIcon>
      </Group>

      <Stack gap="xs">
        <Title order={2}>Details</Title>
        <Card>
          <Card.Section>
            <ThisNodeDetails node={node} nodeDetails={nodeDetails} />
          </Card.Section>
        </Card>
      </Stack>

      <Stack align="flex-start">
        <Title order={2}>Node Status</Title>
        <Card>
          <Card.Section>
            <DisplayStatus
              state={nodeDetails?.state}
              status_text={nodeDetails?.status_text}
            />
          </Card.Section>
        </Card>
        <Button variant="outline" onClick={() => navigate("./status")}>
          Update status
        </Button>
      </Stack>
    </Stack>
  )
}
