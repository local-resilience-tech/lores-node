import { Button, PasswordInput, Stack, Text } from "@mantine/core"
import { useForm } from "@mantine/form"
import {
  ActionPromiseResult,
  Anchor,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../../components"
import { DisplayFormError } from "../../../../components/ActionResult"

export interface AdminLoginData {
  password: string
}

interface AdminLoginFormProps {
  onSubmit: (values: AdminLoginData) => Promise<ActionPromiseResult>
}

export default function AdminLoginForm({ onSubmit }: AdminLoginFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<AdminLoginData>(onSubmit)

  const form = useForm<AdminLoginData>({
    mode: "uncontrolled",
    initialValues: {
      password: "",
    },

    validate: {
      password: (value) => (value ? null : "Password is required"),
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <PasswordInput
          label="Password"
          placeholder="Admin password"
          {...form.getInputProps("password")}
        />

        <DisplayActionResult
          result={actionResult}
          handlers={{
            NoPasswordSet: (
              <DisplayFormError
                heading="Login failed - No password set."
                description={
                  <Text c="red">
                    This node is in the process of being setup. As you're the
                    first user here, you're able to{" "}
                    <Anchor href="/setup">set the admin password</Anchor>.
                  </Text>
                }
              />
            ),
            InvalidCredentials: (
              <DisplayFormError heading="Login failed - Invalid credentials." />
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
