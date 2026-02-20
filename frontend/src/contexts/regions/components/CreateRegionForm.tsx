import { TextInput, Button, Stack, Text } from "@mantine/core"
import { useForm } from "@mantine/form"
import { BootstrapNodeData } from "../../../api/Api"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

export interface CreateRegionData {
  slug: string
  name: string
  organisation_name?: string
  url?: string
}

interface NewRegionFormProps {
  onSubmit: (data: CreateRegionData) => Promise<ActionPromiseResult>
}

export default function CreateRegionForm({ onSubmit }: NewRegionFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<CreateRegionData>(onSubmit)

  const form = useForm<CreateRegionData>({
    mode: "controlled",
    initialValues: {
      slug: "",
      name: "",
      organisation_name: "",
      url: "",
    },
    validate: {
      slug: (value) => {
        if (!value) return "This is required"
        if (value.length > 50) return "Must be less than 50 characters"
        if (!/^[a-z0-9]+(-[a-z0-9]+)*$/.test(value))
          return "Lowercase letters and numbers only, no spaces, hyphens allowed"
        return null
      },
      name: (value) => {
        if (!value) return "This is required"
        if (value.length > 100) return "Must be less than 100 characters"
        return null
      },
      organisation_name: (value) => {
        if (value !== undefined) {
          if (value.length > 100) return "Must be less than 100 characters"
        }
        return null
      },
      url: (value) => {
        if (value !== undefined) {
          if (value.length > 200) return "Must be less than 200 characters"
          if (value && !/^https?:\/\/\S+$/.test(value))
            return "Must be a valid URL starting with http:// or https://"
        }
        return null
      },
    },
  })

  const handleSubmit = (values: CreateRegionData) => onSubmitWithResult(values)

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack gap="lg">
        <Text>
          When you create a new region, it will get a unique id, but you can
          choose some details to describe it.
        </Text>

        <Stack>
          <TextInput
            label="Slug"
            description="A machine friendly name for your region - use lowercase letters and no spaces"
            placeholder="eg merri-crk"
            key="slug"
            withAsterisk
            {...form.getInputProps("slug")}
          />

          <TextInput
            label="Region Name"
            description="A name to identify your Region"
            placeholder="eg Merri Creek Catchment"
            key="name"
            withAsterisk
            {...form.getInputProps("name")}
          />

          <TextInput
            label="Organisation Name"
            description="A name for the organisation that manages this region (optional)"
            placeholder="eg Merri Creek Tech Co-op"
            key="organisation_name"
            {...form.getInputProps("organisation_name")}
          />

          <TextInput
            label="URL"
            description="The URL for your region (optional)"
            placeholder="eg https://merri-crk.coop"
            key="url"
            {...form.getInputProps("url")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Stack>
          <Button loading={form.submitting} type="submit">
            Create New Region
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
