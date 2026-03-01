import { Stack } from "@mantine/core"
import { RegionNodeDetails, RegionNodeStatus } from "../../../api/Api"
import NodeCard from "./NodeCard"
import NodeJoinRequestCard from "./NodeJoinRequestCard"
import { ActionPromiseResult } from "../../../components"

interface NodesListProps {
  nodes: RegionNodeDetails[]
  regionCreatorId?: string | null
  canAdminister?: boolean
  onApprove?: (regionNode: RegionNodeDetails) => Promise<ActionPromiseResult>
}

export default function NodesList({
  nodes,
  regionCreatorId,
  canAdminister,
  onApprove,
}: NodesListProps) {
  return (
    <Stack>
      {nodes.map((node) => {
        const isRegionCreator = regionCreatorId === node.node_id

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
          />
        )
      })}
    </Stack>
  )
}
