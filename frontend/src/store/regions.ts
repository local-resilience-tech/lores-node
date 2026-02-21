import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { Region } from "../api/Api"

export type RegionState = Region[]

const regionsSlice = createSlice({
  name: "regions",
  initialState: [] as RegionState,
  reducers: {
    regionLoaded: (_state, action: PayloadAction<Region[]>) => {
      return action.payload as RegionState
    },
  },
})

export const { regionLoaded: regionsLoaded } = regionsSlice.actions
export default regionsSlice.reducer
