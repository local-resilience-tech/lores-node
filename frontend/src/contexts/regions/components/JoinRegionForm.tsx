import { TextInput, Button, Stack, Text } from "@mantine/core"
import { useForm } from "@mantine/form"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"

export interface JoinRegionData {
  id: string
}

interface JoinRegionFormProps {
  onSubmit: (data: JoinRegionData) => Promise<ActionPromiseResult>
}

export default function JoinRegionForm({ onSubmit }: JoinRegionFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<JoinRegionData>(onSubmit)

  const form = useForm<JoinRegionData>({
    mode: "controlled",
    initialValues: {
      id: "",
    },
    validate: {
      id: (value) => {
        if (!value) return "This is required"
        return null
      },
    },
  })

  const handleSubmit = (
    values: JoinRegionData,
  ): Promise<ActionPromiseResult> => {
    return onSubmitWithResult(values)
  }

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack gap="lg">
        <Text>
          To join a region, you need to know the ID of the region. This will be
          a fairly long string (64 characters) that is generated when the region
          is created. You can ask the creator of the region for this ID.
        </Text>

        <Stack>
          <TextInput
            label="Region ID"
            placeholder="Enter region ID"
            key="id"
            {...form.getInputProps("id")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Stack>
          <Button loading={form.submitting} type="submit">
            Join Region
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
