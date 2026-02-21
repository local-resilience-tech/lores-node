import { Stack, TextInput, Button, Select } from "@mantine/core"
import { useForm } from "@mantine/form"

import type { RegionNodeStatusData } from "../../../api/Api"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

export default function PostStatus({
  onSubmit,
}: {
  onSubmit: (data: RegionNodeStatusData) => Promise<ActionPromiseResult>
}) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<RegionNodeStatusData>(onSubmit)

  const form = useForm<RegionNodeStatusData>({
    mode: "controlled",
    initialValues: {},
    validate: {
      text: (value) => {
        if (!value) return "This is required"
        if (value.length > 255) return "Must be less than 255 characters"
        return null
      },
      state: (value) => {
        if (!value) return null // Optional field
        const validStates = ["active", "inactive", "maintenance", "development"]
        if (!validStates.includes(value)) return "Invalid state"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack>
        <Stack>
          <TextInput
            label="Text"
            placeholder="Enter status text"
            description="Describe the status of your node. This will be displayed to other nodes."
            key="text"
            {...form.getInputProps("text")}
          />

          <Select
            label="State"
            description={`Optional: Set the state of your node. This can help other nodes understand your node's current status.`}
            placeholder="Select state"
            data={[
              { value: "active", label: "Active" },
              { value: "inactive", label: "Inactive" },
              { value: "maintenance", label: "Maintenance" },
              { value: "development", label: "Development" },
            ]}
            key="state"
            {...form.getInputProps("state")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Button loading={form.submitting} type="submit">
          Post
        </Button>
      </Stack>
    </form>
  )
}
