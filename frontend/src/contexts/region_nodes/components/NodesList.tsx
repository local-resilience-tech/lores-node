import { Stack } from "@mantine/core"
import { RegionNodeDetails, RegionNodeStatus } from "../../../api/Api"
import NodeCard from "./NodeCard"
import NodeJoinRequestCard from "./NodeJoinRequestCard"
import { ActionPromiseResult } from "../../../components"
import { NodeHeartbeatDisplay } from "../../../hooks/useNodeHeartbeats"

interface NodesListProps {
  nodes: RegionNodeDetails[]
  regionCreatorId?: string | null
  canAdminister?: boolean
  onApprove?: (regionNode: RegionNodeDetails) => Promise<ActionPromiseResult>
  thisNodeId?: string
  getNodeHeartbeatDisplay?: (nodeId: string) => NodeHeartbeatDisplay
}

export default function NodesList({
  nodes,
  regionCreatorId,
  canAdminister,
  onApprove,
  thisNodeId,
  getNodeHeartbeatDisplay: getNodeheartbeatDisplay,
}: NodesListProps) {
  return (
    <Stack>
      {nodes.map((node) => {
        const isRegionCreator = regionCreatorId === node.node_id
        const nodeHeartbeatDisplay =
          getNodeheartbeatDisplay && getNodeheartbeatDisplay(node.node_id)
        const isThisNode = node.node_id === thisNodeId

        if (node.status == RegionNodeStatus.RequestedToJoin) {
          return (
            <NodeJoinRequestCard
              key={node.id}
              node={node}
              onApprove={onApprove}
              canAdminister={canAdminister}
            />
          )
        }
        return (
          <NodeCard
            key={node.id}
            node={node}
            isRegionCreator={isRegionCreator}
            isThisNode={isThisNode}
            nodeHeartbeatDisplay={nodeHeartbeatDisplay}
          />
        )
      })}
    </Stack>
  )
}
