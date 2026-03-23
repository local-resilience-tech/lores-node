import { Image, Text } from "@mantine/core"
import type { RegionMap } from "../../../api/Api"

type NodesMapProps = {
  map: RegionMap
}

export default function NodesMap({ map }: NodesMapProps) {
  if (!map.map_data_url) {
    return <Text c="dimmed">No map image available.</Text>
  }

  return (
    <Image
      src={map.map_data_url}
      alt="Region map"
      w="100%"
      h="auto"
      radius={0}
    />
  )
}
