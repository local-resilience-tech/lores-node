import { Stack, Table, Text } from "@mantine/core"
import { NodeSteward } from "../../../api/Api"
import { CopyIconButton } from "../../../components"

interface DisplayOneTimeTokenProps {
  steward: NodeSteward
  password_reset_token: string
  maw?: number
}

export default function DisplayOneTimeToken({
  steward,
  password_reset_token,
  maw,
}: DisplayOneTimeTokenProps) {
  return (
    <Stack gap="md" maw={maw}>
      <Text>
        Please give the following details to {steward.name} to allow them to
        complete the setup of their account.
      </Text>

      <Table>
        <Table.Tbody>
          <Table.Tr>
            <Table.Th maw={100}>Node Steward ID</Table.Th>
            <Table.Td ff="monospace">{steward.id}</Table.Td>
          </Table.Tr>
          <Table.Tr>
            <Table.Th maw={100}>Temporary Access Code</Table.Th>
            <Table.Td ff="monospace">{password_reset_token}</Table.Td>
          </Table.Tr>
        </Table.Tbody>
      </Table>

      <CopyIconButton
        value={`Node Steward ID: ${steward.id}\nTemporary Access Code: ${password_reset_token}`}
        prompt="Copy details to clipboard"
        successText="copied"
      />

      <Text>
        The temporary access code will be valid for one use within the next 24
        hours.
      </Text>
    </Stack>
  )
}
