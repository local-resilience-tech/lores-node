import { Stack, Title, Text, Group, ActionIcon, Card } from "@mantine/core"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"
import { IconPlus } from "@tabler/icons-react"
import { NodeSteward } from "../../../api/Api"
import NodeStewardsList from "../components/NodeStewardsList"

export default function AllNodeStewards() {
  const navigate = useNavigate()
  const [nodeStewards, setNodeStewards] = useState<NodeSteward[]>([])

  const listNodeStewards = async () => {
    getApi()
      .adminApi.listNodeStewards()
      .then((response) => {
        setNodeStewards(response.data)
      })
      .catch((error) => {
        if (error.response?.status === 401 || error.response?.status === 403) {
          navigate("/auth/admin/login")
        } else {
          console.error("Error fetching node stewards:", error)
        }
      })
  }

  useEffect(() => {
    listNodeStewards()
  }, [])

  return (
    <Stack>
      <Group justify="space-between">
        <Title>Node Stewards</Title>
        <ActionIcon size="lg" onClick={() => navigate("./new")}>
          <IconPlus />
        </ActionIcon>
      </Group>

      {nodeStewards.length === 0 && (
        <Card maw={500}>
          <Stack gap="md">
            <Text fw="500" size="lg">
              No node stewards found
            </Text>
            <Text c="dimmed">
              Node stewards are users who administer and manage this node. You
              can't do anything with this admin account other than manage node
              stewards, so you should probably create a node steward account for
              yourself using the "
              <Text span fw="bold" c="blue">
                +
              </Text>
              " button above.
            </Text>
          </Stack>
        </Card>
      )}

      {nodeStewards.length > 0 && (
        <NodeStewardsList nodeStewards={nodeStewards} />
      )}
    </Stack>
  )
}
