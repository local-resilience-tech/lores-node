import { Text } from "@mantine/core"
import type { RegionMap } from "../../../api/Api"

type NodesMapProps = {
  map: RegionMap
}

export default function NodesMap({ map }: NodesMapProps) {
  return (
    <Text c="dimmed">
      Node map placeholder {map.map_data_url ? "(map data available)" : ""}
    </Text>
  )
}
