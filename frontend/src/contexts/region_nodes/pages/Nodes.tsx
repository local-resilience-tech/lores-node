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
import { getApi } from "../../../api"
import { actionFailure, actionSuccess } from "../../../components"
import { use } from "react"

export default function Nodes() {
  const region = useAppSelector((state) =>
    activeRegionWithNodes(state.my_regions),
  )
  const thisNodeId = useAppSelector((state) => state.network?.node.id)
  const isNodeAdmin =
    thisNodeId != null && region?.region.creator_node_id === thisNodeId

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

  const onApproveJoin = async (regionNode: RegionNodeDetails) => {
    console.log("Approving join for region node ID:", regionNode.id)
    return getApi()
      .nodeStewardApi.approveJoinRequest({
        node_id: regionNode.node_id,
        region_id: regionNode.region_id,
      })
      .then((result) => {
        return actionSuccess()
      })
      .catch((error) => {
        return actionFailure(error)
      })
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
              onApprove={onApproveJoin}
              canAdminister={isNodeAdmin}
            />
          </Tabs.Panel>
        </Tabs>
      </Stack>
    </Container>
  )
}
