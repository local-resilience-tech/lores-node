import { Container, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import AppsList from "../components/AppsList"
import { App } from "../../../api/Api"
import { getApi } from "../../../api"

export default function LocalApps() {
  const apps = useAppSelector((state) => state.apps)

  const onAppRegister = (app: App) => {
    // Handle app registration logic here
    console.log("Registering app:", app)
    getApi().api.registerApp({ name: app.name })
  }

  return (
    <Container>
      <Stack>
        <Title order={1}>Local Apps</Title>

        {apps && <AppsList apps={apps} onRegister={onAppRegister} />}
      </Stack>
    </Container>
  )
}
