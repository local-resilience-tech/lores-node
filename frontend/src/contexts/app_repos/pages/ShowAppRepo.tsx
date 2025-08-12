import { Container, Stack, Breadcrumbs, Title, Card, Text } from "@mantine/core"
import { useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import { Anchor } from "../../../components"
import AppRepoDetails from "../components/AppRepoDetails"

export default function ShowAppRepo() {
  const { repoName } = useParams<{ repoName: string }>()
  const appRepo = useAppSelector((state) =>
    (state.appRepos || []).find((a) => a.name === repoName)
  )

  if (!repoName) {
    return <Container>Error: Repository name is required</Container>
  }

  if (!appRepo) {
    return <Container>Error: Repository not found</Container>
  }

  return (
    <Container>
      <Stack gap="lg">
        <Stack gap="xs">
          <Breadcrumbs>
            <Anchor href="/this_node/app_repos">App Repositories</Anchor>
            <Text c="dimmed">{appRepo.name}</Text>
          </Breadcrumbs>
          <Title order={1}>
            <Text span inherit c="dimmed">
              Repository:{" "}
            </Text>
            {appRepo.name}
          </Title>
        </Stack>

        <Stack gap="xs">
          <Title order={2}>Details</Title>
          <Card>
            <Card.Section>
              <AppRepoDetails appRepo={appRepo} />
            </Card.Section>
          </Card>
        </Stack>
      </Stack>
    </Container>
  )
}
