import { Stack, Title, Text } from "@mantine/core"
import {
  ActionPromiseResult,
  actionSuccess,
  actionFailure,
  ActionButton,
} from "../../../components"
import { getApi } from "../../../api"

export default function SetupAdmin() {
  const generatePassword = async (): Promise<ActionPromiseResult> => {
    return getApi()
      .auth.generateAdminPassword()
      .then((result) => {
        return actionSuccess()
      })
      .catch((error) => {
        return actionFailure(error)
      })
  }

  return (
    <Stack gap="lg">
      <Stack gap={0}>
        <Text c="dimmed" style={{ fontSize: "1.5rem" }} fw="bold" mb={-5}>
          Lores Node
        </Text>
        <Title order={1}>Setup your admin password</Title>
      </Stack>
      <Text>
        The admin password is only used to create the users you use to steward
        this node. It can be reset at any time.
      </Text>
      <Text>
        The password is auto-generated and only displayed to you this once. If
        you're ready to store it in a safe place (an encrypted password manager,
        for example), click the button below to continue.
      </Text>
      <ActionButton onClick={generatePassword} expand>
        Generate admin password
      </ActionButton>
    </Stack>
  )
}
