import { Input, NativeSelect, Select, VStack } from "@chakra-ui/react"
import { Field, Button, FormActions } from "../../../components"
import { useForm } from "react-hook-form"

export interface PostStatusMessageData {
  text: string
  state?: "active" | "inactive" | "maintenance" | "development"
}

export default function PostStatusMessage() {
  const onSubmit = (data: PostStatusMessageData) => {
    // Handle the form submission logic here
    console.log("Form submitted with data:", data)
  }

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<PostStatusMessageData>()

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <VStack gap={4} align="stretch">
        <Field
          label="Text"
          helperText={`Describe the status of your node. This will be displayed to other nodes.`}
          invalid={!!errors.text}
          errorText={errors.text?.message}
        >
          <Input
            {...register("text", {
              required: "This is required",
              maxLength: {
                value: 255,
                message: "Must be less than 255 characters",
              },
            })}
          />
        </Field>

        <Field
          label="State"
          helperText={`Optional: Set the state of your node. This can help other nodes understand your node's current status.`}
          invalid={!!errors.state}
          errorText={errors.state?.message}
        >
          <NativeSelect.Root size="sm" width="240px">
            <NativeSelect.Field placeholder="Select state">
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
              <option value="maintenance">Maintenance</option>
              <option value="development">Development</option>
            </NativeSelect.Field>
            <NativeSelect.Indicator />
          </NativeSelect.Root>
        </Field>

        <FormActions>
          <Button loading={isSubmitting} type="submit">
            Post
          </Button>
        </FormActions>
      </VStack>
    </form>
  )
}
