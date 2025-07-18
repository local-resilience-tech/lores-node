import { createSlice } from "@reduxjs/toolkit"
import type { Node } from "../api/Api"

export type NodesState = Node[] | null

const nodesSlice = createSlice({
  name: "nodes",
  initialState: null as NodesState,
  reducers: {
    nodesLoaded: (state, action) => {
      return action.payload as NodesState
    },
  },
})

export const { nodesLoaded } = nodesSlice.actions
export default nodesSlice.reducer
