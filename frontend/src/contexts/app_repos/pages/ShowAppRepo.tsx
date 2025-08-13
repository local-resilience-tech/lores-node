import {
  Container,
  Stack,
  Breadcrumbs,
  Title,
  Card,
  Text,
  Group,
} from "@mantine/core"
import { useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import { Anchor, LoadingActionIcon } from "../../../components"
import AppRepoDetails from "../components/AppRepoDetails"
import { AppRepo } from "../../../api/Api"
import AppsForRepoList from "../components/AppsForRepoList"
import { IconRefresh } from "@tabler/icons-react"

export default function ShowAppRepo() {
  const { repoName } = useParams<{ repoName: string }>()
  const appRepo: AppRepo | undefined = useAppSelector((state) =>
    (state.appRepos || []).find((a) => a.name === repoName)
  )

  if (!repoName) {
    return <Container>Error: Repository name is required</Container>
  }

  if (!appRepo) {
    return <Container>Error: Repository not found</Container>
  }

  const refreshRepo = async () => {
    // wait two seconds
    await new Promise((resolve) => setTimeout(resolve, 2000))
  }

  return (
    <Container>
      <Stack gap="lg">
        <Stack gap="xs">
          <Breadcrumbs>
            <Anchor href="/this_node/app_repos">App Repositories</Anchor>
            <Text c="dimmed">{appRepo.name}</Text>
          </Breadcrumbs>
          <Group justify="space-between">
            <Title order={1}>
              <Text span inherit c="dimmed">
                Repository:{" "}
              </Text>
              {appRepo.name}
            </Title>
            <LoadingActionIcon onClick={refreshRepo}>
              <IconRefresh />
            </LoadingActionIcon>
          </Group>
        </Stack>

        <Stack gap="xs">
          <Title order={2}>Details</Title>
          <Card>
            <Card.Section>
              <AppRepoDetails appRepo={appRepo} />
            </Card.Section>
          </Card>
        </Stack>

        <Stack gap="xs">
          <Title order={2}>Apps</Title>
          <Card>
            <Card.Section>
              <AppsForRepoList apps={appRepo.apps} />
            </Card.Section>
          </Card>
        </Stack>
      </Stack>
    </Container>
  )
}
