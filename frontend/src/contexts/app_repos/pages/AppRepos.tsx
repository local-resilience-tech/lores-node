import { Container, Stack, Title } from "@mantine/core"
import { useNavigate } from "react-router-dom"
import AppRepoList from "../components/AppRepoList"
import { useAppSelector } from "../../../store"

export default function LocalApps() {
  const navigate = useNavigate()
  const repos = useAppSelector((state) => state.appRepos)

  return (
    <Container>
      <Stack>
        <Title order={1}>App Repositories</Title>
        <AppRepoList repos={repos} />
      </Stack>
    </Container>
  )
}
