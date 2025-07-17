import { Stack, Text, TextInput, Button } from "@mantine/core"
import { useForm } from "@mantine/form"

export interface NewNodeData {
  name: string
}

export type SubmitNewNodeFunc = (data: NewNodeData) => void

export default function NewNode({
  onSubmitNewNode,
}: {
  onSubmitNewNode: SubmitNewNodeFunc
}) {
  const form = useForm<NewNodeData>({
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
    <Stack>
      <Stack>
        <Text size="xl" fw={500}>
          Welcome to your new Node - part of a local, resilient, internet.
        </Text>
        <Text>To get setup, you'll need to choose a node name.</Text>
        <Text>
          Ideally this should be unique within your region, but don't worry,
          you'll have a chance to change it later.
        </Text>
      </Stack>

      <form onSubmit={form.onSubmit(onSubmitNewNode)}>
        <Stack>
          <TextInput
            label="Node Name"
            placeholder="Enter node name"
            description="A name to identify your Node - use lowercase letters and no spaces"
            key="name"
            {...form.getInputProps("name")}
          />
          <Button loading={form.submitting} type="submit">
            Set Name
          </Button>
        </Stack>
      </form>
    </Stack>
  )
}
