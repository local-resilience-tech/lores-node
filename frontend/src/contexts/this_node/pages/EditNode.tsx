import { Breadcrumbs, Stack, Title, Text } from "@mantine/core"
import EditNodeForm from "../components/EditNodeForm"
import type { UpdateNodeDetails } from "../../../api/Api"
import { getApi } from "../../../api"
import { useAppSelector } from "../../../store"
import { Anchor } from "../../../components"

export default function ThisNode() {
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
      <Stack gap="xs">
        <Breadcrumbs>
          <Anchor href="/this_node">{node.name}</Anchor>
          <Text c="dimmed">edit</Text>
        </Breadcrumbs>
        <Title order={1}>Edit this node</Title>
      </Stack>
      <EditNodeForm node={node} onSubmit={updateNode} />
    </Stack>
  )
}
