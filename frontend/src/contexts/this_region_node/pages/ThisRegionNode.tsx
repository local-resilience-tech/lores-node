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
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { myActiveRegionNode } from "../../../store/my_regions"

export default function ThisRegionNode() {
  const navigate = useNavigate()
  const node = useAppSelector((state) =>
    myActiveRegionNode(state.my_regions, state.network?.node.id),
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
        <IfNodeSteward>
          <ActionIcon size="lg" onClick={() => navigate("./edit")}>
            <IconEdit />
          </ActionIcon>
        </IfNodeSteward>
      </Group>

      <Stack gap="xs">
        <Title order={2}>Details</Title>
        <Card>
          <Card.Section>
            <ThisNodeDetails node={node} nodeDetails={node} />
          </Card.Section>
        </Card>
      </Stack>

      <Stack align="flex-start">
        <Title order={2}>Node Status</Title>
        <Card>
          <Card.Section>
            <DisplayStatus
              state={node?.state}
              status_text={node?.status_text}
            />
          </Card.Section>
        </Card>
        <IfNodeSteward>
          <Button variant="outline" onClick={() => navigate("./status")}>
            Update status
          </Button>
        </IfNodeSteward>
      </Stack>
    </Stack>
  )
}
