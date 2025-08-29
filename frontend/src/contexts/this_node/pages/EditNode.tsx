import { Breadcrumbs, Stack, Title, Text } from "@mantine/core"
import EditNodeForm from "../components/EditNodeForm"
import type { UpdateNodeDetails } from "../../../api/Api"
import { getApi } from "../../../api"
import { useAppSelector } from "../../../store"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
  Anchor,
} from "../../../components"

export default function ThisNode() {
  const node = useAppSelector((state) => state.thisNode)

  if (!node) return null

  const updateNode = async (
    data: UpdateNodeDetails
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.updateThisNode(data)
      .then(actionSuccess)
      .catch(actionFailure)
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
