import { Stack, Title, Text } from "@mantine/core"
import { getApi } from "../../../../api"
import { NodeStewardSetPasswordRequest } from "../../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
  Anchor,
} from "../../../../components"
import NodeStewardSetPasswordForm from "../components/NodeStewardSetPasswordForm"
import { useState } from "react"

export default function NodeStewardSetPassword() {
  const [success, setSuccess] = useState(false)

  const onSubmit = async (
    values: NodeStewardSetPasswordRequest
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .authApi.nodeStewardSetPassword(values)
      .then((response) => {
        setSuccess(true)
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  return (
    <Stack gap="lg">
      <Stack gap={0}>
        <Text c="dimmed" style={{ fontSize: "1.5rem" }} fw="bold" mb={-5}>
          Lores Node
        </Text>
        <Title order={1}>Set your password</Title>
      </Stack>

      {success && (
        <Stack gap="md">
          <Text>Password set successfully.</Text>
          <Text>
            You can now go ahead and <Anchor href="../login">log in</Anchor>.
          </Text>
        </Stack>
      )}

      {!success && (
        <Stack gap="lg">
          <Stack gap="md">
            <Text>
              You should have been given your id and one-use token by the node
              admin.
            </Text>
          </Stack>
          <NodeStewardSetPasswordForm onSubmit={onSubmit} />
        </Stack>
      )}
    </Stack>
  )
}
