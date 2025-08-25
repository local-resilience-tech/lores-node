import { Stack, Title, Text } from "@mantine/core"
import NodeStewardForm from "../components/NodeStewardForm"
import { NodeSteward } from "../../../api/Api"

export default function NewNodeSteward() {
  const handleSubmit = async (values: NodeSteward) => {
    // Handle form submission
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
