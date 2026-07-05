import { ActionIcon, Container, Group, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import LocalAppsList from "../components/LocalAppsList"
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { IconPlus } from "@tabler/icons-react"
import { useNavigate } from "react-router-dom"

export default function LocalApps() {
  const apps = useAppSelector((state) => state.localApps)
  const navigate = useNavigate()

  return (
    <Container>
      <Stack>
        <Group justify="space-between">
          <Title order={1}>Local Apps</Title>
          <IfNodeSteward>
            <ActionIcon size="lg" onClick={() => navigate("./edit")}>
              <IconPlus />
            </ActionIcon>
          </IfNodeSteward>
        </Group>

        {apps && <LocalAppsList apps={apps} />}
      </Stack>
    </Container>
  )
}
