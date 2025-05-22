import { Heading, VStack } from "@chakra-ui/react"
import { NodeDetails } from "../types"
import EditNodeForm from "../components/EditNodeForm"

export default function EditNode({ node }: { node: NodeDetails }) {
  return (
    <VStack gap={4} align="stretch">
      <Heading as="h1" size="2xl">
        Edit This Node
      </Heading>
      <EditNodeForm
        node={node}
        onSubmit={(data: NodeDetails) => {
          console.log("Submitting node data", data)
        }}
      />
    </VStack>
  )
}
