import { Stack, Title, Text } from "@mantine/core"
import { useEffect } from "react"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"

export default function NodeStewardsList() {
  const navigate = useNavigate()

  const listNodeStewards = async () => {
    getApi()
      .adminApi.listNodeStewards()
      .then((response) => {
        console.log("Node Stewards:", response.data)
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
    listNodeStewards()
  }, [])

  return (
    <Stack>
      <Title>Node Stewards</Title>
      <Text>stewards list</Text>
    </Stack>
  )
}
