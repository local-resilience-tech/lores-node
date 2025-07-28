import { Button, Stack, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import { validateUrl } from "@the-node-forge/url-validator"
import { AppRepo } from "../../../api/Api"

interface InstallAppRepositoryFormProps {
  onSubmit: (values: AppRepo) => Promise<void>
}

export default function InstallAppRepositoryForm({
  onSubmit,
}: InstallAppRepositoryFormProps) {
  const form = useForm<AppRepo>({
    initialValues: {
      name: "",
      git_url: "",
    },
    validate: {
      name: (value) => {
        if (!value) return "Name is required"
        if (/[^a-zA-Z0-9_-]/.test(value))
          return "Name can only contain lowercase letters, numbers, underscores, and hyphens"
        return null
      },
      git_url: (value) => {
        if (!value) return "Git URL is required"
        if (!validateUrl(value)) return "Invalid URL format"
        if (!value.startsWith("https://"))
          return "Git URL must start with https://"
        if (!value.endsWith(".git")) return "Git URL must end with .git"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack gap="lg">
        <Stack gap="md">
          <TextInput
            label="Name"
            description="A short name for the app repository, which must be a valid folder name"
            placeholder="eg: example-app"
            {...form.getInputProps("name")}
          />
          <TextInput
            label="Git url"
            description="Use the https clone url of the repository"
            placeholder="eg: https://github.com/local-resilience-tech/apps.git"
            {...form.getInputProps("git_url")}
          />
        </Stack>
        <Button type="submit" loading={form.submitting}>
          Install
        </Button>
      </Stack>
    </form>
  )
}
