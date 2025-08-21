import { Stack, Title, Text } from "@mantine/core"
import AdminLoginForm from "../components/AdminLoginForm"

export default function AdminLogin() {
  const onSubmit = async (values: { password: string }) => {
    // Handle login logic here
  }

  return (
    <Stack gap="lg">
      <Stack gap={0}>
        <Text c="dimmed" style={{ fontSize: "1.5rem" }} fw="bold" mb={-5}>
          Lores Node
        </Text>
        <Title order={1}>Log in as node admin</Title>
      </Stack>
      <Stack gap="md">
        <Text>
          Logging in as the node admin is done with the password you received
          during setup.
        </Text>
        <Text>
          The only thing you can do as the admin is setup regular users for this
          node that you use for all other operations.
        </Text>
      </Stack>
      <AdminLoginForm onSubmit={onSubmit} />
    </Stack>
  )
}
