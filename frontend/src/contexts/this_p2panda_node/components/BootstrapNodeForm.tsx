import { Text, TextInput, Button, Stack } from "@mantine/core"
import { useForm } from "@mantine/form"
import { BootstrapNodeData } from "../../../api/Api"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

export type SubmitBootstrapNodeFunc = (
  data: BootstrapNodeData,
) => Promise<ActionPromiseResult>

export default function BootstrapNodeForm({
  onSubmit,
}: {
  onSubmit: SubmitBootstrapNodeFunc
}) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<BootstrapNodeData>(onSubmit)

  const form = useForm<BootstrapNodeData>({
    mode: "controlled",
    initialValues: {
      // network_name: "",
      node_id: "",
    },
    validate: {
      // network_name: (value) => {
      //   if (!value) return "This is required"
      //   if (value.length > 50) return "Must be less than 50 characters"
      //   if (!/^[a-z]+(-[a-z]+)*$/.test(value))
      //     return "Lowercase letters only, no spaces, hyphens allowed"
      //   return null
      // },
      node_id: (value) => {
        if (!value) return "This is required"
        if (value.length > 64) return "Must be no more than 64 characters"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <Text c="dimmed">
          TODO: We don't yet provide much feedback on whether you put in the
          correct details here, please type carefully.
        </Text>
        <Stack>
          {/* <TextInput
            label="Network Name"
            placeholder="Enter network name"
            description="A unique string that defines this region"
            key="network_name"
            {...form.getInputProps("network_name")}
          /> */}
          <TextInput
            label="Node ID"
            placeholder="Enter node ID"
            description="A hex string that identifies another node in this network"
            key="node_id"
            {...form.getInputProps("node_id")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Stack>
          <Button loading={form.submitting} type="submit">
            Connect
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
