import {
  Button,
  Stack,
  TextInput,
  Title,
  Text,
  Card,
  NativeSelect,
} from "@mantine/core"
import { useForm } from "@mantine/form"
import { AppDefinitionReference, AppRepo } from "../../../api/Api"
import { Anchor } from "../../../components"

interface InstallAppRepositoryFormProps {
  appRepos: AppRepo[] | null
  onSubmit: (values: AppDefinitionReference) => Promise<void>
}

function PleaseInstallAppRepository() {
  return (
    <Card>
      <Stack>
        <Title order={2}>Please install an app repository first</Title>
        <Text>
          To install a new local app, you need to have at least one app
          repository configured.
        </Text>
        <Text>
          <Anchor href="/this_node/app_repos">Manage app repos</Anchor>
        </Text>
      </Stack>
    </Card>
  )
}

export default function InstallAppRepositoryForm({
  appRepos,
  onSubmit,
}: InstallAppRepositoryFormProps) {
  if (!appRepos) {
    return <PleaseInstallAppRepository />
  }

  const form = useForm<AppDefinitionReference>({
    mode: "controlled",
    initialValues: {
      repo_name: "",
      app_name: "",
    },
    validate: {
      repo_name: (value) => (value ? null : "Repository name is required"),
      app_name: (value) => (value ? null : "App name is required"),
    },
  })

  const repoNames = appRepos.map((repo) => repo.name)
  const currentRepo = appRepos.find(
    (repo) => repo.name === form.values.repo_name
  )
  const apps = currentRepo?.apps || []

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack gap="lg">
        <Stack gap="md">
          <NativeSelect
            label="Repository"
            description="Select the app repository"
            data={["", ...repoNames]}
            key="repo_name"
            {...form.getInputProps("repo_name")}
          />
          <NativeSelect
            label="App"
            description="Select the app to install"
            data={[
              "",
              ...apps.map((app) => ({
                label: `${app.name} v${app.version}`,
                value: app.name,
              })),
            ]}
            key="app_name"
            {...form.getInputProps("app_name")}
          />
        </Stack>
        <Button type="submit" loading={form.submitting}>
          Install App
        </Button>
      </Stack>
    </form>
  )
}
