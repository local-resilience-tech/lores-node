import { Container, Title, Text, Stack } from "@mantine/core"
import { RegionNodeStatus } from "../../../api/Api"
import { useAppSelector } from "../../../store"
import { myActiveRegionNode } from "../../../store/my_regions"
import { Outlet } from "react-router-dom"
import { ActionButton, actionSuccess, actionFailure } from "../../../components"
import { getApi } from "../../../api"
import { IfNodeSteward } from "../../auth/node_steward_auth"

function RegionNotJoined({
  region_id,
}: {
  region_id: string | null | undefined
}) {
  const onForgetRegion = () => {
    if (!region_id) return Promise.reject(new Error("No region_id provided"))

    return getApi()
      .nodeStewardApi.forgetRegion({ region_id })
      .then(() => actionSuccess())
      .catch((error) => actionFailure(error))
  }

  return (
    <Container>
      <Title order={1} mb="md">
        Region not joined
      </Title>
      <Stack align="flex-start">
        <Text>
          You have not joined this region. Please join the region to view its
          details.
        </Text>
        <IfNodeSteward>
          {!region_id ? (
            <Text color="red">
              No region ID provided. You cannot forget this region.
            </Text>
          ) : (
            <ActionButton
              variant="outline"
              color="red"
              onClick={onForgetRegion}
            >
              Forget region
            </ActionButton>
          )}
        </IfNodeSteward>
      </Stack>
    </Container>
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
  const activeRegionId = useAppSelector(
    (state) => state.my_regions.activeRegionId,
  )
  const currentStatus = useAppSelector(
    (state) =>
      myActiveRegionNode(state.my_regions, state.network?.node.id)?.status ??
      null,
  )

  if (currentStatus === null) {
    return <RegionNotJoined region_id={activeRegionId} />
  }

  switch (currentStatus) {
    case RegionNodeStatus.RequestedToJoin:
      return <RequestPending />
    case RegionNodeStatus.Member:
      return <Outlet />
  }
}
