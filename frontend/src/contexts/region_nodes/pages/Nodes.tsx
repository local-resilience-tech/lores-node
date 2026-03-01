import {
  Badge,
  Container,
  Stack,
  Tabs,
  Title,
  useMantineTheme,
} from "@mantine/core"
import NodesList from "../components/NodesList"
import { useAppSelector } from "../../../store"
import { activeRegionWithNodes } from "../../../store/my_regions"
import { IconList, IconMessageQuestion } from "@tabler/icons-react"
import { RegionNodeDetails } from "../../../api/Api"

export default function Nodes() {
  const region = useAppSelector((state) =>
    activeRegionWithNodes(state.my_regions),
  )

  const theme = useMantineTheme()
  const joinRequestColor = theme.colors.orange[6]

  if (!region) {
    return <Container>No region</Container>
  }

  const nodes = region.nodes || []
  let member_nodes = [] as RegionNodeDetails[]
  let join_request_nodes = [] as RegionNodeDetails[]

  for (const node of nodes) {
    if (node.status === "RequestedToJoin") {
      join_request_nodes.push(node)
    } else {
      member_nodes.push(node)
    }
  }

  const tabIconSize = 18

  return (
    <Container>
      <Stack>
        <Stack gap={0}>
          <Title order={1}>Nodes</Title>
          <Title order={2} c="dimmed" fz="xl">
            {region.region.name}
          </Title>
        </Stack>

        <Tabs defaultValue="list">
          <Tabs.List>
            <Tabs.Tab
              value="list"
              leftSection={<IconList size={tabIconSize} />}
            >
              List
            </Tabs.Tab>
            <Tabs.Tab
              value="requests"
              leftSection={<IconMessageQuestion size={tabIconSize} />}
              rightSection={
                join_request_nodes.length > 0 ? (
                  <Badge circle color={joinRequestColor}>
                    {join_request_nodes.length}
                  </Badge>
                ) : null
              }
            >
              Join Requests
            </Tabs.Tab>
          </Tabs.List>

          <Tabs.Panel value="list" pt="lg">
            <NodesList
              nodes={member_nodes}
              regionCreatorId={region.region.creator_node_id}
            />
          </Tabs.Panel>

          <Tabs.Panel value="requests" pt="lg">
            <NodesList
              nodes={join_request_nodes}
              regionCreatorId={region.region.creator_node_id}
            />
          </Tabs.Panel>
        </Tabs>
      </Stack>
    </Container>
  )
}
