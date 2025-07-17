import { Divider, Stack, Title, Box } from "@mantine/core"
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
    <Stack gap="lg">
      <Stack>
        <Title order={1}>Edit This Node</Title>
        <EditNodeForm node={node} onSubmit={updateNode} />
      </Stack>
      <Divider />
      <Stack>
        <Title order={2}>Node Status</Title>
        <ManageStatus />
      </Stack>
    </Stack>
  )
}
