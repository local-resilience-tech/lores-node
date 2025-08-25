import { Container, Text } from "@mantine/core"
import { Outlet, useNavigate } from "react-router"
import { getApi } from "../../../api"
import type { Node } from "../../../api/Api"
import { useEffect, useState } from "react"

export default function AdminLayout() {
  const navigate = useNavigate()
  const [node, setNode] = useState<Node | null>(null)

  const loadNode = () => {
    getApi()
      .adminApi.showThisNode()
      .then((response) => {
        setNode(response.data)
      })
      .catch((error) => {
        if (error.response?.status === 401 || error.response?.status === 403) {
          navigate("/auth/admin/login")
        } else {
          console.error("Error fetching node stewards:", error)
        }
      })
  }

  useEffect(() => {
    loadNode()
  }, [])

  return (
    <Container pt="xl">
      <Text c="dimmed" style={{ fontSize: "1.5rem" }} fw="bold" mb={-5}>
        {node ? `Node: ${node.name}` : "Lores Node"}
      </Text>
      <Outlet />
    </Container>
  )
}
