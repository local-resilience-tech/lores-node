import { Text, Button, Stack, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import { BootstrapNodeRequest } from "../../../api/Api"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

interface AddBootstrapFormProps {
  onSubmit: (data: BootstrapNodeRequest) => Promise<ActionPromiseResult>
}

export default function AddBootstrapNodeForm({
  onSubmit,
}: AddBootstrapFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<BootstrapNodeRequest>(onSubmit)

  const form = useForm<BootstrapNodeRequest>({
    mode: "controlled",
    initialValues: {
      node_id: "",
    },
    validate: {
      node_id: (value) => {
        if (!value) return "This is required"

        // Must be hexadecimal and 64 characters long
        if (!/^[a-fA-F0-9]{64}$/.test(value))
          return "Must be a valid 64-character hexadecimal string"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <Text>
          When you add a bootstrap node, it will be used to help initialize the
          network.
        </Text>

        <Stack>
          <TextInput
            label="Bootstrap Node ID"
            description="A unique identifier for another node - must be a 64-character hexadecimal string"
            key="node_id"
            withAsterisk
            {...form.getInputProps("node_id")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Stack>
          <Button loading={form.submitting} type="submit">
            Add Bootstrap Node
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
