import { Stack, Text } from "@mantine/core"
import PostStatus from "./PostStatus"
import { getApi } from "../../../api"
import type { NodeStatusData } from "../../../api/Api"

export default function ManageStatus() {
  const postStatus = async (data: NodeStatusData) => {
    getApi()
      .api.postNodeStatus(data)
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
    <Stack>
      <PostStatus onSubmit={postStatus} />
      <Text c="dimmed">Display current status here</Text>
    </Stack>
  )
}
