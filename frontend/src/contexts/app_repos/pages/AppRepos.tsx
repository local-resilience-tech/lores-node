import { ActionIcon, Container, Group, Stack, Title } from "@mantine/core"
import { IconPlus } from "@tabler/icons-react"
import { useNavigate } from "react-router-dom"
import AppRepoList from "../components/AppRepoList"

export default function LocalApps() {
  const navigate = useNavigate()

  return (
    <Container>
      <Stack>
        <Group justify="space-between">
          <Title order={1}>App Repositories</Title>
          <ActionIcon size="lg" onClick={() => navigate("./new")}>
            <IconPlus />
          </ActionIcon>
        </Group>
        <AppRepoList />
      </Stack>
    </Container>
  )
}
