import { Button, Group, Stack, TextInput } from "@mantine/core"
import { Anchor } from "../../../components"
import { useForm } from "@mantine/form"
import { LocalAppFormData } from "../../../api/Api"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

interface LocalAppFormProps {
  onSubmit: (data: LocalAppFormData) => Promise<ActionPromiseResult>
  submitLabel: string
  cancelPath: string
  initialValues?: Partial<LocalAppFormData>
  disableKeyFields?: boolean
}

export default function LocalAppForm({
  onSubmit,
  submitLabel,
  cancelPath,
  initialValues,
  disableKeyFields,
}: LocalAppFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<LocalAppFormData>(onSubmit)

  const form = useForm<LocalAppFormData>({
    mode: "controlled",
    initialValues: {
      name: "",
      version: "",
      instance_id: "",
      ...initialValues,
    },
    validate: {
      name: (value) => (value.trim() ? null : "Name is required"),
      version: (value) => (value.trim() ? null : "Version is required"),
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="md">
        <TextInput
          label="Name"
          description="A unique name for this app"
          placeholder="eg my-app"
          withAsterisk
          disabled={disableKeyFields}
          key={form.key("name")}
          {...form.getInputProps("name")}
        />

        <TextInput
          label="Instance ID"
          description="Optional instance identifier"
          placeholder="eg my-app-instance"
          disabled={disableKeyFields}
          key={form.key("instance_id")}
          {...form.getInputProps("instance_id")}
        />

        <TextInput
          label="Version"
          description="The version of this app"
          placeholder="eg 1.0.0"
          withAsterisk
          key={form.key("version")}
          {...form.getInputProps("version")}
        />

        <DisplayActionResult result={actionResult} />

        <Group>
          <Button loading={form.submitting} type="submit">
            {submitLabel}
          </Button>
          <Anchor href={cancelPath}>Cancel</Anchor>
        </Group>
      </Stack>
    </form>
  )
}
