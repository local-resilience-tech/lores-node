import { useCallback, useEffect, useMemo, useState } from "react"
import { differenceInMinutes } from "date-fns"
import { useAppSelector } from "../store"
import { activeRegionNodeHeartbeats } from "../store/my_regions"

const HEARTBEAT_REFRESH = 60000 // 1 minute

export type NodeHeartbeatDisplay = {
  label: string
  color: string
}

export const useNodeHeartbeats = () => {
  const [time, setTime] = useState(() => Date.now())

  const nodeHeartbeats = useAppSelector((state) =>
    activeRegionNodeHeartbeats(state.my_regions),
  )

  const awaitingUpdateState = useMemo(
    () => ({ label: "Awaiting update", color: "gray" }),
    [],
  )

  const setHeartbeatDisplay = useCallback(
    (nodeTimestamp: number | undefined) => {
      if (nodeTimestamp === undefined) {
        return awaitingUpdateState
      }

      const minutesSinceLastHeartbeat = differenceInMinutes(time, nodeTimestamp)

      if (minutesSinceLastHeartbeat <= 5) {
        return { label: "Active", color: "green" }
      } else if (minutesSinceLastHeartbeat <= 30) {
        return { label: "Last active ~30 minutes ago", color: "yellow" }
      } else if (minutesSinceLastHeartbeat <= 60) {
        return { label: "Last active ~1 hour ago", color: "yellow" }
      } else {
        return { label: "Last active over 1 hour ago", color: "red" }
      }
    },
    [time, awaitingUpdateState],
  )

  function setInitialNodeState() {
    const map = new Map()

    nodeHeartbeats?.forEach((n) => {
      map.set(n.node_id, setHeartbeatDisplay(n.heartbeat))
    })

    return map
  }

  const [nodeState, setNodeStatus] =
    useState<Map<string, NodeHeartbeatDisplay>>(setInitialNodeState)

  function getHeartbeatDisplay(nodeId: string): NodeHeartbeatDisplay {
    return nodeState.get(nodeId) ?? awaitingUpdateState
  }

  useEffect(() => {
    nodeHeartbeats?.forEach((n) => {
      setNodeStatus(nodeState.set(n.node_id, setHeartbeatDisplay(n.heartbeat)))
    })

    // If there's no heartbeats received after refresh period we
    // recalculate the time since the last one and update the UI
    const heartbeatTimer = setInterval(() => {
      setTime(() => Date.now())
    }, HEARTBEAT_REFRESH)

    return () => clearTimeout(heartbeatTimer)
  }, [setHeartbeatDisplay, nodeHeartbeats, nodeState])

  return getHeartbeatDisplay
}
