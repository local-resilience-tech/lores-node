import { VStack } from "@chakra-ui/react"
import PostStatus from "./PostStatus"

export default function ManageStatus() {
  return (
    <VStack gap={4} align="stretch">
      <PostStatus />
      <p>Display current status here</p>
    </VStack>
  )
}
