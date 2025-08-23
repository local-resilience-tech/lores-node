import { ActionIcon, Container, Group, Stack, Title } from "@mantine/core"
import { IconRefresh } from "@tabler/icons-react"
import { DockerStackWithServices } from "../../../api/Api"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import StacksList from "../components/StacksList"

export default function Stacks() {
  const [stacks, setStacks] = useState<null | DockerStackWithServices[]>(null)

  const loadStacks = async () => {
    getApi()
      .publicApi.listStacks()
      .then((response) => {
        console.log("Stacks loaded:", response.data)
        setStacks(response.data)
      })
      .catch((error) => {
        console.error("Error loading stacks:", error)
      })
  }

  useEffect(() => {
    loadStacks()
  }, [])

  return (
    <Container>
      <Stack>
        <Group justify="space-between">
          <Title order={1}>Docker Stacks</Title>
          <ActionIcon onClick={loadStacks}>
            <IconRefresh />
          </ActionIcon>
        </Group>
        {stacks && <StacksList stacks={stacks} />}
      </Stack>
    </Container>
  )
}
