import { VStack } from "@chakra-ui/react"
import PostStatusMessage from "./PostStatusMessage"

export default function ManageStatusMessage() {
  return (
    <VStack gap={4} align="stretch">
      <PostStatusMessage />
      <p>Display current status here</p>
    </VStack>
  )
}
