import { createSlice } from "@reduxjs/toolkit"
import type { Node } from "../api/Api"

export type ThisNodeState = Node | null

const thisNodeSlice = createSlice({
  name: "this_node",
  initialState: null as ThisNodeState,
  reducers: {
    thisNodeLoaded: (state, action) => {
      return action.payload as Node
    },
  },
})

export const { thisNodeLoaded } = thisNodeSlice.actions
export default thisNodeSlice.reducer
