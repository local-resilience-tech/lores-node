import { Stack, Title, Text } from "@mantine/core"
import { getApi } from "../../../../api"
import { useNavigate } from "react-router-dom"
import {
  NodeStewardCredentials,
  NodeStewardLoginError,
} from "../../../../api/Api"
import { AxiosError } from "axios"
import { actionFailure, ActionPromiseResult } from "../../../../components"
import NodeStewardSetPasswordForm from "../components/NodeStewardSetPasswordForm"

export default function NodeStewardSetPassword() {
  const navigate = useNavigate()

  const onSubmit = async (
    values: NodeStewardCredentials
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .authApi.nodeStewardLogin(values)
      .then((response) => {
        console.log("response", response)
        navigate("/admin/node_stewards")
      })
      .catch((error: AxiosError<NodeStewardLoginError>) => {
        return actionFailure(error)
      })
  }

  return (
    <Stack gap="lg">
      <Stack gap={0}>
        <Text c="dimmed" style={{ fontSize: "1.5rem" }} fw="bold" mb={-5}>
          Lores Node
        </Text>
        <Title order={1}>Set your password</Title>
      </Stack>
      <Stack gap="md">
        <Text>
          You should have been given your id and one-use token by the node
          admin.
        </Text>
      </Stack>
      <NodeStewardSetPasswordForm onSubmit={onSubmit} />
    </Stack>
  )
}
