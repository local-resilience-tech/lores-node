import { Container } from "@chakra-ui/react"
import { useEffect, useState } from "react"
import NewNode, { NewNodeData } from "../components/NewNode"
import ThisNodeApi from "../api"
import { Loading, useLoading } from "../../shared"
import EditNode from "./EditNode"
import { getApi } from "../../../api"
import type { Node } from "../../../api/Api"

const api = new ThisNodeApi()

const getNode = async (): Promise<Node | null> => {
  const result = await getApi().api.showThisNode()

  if (result.status !== 200) {
    console.error("Failed to fetch node identity", result)
    return null
  }

  return result.data
}

export default function EnsureNode() {
  const [node, setNode] = useState<Node | null>(null)
  const [loading, withLoading] = useLoading(true)

  const updateNode = (newNode: Node | null) => {
    console.log("Updating node", newNode)
    setNode(newNode)
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
    <Container maxWidth={"2xl"}>
      {node == null && <NewNode onSubmitNewNode={onSubmitNewNode} />}
      {node != null && <EditNode node={node} />}
    </Container>
  )
}
