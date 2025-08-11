import { ActionIcon, Container, Group, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import LocalAppsList, { LocalAppWithRepo } from "../components/LocalAppsList"
import { LocalApp } from "../../../api/Api"
import { getApi } from "../../../api"
import { IconPlus } from "@tabler/icons-react"
import { useNavigate } from "react-router-dom"
import { useState } from "react"

type AppErrors = Map<string, string>

export default function LocalApps() {
  const apps = useAppSelector((state) => state.localApps)
  const repos = useAppSelector((state) => state.appRepos)

  let appsWithRepos: LocalAppWithRepo[] =
    apps?.map((app) => {
      const repo = repos?.find((r) => r.name === app.repo_name)
      const repo_app_definition = repo?.apps?.find(
        (def) => def.name === app.name
      )
      return { app, repo, repo_app_definition }
    }) || []

  const navigate = useNavigate()
  const [appErrors, setAppErrors] = useState<AppErrors>(new Map())

  const handleError = (error: any, app: LocalApp) => {
    const message = error.response?.data || "Unknown error"
    setAppErrors((prev) => new Map(prev).set(app.name, message))
    console.error("Error:", message)
  }

  const onAppDeploy = async (app: LocalApp) => {
    console.log("Deploying app:", app)
    getApi()
      .api.deployLocalApp(app.name)
      .catch((error) => handleError(error, app))
  }

  const onAppRemoveDeploy = async (app: LocalApp) => {
    console.log("Deploying app:", app)
    getApi()
      .api.removeDeploymentOfLocalApp(app.name)
      .catch((error) => handleError(error, app))
  }

  const onAppRegister = async (app: LocalApp) => {
    console.log("Registering app:", app)
    getApi()
      .api.registerApp({ app_name: app.name })
      .catch((error) => handleError(error, app))
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
            apps={appsWithRepos}
            onDeploy={onAppDeploy}
            onRemoveDeploy={onAppRemoveDeploy}
            onRegister={onAppRegister}
            appErrors={appErrors}
          />
        )}
      </Stack>
    </Container>
  )
}
