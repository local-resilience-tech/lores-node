import { Heading, VStack } from "@chakra-ui/react"
import { NodeDetails } from "../types"
import EditNodeForm from "../components/EditNodeForm"
import ThisNodeApi, { UpdateNodeData } from "../api"

const api = new ThisNodeApi()

export default function EditNode({ node }: { node: NodeDetails }) {
  const updateNode = async (data: UpdateNodeData) => {
    const result = await api.update(data)
    if ("Ok" in result) {
      console.log("Node updated successfully", result.Ok)
    }
    if ("Err" in result) {
      console.error("Error updating node", result.Err)
    }
  }

  return (
    <VStack gap={4} align="stretch">
      <Heading as="h1" size="2xl">
        Edit This Node
      </Heading>
      <EditNodeForm node={node} onSubmit={updateNode} />
    </VStack>
  )
}
