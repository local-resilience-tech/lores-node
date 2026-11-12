import { ActionIcon, Group, Stack, Title, Text } from "@mantine/core"
import { IconPlus } from "@tabler/icons-react"
import { useNavigate } from "react-router-dom"
import { useAppSelector } from "../../../store"
import RegisteredDomainProvidersList from "../components/RegisteredDomainProvidersList"

export default function ManageDomainProviders() {
  const navigate = useNavigate()

  const registeredDomainProviders = useAppSelector((state) => state.registeredDomains)
  return (
    <Stack gap="lg">
      <Group justify="space-between">
        <Title order={1}>Domain providers</Title>
        <ActionIcon size="lg" onClick={() => navigate("./new")}>
          <IconPlus />
        </ActionIcon>
      </Group>
      {registeredDomainProviders && registeredDomainProviders.length > 0 ? (
        <RegisteredDomainProvidersList details={registeredDomainProviders} />
      ) : (
        <Text>No providers currently configured</Text>
      )}
    </Stack>
  )
}
