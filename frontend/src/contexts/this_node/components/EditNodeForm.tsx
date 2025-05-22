import { useForm } from "react-hook-form"
import { NodeDetails } from "../types"
import { Input, VStack } from "@chakra-ui/react"

import { Field, Button, FormActions } from "../../../components"

export interface EditNodeData {
  name: string
}

export default function EditNodeForm({
  node,
  onSubmit,
}: {
  node: NodeDetails
  onSubmit: (data: EditNodeData) => void
}) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<EditNodeData>({ defaultValues: { name: node.name } })

  return (
    <VStack gap={4} align="stretch">
      <form onSubmit={handleSubmit(onSubmit)}>
        <Field
          label="Node Name"
          helperText={`A name to identify your Node - use lowercase letters and no spaces`}
          invalid={!!errors.name}
          errorText={errors.name?.message}
        >
          <Input
            {...register("name", {
              required: "This is required",
              maxLength: {
                value: 50,
                message: "Must be less than 50 characters",
              },
              pattern: {
                value: /^[a-z]+(-[a-z]+)*$/,
                message: "Lowercase letters only, no spaces, hyphens allowed",
              },
            })}
          />
        </Field>

        <FormActions>
          <Button loading={isSubmitting} type="submit">
            Update
          </Button>
        </FormActions>
      </form>
    </VStack>
  )
}
