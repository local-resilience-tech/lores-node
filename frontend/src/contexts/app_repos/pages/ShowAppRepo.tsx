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
import { useAppDispatch, useAppSelector } from "../../../store"
import {
  actionFailure,
  actionSuccess,
  Anchor,
  LoadingActionIcon,
} from "../../../components"
import AppRepoDetails from "../components/AppRepoDetails"
import { AppRepo } from "../../../api/Api"
import AppsForRepoList from "../components/AppsForRepoList"
import { IconRefresh } from "@tabler/icons-react"
import { ActionPromiseResult } from "../../../components"
import { getApi } from "../../../api"
import { appRepoUpdated } from "../../../store/app_repos"
import { IfNodeSteward } from "../../auth/node_steward_auth"

export default function ShowAppRepo() {
  const { repoName } = useParams<{ repoName: string }>()
  const appRepo: AppRepo | undefined = useAppSelector((state) =>
    (state.appRepos || []).find((a) => a.name === repoName)
  )
  const dispatch = useAppDispatch()

  if (!repoName) {
    return <Container>Error: Repository name is required</Container>
  }

  if (!appRepo) {
    return <Container>Error: Repository not found</Container>
  }

  const refreshRepo = async (): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.reloadAppRepo(repoName)
      .then((result) => {
        dispatch(appRepoUpdated(result.data))
        return actionSuccess()
      })
      .catch(actionFailure)
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
            <IfNodeSteward>
              <LoadingActionIcon onClick={refreshRepo}>
                <IconRefresh />
              </LoadingActionIcon>
            </IfNodeSteward>
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
