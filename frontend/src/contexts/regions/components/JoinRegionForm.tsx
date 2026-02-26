import { TextInput, Button, Stack, Text, Textarea } from "@mantine/core"
import { useForm } from "@mantine/form"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

export interface JoinRegionRequestData {
  id: string
  about_your_node: string
  about_your_stewards: string
  node_steward_conduct_url?: string
}

interface JoinRegionFormProps {
  onSubmit: (data: JoinRegionRequestData) => Promise<ActionPromiseResult>
}

export default function JoinRegionForm({ onSubmit }: JoinRegionFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<JoinRegionRequestData>(onSubmit)

  const form = useForm<JoinRegionRequestData>({
    mode: "controlled",
    initialValues: {
      id: "",
      about_your_node: "",
      about_your_stewards: "",
    },
    validate: {
      id: (value) => {
        if (!value) return "This is required"
        return null
      },
    },
  })

  const handleSubmit = (
    values: JoinRegionRequestData,
  ): Promise<ActionPromiseResult> => {
    return onSubmitWithResult(values)
  }

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack gap="lg">
        <Text>
          To request to join a region, you need to know the ID of the region.
          This will be a fairly long string (64 characters) that is generated
          when the region is created. You can ask the creator of the region for
          this ID.
        </Text>

        <Stack>
          <TextInput
            label="Region ID"
            placeholder="Enter region ID"
            key="id"
            {...form.getInputProps("id")}
          />

          <Textarea
            label="About Your Node"
            description="Tell the region approver(s) a bit about your node. This could include where
            your node is located, and what your intentions are for it. When deciding whether to approve your request, the approver(s) will not yet be able to see other information about your node yet, such as node name or location, so include this if relevant."
            placeholder="Tell us about your node"
            key="about_your_node"
            rows={4}
            {...form.getInputProps("about_your_node")}
          />

          <Textarea
            label="About Your Stewards"
            description="Tell the region approver(s) a bit about the people who will be stewarding your node. Including personal information here like full names and contact details may not be ideal. If you have contacted the region via other means, perhaps reference that."
            placeholder="Tell us about your stewards"
            key="about_your_stewards"
            rows={4}
            {...form.getInputProps("about_your_stewards")}
          />

          <TextInput
            label="Agree to code of conduct for node stewards (optional)"
            description="If the region has a code of conduct, and they have shared the URL for it with you, post it here to indicate that you have read and agree to it."
            placeholder="https://example.com/conduct"
            key="node_steward_conduct_url"
            {...form.getInputProps("node_steward_conduct_url")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Stack>
          <Button loading={form.submitting} type="submit">
            Request to Join Region
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
