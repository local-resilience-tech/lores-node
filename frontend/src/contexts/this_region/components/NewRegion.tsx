import { Text, TextInput, Button, Stack } from "@mantine/core"
import { useForm } from "@mantine/form"

export interface NewRegionData {
  name: string
}

export type SubmitNewRegionFunc = (data: NewRegionData) => void

export default function NewRegion({
  onSubmitNewRegion,
}: {
  onSubmitNewRegion: SubmitNewRegionFunc
}) {
  const form = useForm<NewRegionData>({
    mode: "controlled",
    initialValues: {
      name: "",
    },
    validate: {
      name: (value) => {
        if (!value) return "This is required"
        if (value.length > 50) return "Must be less than 50 characters"
        if (!/^[a-z]+(-[a-z]+)*$/.test(value))
          return "Lowercase letters only, no spaces, hyphens allowed"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitNewRegion)}>
      <Stack>
        <Stack>
          <TextInput
            label="Region Name"
            placeholder="Enter region name"
            description="A name to identify your Region - use lowercase letters and no spaces"
            key="name"
            {...form.getInputProps("name")}
          />
        </Stack>

        <Stack>
          <Button loading={form.submitting} type="submit">
            Create Region
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
