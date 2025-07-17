import { useForm } from "@mantine/form"
import { FormActions } from "../../../components"
import type { Node, UpdateNodeDetails } from "../../../api/Api"
import { Button, Stack, TextInput } from "@mantine/core"

export default function EditNodeForm({
  node,
  onSubmit,
}: {
  node: Node
  onSubmit: (data: UpdateNodeDetails) => void
}) {
  const form = useForm<UpdateNodeDetails>({
    mode: "controlled",
    initialValues: {
      name: node.name,
      public_ipv4: "",
    },
    validate: {
      name: (value) => {
        if (!value) return "This is required"
        if (value.length > 50) return "Must be less than 50 characters"
        if (!/^[a-z]+(-[a-z]+)*$/.test(value))
          return "Lowercase letters only, no spaces, hyphens allowed"
        return null
      },
      public_ipv4: (value) => {
        if (!value) return null // Optional field
        if (!/^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$/.test(value))
          return "Invalid IPv4 address"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Node name"
          placeholder="Enter node name"
          description="A name to identify your Node - use lowercase letters and no spaces"
          key="name"
          {...form.getInputProps("name")}
        />

        <TextInput
          label="Public IPv4"
          placeholder="Enter public IPv4"
          description="The public IPv4 address of your node"
          key="public_ipv4"
          {...form.getInputProps("public_ipv4")}
        />

        <FormActions>
          <Button loading={form.submitting} type="submit">
            Update
          </Button>
        </FormActions>
      </Stack>
    </form>
  )
}
