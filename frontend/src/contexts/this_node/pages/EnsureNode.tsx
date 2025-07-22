import { Container } from "@mantine/core"
import { useEffect } from "react"
import NewNode, { NewNodeData } from "../components/NewNode"
import { Loading, useLoading } from "../../shared"
import { getApi } from "../../../api"
import type { Node } from "../../../api/Api"
import { useAppDispatch, useAppSelector } from "../../../store"
import { thisNodeLoaded } from "../../../store/this_node"
import { Outlet } from "react-router"

const getNode = async (): Promise<Node | null> => {
  const result = await getApi().api.showThisNode()

  if (result.status !== 200) {
    console.error("Failed to fetch node identity", result)
    return null
  }

  return result.data
}

export default function EnsureNode() {
  const node = useAppSelector((state) => state.thisNode)
  const dispatch = useAppDispatch()
  const [loading, withLoading] = useLoading(false)

  const updateNode = (newNode: Node | null) => {
    console.log("Updating node", newNode)
    dispatch(thisNodeLoaded(newNode))
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

  const onSubmitNewNode = (data: NewNodeData) => {
    getApi()
      .api.createThisNode({ name: data.name })
      .then((result) => {
        if (result.status === 201) {
          updateNode(result.data)
        } else {
          console.error("Failed to create node", result)
        }
      })
      .catch((error) => {
        console.error("Error creating node", error)
      })
  }

  if (loading) return <Loading />

  return (
    <>
      {node == null && (
        <Container>
          <NewNode onSubmitNewNode={onSubmitNewNode} />
        </Container>
      )}
      {node != null && <Outlet />}
    </>
  )
}
