import { Button, PasswordInput, Stack, TextInput, Text } from "@mantine/core"
import { useForm } from "@mantine/form"
import { NodeStewardSetPasswordRequest } from "../../../../api/Api"
import {
  ActionPromiseResult,
  Anchor,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../../components"
import { DisplayFormError } from "../../../../components/ActionResult"

interface NodeStewardLoginFormProps {
  onSubmit: (
    values: NodeStewardSetPasswordRequest
  ) => Promise<ActionPromiseResult>
}

export default function NodeStewardLoginForm({
  onSubmit,
}: NodeStewardLoginFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<NodeStewardSetPasswordRequest>(onSubmit)

  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      id: "",
      token: "",
      new_password: "",
    },

    validate: {
      id: (value) => (value ? null : "ID is required"),
      token: (value) => (value ? null : "Token is required"),
      new_password: (value) => {
        if (!value) return "New password is required"
        if (value.length < 8)
          return "Password must be at least 8 characters long"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <TextInput
          label="ID"
          placeholder="Node steward ID"
          {...form.getInputProps("id")}
        />

        <TextInput
          label="Token"
          placeholder="Node steward token"
          {...form.getInputProps("token")}
        />

        <PasswordInput
          label="New password"
          placeholder="Node steward new password"
          {...form.getInputProps("new_password")}
        />

        <DisplayActionResult
          result={actionResult}
          handlers={{
            InvalidId: (
              <DisplayFormError
                heading="Set password failed - Invalid ID."
                description="Check that you typed the ID correctly."
              />
            ),
            InvalidToken: (
              <DisplayFormError
                heading="Set password failed - Invalid token."
                description="Check that you typed the token correctly."
              />
            ),
            TokenExpired: (
              <DisplayFormError
                heading="Set password failed - Token expired."
                description="The token you provided has expired. These tokens only last for 24 hours. Ask the node administrator to generate a new one for you."
              />
            ),
            InvalidNewPassword: (
              <DisplayFormError
                heading="Set password failed - Invalid new password."
                description="The new password is not strong enough."
              />
            ),
          }}
        />

        <Button fullWidth type="submit" loading={form.submitting}>
          Set password
        </Button>
      </Stack>
    </form>
  )
}
