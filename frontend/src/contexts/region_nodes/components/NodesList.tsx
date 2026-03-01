import { Stack } from "@mantine/core"
import { RegionNodeDetails, RegionNodeStatus } from "../../../api/Api"
import NodeCard from "./NodeCard"
import NodeJoinRequestCard from "./NodeJoinRequestCard"
import { ActionPromiseResult } from "../../../components"

interface NodesListProps {
  nodes: RegionNodeDetails[]
  regionCreatorId?: string | null
  onApprove?: (regionNode: RegionNodeDetails) => Promise<ActionPromiseResult>
}

export default function NodesList({
  nodes,
  regionCreatorId,
  onApprove,
}: NodesListProps) {
  return (
    <Stack>
      {nodes.map((node) => {
        if (node.status == RegionNodeStatus.RequestedToJoin) {
          return (
            <NodeJoinRequestCard
              key={node.id}
              node={node}
              onApprove={onApprove}
            />
          )
        }
        return (
          <NodeCard
            key={node.id}
            node={node}
            regionCreatorId={regionCreatorId}
          />
        )
      })}
    </Stack>
  )
}
