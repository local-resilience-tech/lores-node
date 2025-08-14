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
import { AppRepoAppReference, AppRepo } from "../../../api/Api"
import {
  ActionPromiseResult,
  ActionResult,
  Anchor,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"
import { useState } from "react"

interface InstallAppRepositoryFormProps {
  appRepos: AppRepo[] | null
  onSubmit: (values: AppRepoAppReference) => Promise<ActionPromiseResult | void>
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

  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<AppRepoAppReference>(onSubmit)

  const form = useForm<AppRepoAppReference>({
    mode: "controlled",
    initialValues: {
      repo_name: "",
      app_name: "",
      version: "",
    },
    validate: {
      repo_name: (value) => (value ? null : "Repository name is required"),
      app_name: (value) => (value ? null : "App name is required"),
      version: (value) => (value ? null : "Version is required."),
    },
  })

  const repoNames = appRepos.map((repo) => repo.name)
  const currentRepo = form.values.repo_name
    ? appRepos.find((repo) => repo.name === form.values.repo_name)
    : undefined
  const apps = currentRepo?.apps || []
  const currentApp = form.values.app_name
    ? apps.find((app) => app.name === form.values.app_name)
    : undefined
  const versions = currentApp?.versions || []

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
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
            disabled={!currentRepo}
            data={["", ...apps.map((app) => app.name)]}
            key="app_name"
            {...form.getInputProps("app_name")}
          />

          <NativeSelect
            label="Version"
            description="Select the version to install"
            disabled={!currentApp}
            data={[
              "",
              ...versions.map((version) => ({
                value: version,
                label:
                  version == currentApp?.latest_version
                    ? `${version} (latest)`
                    : version,
              })),
            ]}
            key="version"
            {...form.getInputProps("version")}
          />
        </Stack>

        <DisplayActionResult
          result={actionResult}
          handlers={{
            InUse: (
              <Text c="red">
                <strong>Install failed - Repository in use.</strong>
                <br />
                The repository is checked out to another version, which
                generally means that it's in-use for another upgrade or install.
                It's possible that it got stuck in this state due to a crash
                during an upgrade. If you are confident that nothing else is
                using this repository, you can refresh it on the{" "}
                <Anchor href={`/this_node/app_repos/repo/${currentRepo?.name}`}>
                  {currentRepo?.name}
                </Anchor>{" "}
                repository page. Then come back here and try the upgrade again.
              </Text>
            ),
          }}
        />
        <Button type="submit" loading={form.submitting}>
          Install App
        </Button>
      </Stack>
    </form>
  )
}
