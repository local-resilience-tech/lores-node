import { Button, Stack, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"

interface NewLocalAppFormValues {
  gitUrl: string
  path: string
}

export default function InstallAppRepositoryForm() {
  const form = useForm<NewLocalAppFormValues>({
    initialValues: {
      gitUrl: "",
      path: "",
    },
    validate: {
      gitUrl: (value) => (value ? null : "Git URL is required"),
      path: (value) => (value ? null : "Path is required"),
    },
  })

  return (
    <form>
      <Stack gap="lg">
        <Stack gap="md">
          <TextInput
            label="Git url"
            description="Use the https clone url of the repository"
            placeholder="https://github.com/local-resilience-tech/apps.git"
            {...form.getInputProps("gitUrl")}
          />
          <TextInput
            label="Path"
            description="Path within the git repository, or leave empty for root"
            placeholder="/apps/my-app"
            {...form.getInputProps("path")}
          />
        </Stack>
        <Button type="submit">Install App</Button>
      </Stack>
    </form>
  )
}
