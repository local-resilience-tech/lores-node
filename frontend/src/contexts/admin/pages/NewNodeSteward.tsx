import {
  Stack,
  Title,
  Text,
  Card,
  Table,
  Group,
  CopyButton,
} from "@mantine/core"
import NodeStewardForm from "../components/NodeStewardForm"
import {
  NodeStewardCreationData,
  NodeStewardCreationResult,
} from "../../../api/Api"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
  Anchor,
  CopyIconButton,
} from "../../../components"
import { useState } from "react"

export default function NewNodeSteward() {
  const navigate = useNavigate()
  const [result, setResult] = useState<NodeStewardCreationResult | null>(null)

  const handleSubmit = async (
    values: NodeStewardCreationData
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .adminApi.createNodeSteward(values)
      .then((response) => {
        setResult(response.data)
        return actionSuccess()
      })
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

      {result && (
        <Card maw={600}>
          <Stack gap="md">
            <Text fw={500} size="xl">
              Node steward <strong>{result.node_steward.name}</strong> created
              successfully
            </Text>
            <Text>
              Please give the following details to {result.node_steward.name} to
              allow them to complete the setup of their account.
            </Text>

            <Table>
              <Table.Tbody>
                <Table.Tr>
                  <Table.Th maw={100}>Node Steward ID</Table.Th>
                  <Table.Td ff="monospace">{result.node_steward.id}</Table.Td>
                </Table.Tr>
                <Table.Tr>
                  <Table.Th maw={100}>Temporary Access Code</Table.Th>
                  <Table.Td ff="monospace">
                    {result.password_reset_token}
                  </Table.Td>
                </Table.Tr>
              </Table.Tbody>
            </Table>

            <CopyIconButton
              value={`Node Steward ID: ${result.node_steward.id}\nTemporary Access Code: ${result.password_reset_token}`}
              prompt="Copy details to clipboard"
              successText="copied"
            />

            <Text>
              We only display these details this once. The temporary access code
              will be valid for one use within the next 24 hours.
            </Text>

            <Anchor href="../">Back to list</Anchor>
          </Stack>
        </Card>
      )}

      {!result && (
        <>
          <Stack gap="md" maw={600}>
            <Text>
              We identify node stewards with a unique ID, rather than an email
              address, in case email verification is not possible.
            </Text>
            <Text>
              We'll create that ID for you, and display a temporary access code
              that the you can give to the node steward to log in for the first
              time and set their password.
            </Text>
          </Stack>
          <NodeStewardForm onSubmit={handleSubmit} />
        </>
      )}
    </Stack>
  )
}
