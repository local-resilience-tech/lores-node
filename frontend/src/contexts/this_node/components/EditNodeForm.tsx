import { useForm } from "react-hook-form"
import { NodeDetails } from "../types"
import { Input, VStack } from "@chakra-ui/react"

import { Field, Button, FormActions } from "../../../components"
import { UpdateNodeData } from "../api"

export default function EditNodeForm({
  node,
  onSubmit,
}: {
  node: NodeDetails
  onSubmit: (data: UpdateNodeData) => void
}) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<UpdateNodeData>({ defaultValues: { name: node.name } })

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <VStack gap={4} align="stretch">
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

        <Field
          label="Public IPv4"
          helperText={`The public IPv4 address of your node`}
          invalid={!!errors.public_ipv4}
          errorText={errors.public_ipv4?.message}
        >
          <Input
            {...register("public_ipv4", {
              required: false,
              pattern: {
                value: /^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$/,
                message: "Invalid IPv4 address",
              },
            })}
          />
        </Field>

        <FormActions>
          <Button loading={isSubmitting} type="submit">
            Update
          </Button>
        </FormActions>
      </VStack>
    </form>
  )
}
