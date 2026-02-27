import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { RegionNodeDetails } from "../api/Api"

export type NodesState = RegionNodeDetails[] | null

export type NodesMap = Map<number, RegionNodeDetails>

export function hashById(nodes: RegionNodeDetails[] | null): NodesMap {
  if (!nodes) return new Map<number, RegionNodeDetails>()

  return nodes.reduce((acc, node) => {
    acc.set(node.id, node)
    return acc
  }, new Map<number, RegionNodeDetails>())
}

export function getNodeById(
  nodes: RegionNodeDetails[] | null,
  id: number | null | undefined,
): RegionNodeDetails | null {
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
    nodeUpdated: (
      state: NodesState,
      action: PayloadAction<RegionNodeDetails>,
    ) => {
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
