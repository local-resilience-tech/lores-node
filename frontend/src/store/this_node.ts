import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { Node, NodeDetails } from "../api/Api"

export type ThisNodeState = Node | null

const thisNodeSlice = createSlice({
  name: "this_node",
  initialState: null as ThisNodeState,
  reducers: {
    thisNodeLoaded: (state, action) => {
      return action.payload as Node
    },
  },
  extraReducers: (builder) => {
    builder.addMatcher(
      (action) => action.type === "nodes/nodeUpdated",
      (state, { payload }: PayloadAction<NodeDetails>) => {
        if (state && state.id === payload.id) {
          return { ...state, ...payload }
        }
        return state
      }
    )
  },
})

export const { thisNodeLoaded } = thisNodeSlice.actions
export default thisNodeSlice.reducer
