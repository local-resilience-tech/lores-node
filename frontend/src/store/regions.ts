import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { Region } from "../api/Api"

export function activeRegion(state: RegionState): Region | null {
  if (!state.activeRegionId || !state.all) return null
  return state.all.find((r) => r.id === state.activeRegionId) ?? null
}

export type RegionState = {
  activeRegionId?: string | null
  all: Region[] | null
}

const regionsSlice = createSlice({
  name: "regions",
  initialState: { activeRegionId: null, all: null } as RegionState,
  reducers: {
    regionsLoaded: (state, action: PayloadAction<Region[]>) => {
      const regions = action.payload

      state.all = regions
      state.activeRegionId = regions.length > 0 ? regions[0].id : null

      return state
    },
    joinedRegion: (state, action: PayloadAction<Region>) => {
      const region = action.payload
      const existingRegion = state?.all?.find((r) => r.id === region.id)

      if (!existingRegion && state !== null) state.all?.push(region)

      return state
    },
    activeRegionChanged: (state, action: PayloadAction<string | null>) => {
      const newRegionId = action.payload

      if (newRegionId !== null && newRegionId != "") {
        const regionExists = state.all?.some((r) => r.id === newRegionId)

        if (regionExists) {
          state.activeRegionId = newRegionId
        } else {
          console.warn(
            `Attempted to set active region to ${newRegionId}, but it does not exist in the regions list.`,
          )
        }
      }

      return state
    },
  },
})

export const { regionsLoaded, joinedRegion, activeRegionChanged } =
  regionsSlice.actions
export default regionsSlice.reducer
