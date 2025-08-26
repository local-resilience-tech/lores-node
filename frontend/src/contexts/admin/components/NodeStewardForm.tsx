import { Button, Stack, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import { NodeStewardCreationData } from "../../../api/Api"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

interface NodeStewardFormProps {
  onSubmit: (
    values: NodeStewardCreationData
  ) => Promise<ActionPromiseResult | void>
}

export default function NodeStewardForm({ onSubmit }: NodeStewardFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<NodeStewardCreationData>(onSubmit)

  const form = useForm<NodeStewardCreationData>({
    initialValues: {
      name: "",
    },
    validate: {
      name: (value) => {
        if (!value) return "Name is required"
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <Stack gap="md">
          <TextInput
            label="Name"
            description="A display name for this user, they can change it later"
            placeholder="eg: Octavia"
            {...form.getInputProps("name")}
          />
        </Stack>
        <DisplayActionResult result={actionResult} />
        <Button type="submit" loading={form.submitting}>
          Install
        </Button>
      </Stack>
    </form>
  )
}
