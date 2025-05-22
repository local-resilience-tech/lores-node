import { Container } from "@chakra-ui/react"
import { useEffect, useState } from "react"
import NewNode, { NewNodeData } from "../components/NewNode"
import { NodeIdentity } from "../types"
import ThisNodeApi from "../api"
import { ApiResult } from "../../shared/types"
import { Loading, useLoading } from "../../shared"
import EditNode from "./EditNode"

const api = new ThisNodeApi()

const getNode = async (): Promise<NodeIdentity | null> => {
  const result = await api.show()
  if ("Ok" in result) return result.Ok
  return null
}

export default function EnsureNode() {
  const [node, setNode] = useState<NodeIdentity | null>(null)
  const [loading, withLoading] = useLoading(true)

  const updateNode = (newNode: NodeIdentity | null) => {
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
    api
      .create({ name: data.name })
      .then((result: ApiResult<NodeIdentity, any>) => {
        if ("Ok" in result) updateNode(result.Ok)
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
