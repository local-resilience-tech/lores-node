import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { Node } from "../api/Api"

export type NodesState = Node[] | null

const nodesSlice = createSlice({
  name: "nodes",
  initialState: null as NodesState,
  reducers: {
    nodesLoaded: (state, action) => {
      return action.payload as NodesState
    },
    nodeUpdated: (state: NodesState, action: PayloadAction<Node>) => {
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
