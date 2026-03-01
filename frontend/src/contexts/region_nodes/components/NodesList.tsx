import { Stack } from "@mantine/core"
import { RegionNodeDetails } from "../../../api/Api"
import NodeCard from "./NodeCard"

interface NodesListProps {
  nodes: RegionNodeDetails[]
}

export default function NodesList({ nodes: nodes }: NodesListProps) {
  return (
    <Stack>
      {nodes.map((node) => (
        <NodeCard key={node.id} node={node} />
      ))}
    </Stack>
  )
}
