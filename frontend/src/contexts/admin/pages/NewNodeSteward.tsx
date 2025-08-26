import { Stack, Title, Text } from "@mantine/core"
import NodeStewardForm from "../components/NodeStewardForm"
import { NodeStewardCreationData } from "../../../api/Api"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"

export default function NewNodeSteward() {
  const navigate = useNavigate()

  const handleSubmit = async (
    values: NodeStewardCreationData
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .adminApi.createNodeSteward(values)
      .then(actionSuccess)
      .catch((error) => {
        if (error.response?.status === 401 || error.response?.status === 403) {
          navigate("/auth/admin/login")
        } else {
          return actionFailure(error)
        }
      })
  }

  return (
    <Stack gap="lg">
      <Title>New node steward</Title>
      <Stack gap="md" maw={600}>
        <Text>
          We identify node stewards with a unique ID, rather than an email
          address, in case email verification is not possible.
        </Text>
        <Text>
          We'll create that ID for you, and display a temporary access code that
          the you can give to the node steward to log in for the first time and
          set their password.
        </Text>
      </Stack>
      <NodeStewardForm onSubmit={handleSubmit} />
    </Stack>
  )
}
