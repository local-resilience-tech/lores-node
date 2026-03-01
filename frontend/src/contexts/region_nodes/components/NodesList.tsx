import { Stack } from "@mantine/core"
import { RegionNodeDetails, RegionNodeStatus } from "../../../api/Api"
import NodeCard from "./NodeCard"
import NodeJoinRequestCard from "./NodeJoinRequestCard"

interface NodesListProps {
  nodes: RegionNodeDetails[]
}

export default function NodesList({ nodes: nodes }: NodesListProps) {
  return (
    <Stack>
      {nodes.map((node) => {
        if (node.status == RegionNodeStatus.RequestedToJoin) {
          return <NodeJoinRequestCard key={node.id} node={node} />
        }
        return <NodeCard key={node.id} node={node} />
      })}
    </Stack>
  )
}
