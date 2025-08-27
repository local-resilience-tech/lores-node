import {
  Divider,
  Stack,
  Title,
  Text,
  Card,
  Group,
  ActionIcon,
} from "@mantine/core"
import EditNodeForm from "../components/EditNodeForm"
import ManageStatus from "../components/ManageStatus"
import type { UpdateNodeDetails } from "../../../api/Api"
import { getApi } from "../../../api"
import { useAppSelector } from "../../../store"
import ThisNodeDetails from "../components/ThisNodeDetails"
import { useNavigate } from "react-router-dom"
import { IconEdit } from "@tabler/icons-react"

export default function ThisNode() {
  const navigate = useNavigate()
  const node = useAppSelector((state) => state.thisNode)

  if (!node) return null

  const updateNode = async (data: UpdateNodeDetails) => {
    getApi()
      .publicApi.updateThisNode(data)
      .then((result) => {
        if (result.status === 200) {
          console.log("Node updated successfully", result.data)
        } else {
          console.error("Failed to create node", result)
        }
      })
      .catch((error) => {
        console.error("Error creating node", error)
      })
  }

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
            <ThisNodeDetails node={node} />
          </Card.Section>
        </Card>
      </Stack>

      <Stack>
        <Title order={2}>Node Status</Title>
        <ManageStatus />
      </Stack>
    </Stack>
  )
}
