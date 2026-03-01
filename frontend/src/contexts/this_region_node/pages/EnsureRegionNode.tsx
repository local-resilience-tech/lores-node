import { Container } from "@mantine/core"
import { useEffect } from "react"
import NewNode, { NewNodeData } from "../components/NewNode"
import { Loading, useLoading } from "../../shared"
import { getApi } from "../../../api"
import type { RegionNode } from "../../../api/Api"
import { useAppDispatch, useAppSelector } from "../../../store"
import { Outlet } from "react-router"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"
import { myActiveRegionNode } from "../../../store/my_regions"

export default function EnsureRegionNode() {
  const node = useAppSelector((state) =>
    myActiveRegionNode(state.my_regions, state.network?.node.id),
  )
  const dispatch = useAppDispatch()
  const [loading, withLoading] = useLoading(false)

  const updateNode = (newNode: RegionNode | null) => {
    console.log("Updating node", newNode)
    // dispatch(thisRegionNodeLoaded(newNode))
  }

  const onSubmitNewNode = (data: NewNodeData): Promise<ActionPromiseResult> =>
    getApi()
      .nodeStewardApi.createThisRegionNode({ name: data.name })
      .then((result) => {
        updateNode(result.data)
        return actionSuccess()
      })
      .catch(actionFailure)

  if (loading) return <Loading />

  return (
    <>
      {node == null && (
        <Container>
          <NewNode onSubmit={onSubmitNewNode} />
        </Container>
      )}
      {node != null && <Outlet />}
    </>
  )
}
