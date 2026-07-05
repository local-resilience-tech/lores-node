import { ActionIcon, Container, Group, Stack, Title } from "@mantine/core"
import orderBy from "lodash.orderby"
import { useAppSelector } from "../../../store"
import LocalAppsList from "../components/LocalAppsList"
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { IconPlus } from "@tabler/icons-react"
import { useNavigate } from "react-router-dom"

export default function LocalApps() {
  const apps = useAppSelector((state) => state.localApps)
  const navigate = useNavigate()

  const sortedApps = apps
    ? orderBy(apps, ["app.name", "app.instance_id"])
    : null

  return (
    <Container>
      <Stack>
        <Group justify="space-between">
          <Title order={1}>Local Apps</Title>
          <IfNodeSteward>
            <ActionIcon size="lg" onClick={() => navigate("./new")}>
              <IconPlus />
            </ActionIcon>
          </IfNodeSteward>
        </Group>

        {sortedApps && <LocalAppsList apps={sortedApps} />}
      </Stack>
    </Container>
  )
}
