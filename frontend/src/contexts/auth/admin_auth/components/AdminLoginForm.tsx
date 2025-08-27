import { Button, PasswordInput, Stack } from "@mantine/core"
import { useForm } from "@mantine/form"

interface AdminLoginFormProps {
  onSubmit: (values: { password: string }) => Promise<void>
}

export default function AdminLoginForm({ onSubmit }: AdminLoginFormProps) {
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      password: "",
    },

    validate: {
      password: (value) => (value ? null : "Password is required"),
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack gap="lg">
        <PasswordInput
          label="Password"
          placeholder="Admin password"
          {...form.getInputProps("password")}
        />
        <Button fullWidth type="submit">
          Log in
        </Button>
      </Stack>
    </form>
  )
}
