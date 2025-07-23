import { Container, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import LocalAppsList from "../components/LocalAppsList"
import { LocalApp } from "../../../api/Api"
import { getApi } from "../../../api"

export default function LocalApps() {
  const apps = useAppSelector((state) => state.apps)

  const onAppRegister = (app: LocalApp) => {
    // Handle app registration logic here
    console.log("Registering app:", app)
    getApi().api.registerApp({ name: app.name })
  }

  return (
    <Container>
      <Stack>
        <Title order={1}>Local Apps</Title>

        {apps && <LocalAppsList apps={apps} onRegister={onAppRegister} />}
      </Stack>
    </Container>
  )
}
