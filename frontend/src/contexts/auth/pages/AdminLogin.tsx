import { Stack, Title, Text } from "@mantine/core"
import AdminLoginForm from "../components/AdminLoginForm"
import { getApi } from "../../../api"
import { useState } from "react"
import { useNavigate } from "react-router-dom"

type AuthResult = "unauthorized" | "server_error"

export default function AdminLogin() {
  const [result, setResult] = useState<AuthResult | null>(null)
  const navigate = useNavigate()

  const onSubmit = async (values: { password: string }) => {
    getApi()
      .auth.adminLogin(values)
      .then((response) => {
        console.log("response", response)
        navigate("/admin/node_stewards")
      })
      .catch((error) => {
        console.error("error", error)
        switch (error.status) {
          case 401:
            setResult("unauthorized")
            break
          default:
            setResult("server_error")
        }
      })
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
      {result === "unauthorized" && (
        <Text c="red">Invalid credentials, please try again.</Text>
      )}
      {result === "server_error" && (
        <Text c="red">
          An unexpected error occurred, please try again later.
        </Text>
      )}
      <AdminLoginForm onSubmit={onSubmit} />
    </Stack>
  )
}
