import { Container, Title, Text } from "@mantine/core"
import { RegionNodeStatus } from "../../../api/Api"
import { useAppSelector } from "../../../store"
import { myActiveRegionNode } from "../../../store/my_regions"
import { Outlet } from "react-router-dom"

function RegionNotJoined() {
  return (
    <div style={{ padding: "2rem" }}>
      You have not joined this region. Please join the region to view its
      details.
    </div>
  )
}

function RequestPending() {
  return (
    <Container>
      <Title order={1} mb="md">
        Request Pending
      </Title>
      <Text>Your request to join this region is pending approval.</Text>
    </Container>
  )
}

export default function EnsureJoinedRegion() {
  const currentStatus = useAppSelector(
    (state) =>
      myActiveRegionNode(state.my_regions, state.network?.node.id)?.status ??
      null,
  )

  if (currentStatus === null) {
    return <RegionNotJoined />
  }

  switch (currentStatus) {
    case RegionNodeStatus.RequestedToJoin:
      return <RequestPending />
    case RegionNodeStatus.Member:
      return <Outlet />
  }
}
