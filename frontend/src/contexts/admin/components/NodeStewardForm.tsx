import { Button, Stack, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import { NodeSteward } from "../../../api/Api"

interface NodeStewardFormProps {
  onSubmit: (values: NodeSteward) => Promise<void>
}

export default function NodeStewardForm({ onSubmit }: NodeStewardFormProps) {
  const form = useForm<NodeSteward>({
    initialValues: {
      id: "",
      name: "",
      active: true,
    },
    validate: {
      name: (value) => {
        if (!value) return "Name is required"
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack gap="lg">
        <Stack gap="md">
          <TextInput
            label="Name"
            description="A display name for this user, they can change it later"
            placeholder="eg: Octavia"
            {...form.getInputProps("name")}
          />
        </Stack>
        <Button type="submit" loading={form.submitting}>
          Install
        </Button>
      </Stack>
    </form>
  )
}
