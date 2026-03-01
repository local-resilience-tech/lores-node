import { Stack } from "@mantine/core"
import { RegionNodeDetails, RegionNodeStatus } from "../../../api/Api"
import NodeCard from "./NodeCard"
import NodeJoinRequestCard from "./NodeJoinRequestCard"

interface NodesListProps {
  nodes: RegionNodeDetails[]
  regionCreatorId?: string | null
}

export default function NodesList({ nodes, regionCreatorId }: NodesListProps) {
  let ordered_nodes = [...nodes]
  ordered_nodes = ordered_nodes.sort((a, b) => {
    if (a.status === RegionNodeStatus.RequestedToJoin) return -1
    if (b.status === RegionNodeStatus.RequestedToJoin) return 1
    return 0
  })

  return (
    <Stack>
      {ordered_nodes.map((node) => {
        if (node.status == RegionNodeStatus.RequestedToJoin) {
          return <NodeJoinRequestCard key={node.id} node={node} />
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
