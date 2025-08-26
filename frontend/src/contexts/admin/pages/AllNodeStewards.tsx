import { Stack, Title, Text, Group, ActionIcon, Card } from "@mantine/core"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"
import { IconPlus } from "@tabler/icons-react"
import { NodeSteward, NodeStewardStatus } from "../../../api/Api"
import NodeStewardsList from "../components/NodeStewardsList"
import { NodeStewardAction } from "../components/NodeStewardAction"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"
import DisplayOneTimeToken from "../components/DisplayOneTimeToken"

export default function AllNodeStewards() {
  const navigate = useNavigate()
  const [nodeStewards, setNodeStewards] = useState<NodeSteward[]>([])
  const [stewardTokens, setStewardTokens] = useState<Record<string, string>>({})

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

  const updateLocalNodeSteward = (record: NodeSteward) => {
    setNodeStewards((prev) =>
      prev.map((steward) => (steward.id === record.id ? record : steward))
    )
  }

  const updateStewardToken = (id: string, token: string) => {
    setStewardTokens((prev) => ({
      ...prev,
      [id]: token,
    }))
  }

  useEffect(() => {
    listNodeStewards()
  }, [])

  const getActions = (record: NodeSteward): NodeStewardAction[] => {
    let result: NodeStewardAction[] = []

    if (record.status === NodeStewardStatus.TokenExpired) {
      result.push({
        type: "reset_token",
        buttonColor: "orange",
        primary: true,
        handler: (record: NodeSteward): Promise<ActionPromiseResult> => {
          return getApi()
            .adminApi.resetNodeStewardToken(record.id)
            .then((result) => {
              updateLocalNodeSteward(result.data.node_steward)
              updateStewardToken(record.id, result.data.password_reset_token)
              return actionSuccess()
            })
            .catch((error) => {
              if (
                error.response?.status === 401 ||
                error.response?.status === 403
              ) {
                navigate("/auth/admin/login")
              } else {
                return actionFailure(error)
              }
            })
        },
      })
    }

    if (stewardTokens[record.id]) {
      result.push({
        type: "display_token",
        buttonColor: "blue",
        primary: false,
        overlay: (
          <DisplayOneTimeToken
            steward={record}
            password_reset_token={stewardTokens[record.id]}
            maw={400}
          />
        ),
      })
    }

    return result
  }

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
        <NodeStewardsList nodeStewards={nodeStewards} getActions={getActions} />
      )}
    </Stack>
  )
}
