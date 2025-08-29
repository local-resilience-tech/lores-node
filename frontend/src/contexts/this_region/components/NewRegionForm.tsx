import { TextInput, Button, Stack } from "@mantine/core"
import { useForm } from "@mantine/form"
import { BootstrapNodeData } from "../../../api/Api"
import { ActionPromiseResult } from "../../../components"

export interface NewRegionData {
  name: string
}

interface NewRegionFormProps {
  onSubmit: (data: BootstrapNodeData) => Promise<ActionPromiseResult>
}

export default function NewRegionForm({ onSubmit }: NewRegionFormProps) {
  const form = useForm<NewRegionData>({
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

  const handleSubmit = (values: NewRegionData) => {
    const data: BootstrapNodeData = {
      network_name: values.name,
      node_id: null,
    }
    onSubmit(data)
  }

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack>
        <Stack>
          <TextInput
            label="Region Name"
            placeholder="Enter region name"
            description="A name to identify your Region - use lowercase letters and no spaces"
            key="name"
            {...form.getInputProps("name")}
          />
        </Stack>

        <Stack>
          <Button loading={form.submitting} type="submit">
            Create Region
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
