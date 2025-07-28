import { Button, Stack, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import { validateUrl } from "@the-node-forge/url-validator"

interface NewLocalAppFormValues {
  gitUrl: string
}

export default function InstallAppRepositoryForm() {
  const form = useForm<NewLocalAppFormValues>({
    initialValues: {
      gitUrl: "",
    },
    validate: {
      gitUrl: (value) => {
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
    <form
      onSubmit={form.onSubmit((values) => {
        console.log(values)
      })}
    >
      <Stack gap="lg">
        <Stack gap="md">
          <TextInput
            label="Git url"
            description="Use the https clone url of the repository"
            placeholder="eg: https://github.com/local-resilience-tech/apps.git"
            {...form.getInputProps("gitUrl")}
          />
        </Stack>
        <Button type="submit">Install App</Button>
      </Stack>
    </form>
  )
}
