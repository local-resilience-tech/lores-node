import { VStack } from "@chakra-ui/react"
import PostStatus from "./PostStatus"
import ThisNodeApi, { PostStatusData } from "../api"

const api = new ThisNodeApi()

export default function ManageStatus() {
  const postStatus = async (data: PostStatusData) => {
    const result = await api.postStatus(data)
    if ("Ok" in result) {
      console.log("Node updated successfully", result.Ok)
    }
    if ("Err" in result) {
      console.error("Error updating node", result.Err)
    }
  }

  return (
    <VStack gap={4} align="stretch">
      <PostStatus onSubmit={postStatus} />
      <p>Display current status here</p>
    </VStack>
  )
}
