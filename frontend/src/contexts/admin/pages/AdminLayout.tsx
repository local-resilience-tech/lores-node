import { Container } from "@mantine/core"
import { Outlet } from "react-router"

export default function AdminLayout() {
  return (
    <Container pt="xl">
      <Outlet />
    </Container>
  )
}
