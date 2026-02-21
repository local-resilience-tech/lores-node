import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { Region } from "../api/Api"

export type RegionState = Region[] | null

const regionsSlice = createSlice({
  name: "regions",
  initialState: null as RegionState,
  reducers: {
    regionsLoaded: (_state, action: PayloadAction<Region[]>) => {
      return action.payload as RegionState
    },
    joinedRegion: (state, action: PayloadAction<Region>) => {
      const region = action.payload
      const existingRegion = state?.find((r) => r.id === region.id)

      if (!existingRegion && state !== null) state.push(region)

      return state
    },
  },
})

export const { regionsLoaded, joinedRegion } = regionsSlice.actions
export default regionsSlice.reducer
