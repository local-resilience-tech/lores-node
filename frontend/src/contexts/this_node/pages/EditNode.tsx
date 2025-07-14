import { Heading, VStack } from "@chakra-ui/react"
import EditNodeForm from "../components/EditNodeForm"
import ThisNodeApi, { UpdateNodeData } from "../api"
import ManageStatus from "../components/ManageStatus"
import type { Node } from "../../../api/Api"

const api = new ThisNodeApi()

export default function EditNode({ node }: { node: Node }) {
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
      <Heading as="h1" size="2xl">
        Node Status
      </Heading>
      <ManageStatus />
    </VStack>
  )
}
