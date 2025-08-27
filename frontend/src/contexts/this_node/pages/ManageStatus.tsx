import { Breadcrumbs, Stack, Text, Title } from "@mantine/core"
import PostStatus from "../components/PostStatus"
import { getApi } from "../../../api"
import type { NodeStatusData } from "../../../api/Api"
import { actionFailure, ActionPromiseResult, Anchor } from "../../../components"
import { useAppSelector } from "../../../store"

export default function ManageStatus() {
  const node = useAppSelector((state) => state.thisNode)

  if (!node) return null

  const postStatus = async (
    data: NodeStatusData
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.postNodeStatus(data)
      .then((result) => {
        if (result.status === 200) {
          console.log("Node updated successfully", result.data)
        } else {
          console.error("Failed to create node", result)
        }
      })
      .catch(actionFailure)
  }

  return (
    <Stack gap="lg">
      <Stack gap="xs">
        <Breadcrumbs>
          <Anchor href="/this_node">{node.name}</Anchor>
          <Text c="dimmed">status update</Text>
        </Breadcrumbs>
        <Title order={1}>Status update</Title>
      </Stack>
      <PostStatus onSubmit={postStatus} />
    </Stack>
  )
}
