import { Stack, Title, Text, Group } from "@mantine/core"
import {
  ActionPromiseResult,
  actionSuccess,
  actionFailure,
  ActionButton,
  CopyIconButton,
  Anchor,
} from "../../../components"
import { getApi } from "../../../api"
import { useState } from "react"

export default function SetupAdmin() {
  const [password, setPassword] = useState<string | null>(null)

  const generatePassword = async (): Promise<ActionPromiseResult> => {
    return getApi()
      .auth.generateAdminPassword()
      .then((result) => {
        setPassword(result.data || null)
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
      {!password && (
        <Stack gap="xl">
          <Stack gap="md">
            <Text>
              The admin password is only used to create the users you use to
              steward this node. It can be reset at any time.
            </Text>
            <Text>
              The password is auto-generated and only displayed to you this
              once. If you're ready to store it in a safe place (an encrypted
              password manager, for example), click the button below to
              continue.
            </Text>
          </Stack>
          <ActionButton onClick={generatePassword} expand>
            Generate admin password
          </ActionButton>
        </Stack>
      )}

      {password && (
        <Stack gap="md">
          <Text>Your new admin password is:</Text>
          <Group>
            <Text
              component="pre"
              style={{
                border: "1px solid var(--mantine-color-gray-6)",
                padding: "4px 10px",
                borderRadius: "4px",
              }}
            >
              {password}
            </Text>
            <CopyIconButton value={password} />
          </Group>
          <Text>
            Make sure to store this password in a safe place. When you've done
            that, you can go ahead and:
          </Text>
          <Text>
            <Anchor href="../login">Login as admin</Anchor>
          </Text>
        </Stack>
      )}
    </Stack>
  )
}
