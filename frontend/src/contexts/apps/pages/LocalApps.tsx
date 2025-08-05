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

  const onAppDeploy = (app: LocalApp) => {
    console.log("Deploying app:", app)
    getApi().api.deployLocalApp(app.name)
  }

  const onAppRemoveDeploy = (app: LocalApp) => {
    console.log("Deploying app:", app)
    getApi().api.removeDeploymentOfLocalApp(app.name)
  }

  const onAppRegister = (app: LocalApp) => {
    console.log("Registering app:", app)
    getApi().api.registerApp({ app_name: app.name })
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

        {apps && (
          <LocalAppsList
            apps={apps}
            onDeploy={onAppDeploy}
            onRemoveDeploy={onAppRemoveDeploy}
            onRegister={onAppRegister}
          />
        )}
      </Stack>
    </Container>
  )
}
