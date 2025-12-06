import { Container, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import LocalAppsList, { LocalAppWithRepo } from "../components/LocalAppsList"
import { useNavigate } from "react-router-dom"

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

  return (
    <Container>
      <Stack>
        <Title order={1}>Local Apps</Title>

        {apps && <LocalAppsList apps={appsWithRepos} />}
      </Stack>
    </Container>
  )
}
