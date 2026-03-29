import { ActionIcon, Box, Image, Popover, Text, Tooltip } from "@mantine/core"
import { IconMapPinFilled, IconX } from "@tabler/icons-react"
import { useState } from "react"
import type { LatLng, RegionMap, RegionNodeDetails } from "../../../api/Api"
import { Coordinate2D } from "../utilities/coordinate_2D"
import NodeCard from "./NodeCard"

type NodesMapProps = {
  map: RegionMap
  nodes: RegionNodeDetails[]
  regionCreatorId?: string | null
}

export default function NodesMap({
  map,
  nodes,
  regionCreatorId,
}: NodesMapProps) {
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null)

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
      style={{
        position: "relative",
        display: "inline-block",
        width: "100%",
        overflow: "hidden",
      }}
      onClick={() => setSelectedNodeId(null)}
    >
      <Image
        src={map.map_data_url}
        alt="Region map"
        w="100%"
        h="auto"
        radius={0}
      />
      {nodesWithPosition.map(({ node, position }) => (
        <Popover
          key={node.node_id}
          opened={selectedNodeId === node.node_id}
          onDismiss={() => setSelectedNodeId(null)}
          withinPortal={false}
          position="top"
          offset={8}
          withArrow
          shadow="md"
          middlewares={{ flip: true, shift: { padding: 8 } }}
        >
          <Popover.Target>
            <Tooltip
              label={node.name ?? node.node_id}
              withArrow
              color="gray"
              disabled={selectedNodeId === node.node_id}
            >
              <Box
                style={{
                  position: "absolute",
                  left: `${position.x}%`,
                  top: `${position.y}%`,
                  transform: "translate(-50%, -100%)",
                  cursor: "pointer",
                  lineHeight: 0,
                }}
                onClick={(event) => {
                  event.stopPropagation()
                  setSelectedNodeId((current) =>
                    current === node.node_id ? null : node.node_id,
                  )
                }}
              >
                <IconMapPinFilled
                  color="blue"
                  size={36}
                  style={{ display: "block" }}
                />
              </Box>
            </Tooltip>
          </Popover.Target>

          <Popover.Dropdown
            onClick={(event) => event.stopPropagation()}
            p={0}
            style={{ width: "min(420px, calc(100vw - 1rem))" }}
          >
            <NodeCard
              node={node}
              isRegionCreator={regionCreatorId === node.node_id}
              rightSection={
                <ActionIcon
                  variant="subtle"
                  color="gray"
                  aria-label="Close node details"
                  onClick={(event) => {
                    event.stopPropagation()
                    setSelectedNodeId(null)
                  }}
                >
                  <IconX size={16} />
                </ActionIcon>
              }
            />
          </Popover.Dropdown>
        </Popover>
      ))}
    </Box>
  )
}

function interpolatePosition(
  min: LatLng,
  max: LatLng,
  source: LatLng,
): Coordinate2D {
  const minCoordinate = Coordinate2D.fromLatLng(min)
  const maxCoordinate = Coordinate2D.fromLatLng(max)

  return Coordinate2D.fromLatLng(source)
    .normalizeBetween(minCoordinate, maxCoordinate)
    .invertYWithinUnitRange()
    .toScreenPercent()
}
