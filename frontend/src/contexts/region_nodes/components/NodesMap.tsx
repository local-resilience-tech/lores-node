import { Box, Image, Text, Tooltip } from "@mantine/core"
import { IconMapPin } from "@tabler/icons-react"
import type { LatLng, RegionMap, RegionNodeDetails } from "../../../api/Api"

interface Coordinate2D {
  x: number
  y: number
}

type NodesMapProps = {
  map: RegionMap
  nodes: RegionNodeDetails[]
}

export default function NodesMap({ map, nodes }: NodesMapProps) {
  if (!map.map_data_url) {
    return <Text c="dimmed">No map image available.</Text>
  }

  const nodesWithPosition = nodes.flatMap((node) => {
    if (!node.latlng) return []
    const position = interpolatePosition(
      map.min_latlng,
      map.max_latlng,
      node.latlng,
    )
    return [{ node, position }]
  })

  return (
    <Box
      style={{ position: "relative", display: "inline-block", width: "100%" }}
    >
      <Image
        src={map.map_data_url}
        alt="Region map"
        w="100%"
        h="auto"
        radius={0}
      />
      {nodesWithPosition.map(({ node, position }) => (
        <Tooltip key={node.node_id} label={node.name ?? node.node_id} withArrow>
          <Box
            style={{
              position: "absolute",
              left: `${position.x}%`,
              top: `${position.y}%`,
              transform: "translate(-50%, -100%)",
              cursor: "default",
              lineHeight: 0,
            }}
          >
            <IconMapPin color="blue" size={32} style={{ display: "block" }} />
          </Box>
        </Tooltip>
      ))}
    </Box>
  )
}

function interpolatePosition(
  min: LatLng,
  max: LatLng,
  source: LatLng,
): Coordinate2D {
  const lngRange = max.lng - min.lng
  const latRange = max.lat - min.lat
  return {
    x: lngRange !== 0 ? ((source.lng - min.lng) / lngRange) * 100 : 50,
    y: latRange !== 0 ? ((max.lat - source.lat) / latRange) * 100 : 50,
  }
}
