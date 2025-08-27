import { Stack, Title, Text } from "@mantine/core"
import NodeStewardLoginForm from "../components/NodeStewardLoginForm"
import { getApi } from "../../../../api"
import { useNavigate } from "react-router-dom"
import {
  NodeStewardCredentials,
  NodeStewardLoginError,
} from "../../../../api/Api"
import { AxiosError } from "axios"
import {
  actionFailure,
  ActionPromiseResult,
  Anchor,
} from "../../../../components"

export default function NodeStewardLogin() {
  const navigate = useNavigate()

  const onSubmit = async (
    values: NodeStewardCredentials
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .authApi.nodeStewardLogin(values)
      .then((response) => {
        console.log("response", response)
        navigate("/")
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
        <Title order={1}>Log in as node steward</Title>
      </Stack>
      <Stack gap="md">
        <Text>
          Node stewards are users who manage this node. If you are a new node
          steward, you will have been given a one-use token{" "}
          <Anchor href="../set_password">to set a password</Anchor>.
        </Text>
      </Stack>
      <NodeStewardLoginForm onSubmit={onSubmit} />
    </Stack>
  )
}
