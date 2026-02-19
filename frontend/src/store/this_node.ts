import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { RegionNode, RegionNodeDetails } from "../api/Api"

export type ThisNodeState = RegionNode | null

const thisNodeSlice = createSlice({
  name: "this_node",
  initialState: null as ThisNodeState,
  reducers: {
    thisNodeLoaded: (state, action) => {
      return action.payload as RegionNode
    },
  },
  extraReducers: (builder) => {
    builder.addMatcher(
      (action) => action.type === "nodes/nodeUpdated",
      (state, { payload }: PayloadAction<RegionNodeDetails>) => {
        if (state && state.id === payload.id) {
          return { ...state, ...payload }
        }
        return state
      },
    )
  },
})

export const { thisNodeLoaded } = thisNodeSlice.actions
export default thisNodeSlice.reducer
