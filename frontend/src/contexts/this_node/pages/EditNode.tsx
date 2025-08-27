import { Stack, Title } from "@mantine/core"
import EditNodeForm from "../components/EditNodeForm"
import type { UpdateNodeDetails } from "../../../api/Api"
import { getApi } from "../../../api"
import { useAppSelector } from "../../../store"
import { useNavigate } from "react-router-dom"

export default function ThisNode() {
  const navigate = useNavigate()
  const node = useAppSelector((state) => state.thisNode)

  if (!node) return null

  const updateNode = async (data: UpdateNodeDetails) => {
    getApi()
      .publicApi.updateThisNode(data)
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
      <Title order={1}>Edit this node</Title>
      <EditNodeForm node={node} onSubmit={updateNode} />
    </Stack>
  )
}
