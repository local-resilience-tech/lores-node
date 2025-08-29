import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { NodeDetails } from "../api/Api"

export type NodesState = NodeDetails[] | null

export type NodesMap = Map<string, NodeDetails>

export function hashById(nodes: NodeDetails[] | null): NodesMap {
  if (!nodes) return new Map<string, NodeDetails>()

  return nodes.reduce((acc, node) => {
    acc.set(node.id, node)
    return acc
  }, new Map<string, NodeDetails>())
}

export function getNodeById(
  nodes: NodeDetails[] | null,
  id: string | null | undefined
): NodeDetails | null {
  if (!nodes || !id) return null

  return nodes.find((node) => node.id === id) || null
}

export const nodesSlice = createSlice({
  name: "nodes",
  initialState: null as NodesState,
  reducers: {
    nodesLoaded: (state, action) => {
      return action.payload as NodesState
    },
    nodeUpdated: (state: NodesState, action: PayloadAction<NodeDetails>) => {
      const updatedNode = action.payload

      if (state) {
        const index = state.findIndex((node) => node.id === updatedNode.id)

        if (index !== -1) {
          state[index] = updatedNode
        } else {
          state.push(updatedNode)

          return state
        }
      } else {
        return [updatedNode]
      }
    },
  },
})

export const { nodesLoaded, nodeUpdated } = nodesSlice.actions
export default nodesSlice.reducer
