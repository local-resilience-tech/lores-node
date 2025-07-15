import { Heading, VStack } from "@chakra-ui/react"
import EditNodeForm from "../components/EditNodeForm"
import ManageStatus from "../components/ManageStatus"
import type { Node, UpdateNodeDetails } from "../../../api/Api"
import { getApi } from "../../../api"

export default function EditNode({ node }: { node: Node }) {
  const updateNode = async (data: UpdateNodeDetails) => {
    getApi()
      .api.updateThisNode(data)
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
