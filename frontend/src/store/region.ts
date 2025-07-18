import { createSlice } from "@reduxjs/toolkit"
import type { Region } from "../api/Api"

export type RegionState = Region | null

const regionSlice = createSlice({
  name: "region",
  initialState: null as RegionState,
  reducers: {
    regionLoaded: (_state, action) => {
      return action.payload as RegionState
    },
  },
})

export const { regionLoaded } = regionSlice.actions
export default regionSlice.reducer
