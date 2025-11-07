import { Button, PasswordInput, Stack, TextInput, Text } from "@mantine/core"
import { useForm } from "@mantine/form"
import { NodeStewardCredentials } from "../../../../api/Api"
import {
  ActionPromiseResult,
  Anchor,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../../components"
import { DisplayFormError } from "../../../../components/ActionResult"

interface NodeStewardLoginFormProps {
  onSubmit: (values: NodeStewardCredentials) => Promise<ActionPromiseResult>
}

export default function NodeStewardLoginForm({
  onSubmit,
}: NodeStewardLoginFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<NodeStewardCredentials>(onSubmit)

  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      id: "",
      password: "",
    },

    validate: {
      id: (value) => (value ? null : "ID is required"),
      password: (value) => (value ? null : "Password is required"),
    },
  })

  console.log("rendering NodeStewardLoginForm")

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <TextInput
          label="ID"
          placeholder="Node steward ID"
          {...form.getInputProps("id")}
        />

        <PasswordInput
          label="Password"
          placeholder="Node steward password"
          {...form.getInputProps("password")}
        />

        <DisplayActionResult
          result={actionResult}
          redirectToLogin={false}
          handlers={{
            NoPasswordSet: (
              <DisplayFormError
                heading="Login failed - No password set."
                description={
                  <Text c="red">
                    This user has not set up their password yet.{" "}
                    <Anchor href="../set_password">
                      Click here to set a password
                    </Anchor>
                    .
                  </Text>
                }
              />
            ),
            InvalidCredentials: (
              <DisplayFormError heading="Login failed - Invalid credentials." />
            ),
            AccountDisabled: (
              <DisplayFormError
                heading="Login failed - Your account is disabled."
                description={
                  <Text c="red">
                    The node admin user has disabled your account, you can reach
                    out to them for assistance.
                  </Text>
                }
              />
            ),
          }}
        />

        <Button fullWidth type="submit" loading={form.submitting}>
          Log in
        </Button>
      </Stack>
    </form>
  )
}
