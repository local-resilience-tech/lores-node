import { ActionIcon, Group, Stack, Title, Text } from "@mantine/core"
import { IconPlus } from "@tabler/icons-react"
import { useNavigate } from "react-router-dom"

export default function ManageDomainProviders() {
  const navigate = useNavigate()

  return (
    <Stack gap="lg">
      <Group justify="space-between">
        <Title order={1}>Domain providers</Title>
        <ActionIcon size="lg" onClick={() => navigate("./new")}>
          <IconPlus />
        </ActionIcon>
      </Group>
      {/*to do - display the existing providers here*/}
      {/*to do - display when there's no existing providers */}
      <Text>No providers currently configured</Text>
    </Stack>
  )
}
