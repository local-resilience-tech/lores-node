import { Container } from "@mantine/core"
import { Outlet } from "react-router"

export default function AuthLayout() {
  return (
    <Container mt="xl" maw={600}>
      <Outlet />
    </Container>
  )
}
