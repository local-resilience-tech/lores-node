import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { RegionNode, RegionNodeDetails } from "../api/Api"

export type ThisRegionNodeState = RegionNode | null

const thisRegionNodeSlice = createSlice({
  name: "this_region_node",
  initialState: null as ThisRegionNodeState,
  reducers: {
    thisRegionNodeLoaded: (state, action) => {
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

export const { thisRegionNodeLoaded } = thisRegionNodeSlice.actions
export default thisRegionNodeSlice.reducer
