import { TextInput, Button, Stack, Text } from "@mantine/core"
import { useForm } from "@mantine/form"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"
import { CreateRegionData } from "../../../api/Api"

interface CreateRegionFormProps {
  onSubmit: (data: CreateRegionData) => Promise<ActionPromiseResult>
}

export default function CreateRegionForm({ onSubmit }: CreateRegionFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<CreateRegionData>(onSubmit)

  const form = useForm<CreateRegionData>({
    mode: "controlled",
    initialValues: {
      slug: "",
      name: "",
      organisation_name: "",
      organisation_url: "",
      node_steward_conduct_url: "",
      user_conduct_url: "",
      user_privacy_url: "",
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
        if (value) {
          if (value.length > 100) return "Must be less than 100 characters"
        }
        return null
      },
      organisation_url: (value) => {
        if (value) {
          if (value && !/^https?:\/\/\S+$/.test(value))
            return "Must be a valid URL starting with http:// or https://"
        }
        return null
      },
      node_steward_conduct_url: (value) => {
        if (value) {
          if (value && !/^https?:\/\/\S+$/.test(value))
            return "Must be a valid URL starting with http:// or https://"
        }
        return null
      },
      user_conduct_url: (value) => {
        if (value) {
          if (value && !/^https?:\/\/\S+$/.test(value))
            return "Must be a valid URL starting with http:// or https://"
        }
        return null
      },
      user_privacy_url: (value) => {
        if (value) {
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
          When you create a new region, it will get a unique ID, but you can
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
            label="Organisation URL"
            description="The URL for your regional organisation or project (optional)"
            placeholder="eg https://merri-crk.coop"
            key="organisation_url"
            {...form.getInputProps("organisation_url")}
          />

          <TextInput
            label="Node Steward Conduct URL"
            description="A URL describing the agreements for conduct of node stewards in this region (optional)"
            placeholder="eg https://merri-crk.coop/node-steward-conduct"
            key="node_steward_conduct_url"
            {...form.getInputProps("node_steward_conduct_url")}
          />

          <TextInput
            label="User Conduct URL"
            description="A URL describing the agreements for conduct of users in this region (optional)"
            placeholder="eg https://merri-crk.coop/user-conduct"
            key="user_conduct_url"
            {...form.getInputProps("user_conduct_url")}
          />

          <TextInput
            label="User Privacy URL"
            description="A URL describing how user data is handled in this region (optional)"
            placeholder="eg https://merri-crk.coop/user-privacy"
            key="user_privacy_url"
            {...form.getInputProps("user_privacy_url")}
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
