import { ActionIcon, Container, Group, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import LocalAppsList from "../components/LocalAppsList"
import { LocalApp } from "../../../api/Api"
import { getApi } from "../../../api"
import { IconPlus } from "@tabler/icons-react"
import { useNavigate } from "react-router-dom"

export default function LocalApps() {
  const apps = useAppSelector((state) => state.localApps)
  const navigate = useNavigate()

  const onAppRegister = (app: LocalApp) => {
    // Handle app registration logic here
    console.log("Registering app:", app)
    getApi().api.registerApp({ name: app.name })
  }

  return (
    <Container>
      <Stack>
        <Group justify="space-between">
          <Title order={1}>Local Apps</Title>
          <ActionIcon size="lg" onClick={() => navigate("./new")}>
            <IconPlus />
          </ActionIcon>
        </Group>

        {apps && <LocalAppsList apps={apps} onRegister={onAppRegister} />}
      </Stack>
    </Container>
  )
}
