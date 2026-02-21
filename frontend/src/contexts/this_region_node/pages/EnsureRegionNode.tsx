import { Container } from "@mantine/core"
import { useEffect } from "react"
import NewNode, { NewNodeData } from "../components/NewNode"
import { Loading, useLoading } from "../../shared"
import { getApi } from "../../../api"
import type { RegionNode } from "../../../api/Api"
import { useAppDispatch, useAppSelector } from "../../../store"
import { thisRegionNodeLoaded } from "../../../store/this_region_node"
import { Outlet } from "react-router"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"

const getNode = async (): Promise<RegionNode | null> => {
  const result = await getApi().publicApi.showThisRegionNode()

  if (result.status !== 200) {
    console.error("Failed to fetch node identity", result)
    return null
  }

  return result.data
}

export default function EnsureRegionNode() {
  const node = useAppSelector((state) => state.thisRegionNode)
  const dispatch = useAppDispatch()
  const [loading, withLoading] = useLoading(false)

  const updateNode = (newNode: RegionNode | null) => {
    console.log("Updating node", newNode)
    dispatch(thisRegionNodeLoaded(newNode))
  }

  const fetchNode = async () => {
    withLoading(async () => {
      console.log("EFFECT: fetchNode")
      const newNode = await getNode()
      updateNode(newNode)
    })
  }

  useEffect(() => {
    if (node == null) fetchNode()
  }, [])

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
