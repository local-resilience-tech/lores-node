import { Stack, Title, Text, Group, ActionIcon, Card } from "@mantine/core"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"
import { IconPlus } from "@tabler/icons-react"

export default function NodeStewardsList() {
  const navigate = useNavigate()
  const [nodeStewards, setNodeStewards] = useState([])

  const listNodeStewards = async () => {
    getApi()
      .adminApi.listNodeStewards()
      .then((response) => {
        console.log("Node Stewards:", response.data)
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

      {nodeStewards.length > 0 && <Text>stewards list</Text>}
    </Stack>
  )
}
