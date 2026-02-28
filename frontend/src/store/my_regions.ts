import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { Region, RegionNodeDetails, RegionWithNodes } from "../api/Api"

export function activeRegionWithNodes(
  state: MyRegionState,
): RegionWithNodes | null {
  if (!state.activeRegionId || !state.all) return null
  return state.all.find((r) => r.region.id === state.activeRegionId) ?? null
}

export function activeRegion(state: MyRegionState): Region | null {
  const regionWithNodes = activeRegionWithNodes(state)
  return regionWithNodes ? regionWithNodes.region : null
}

export function myActiveRegionNode(
  state: MyRegionState,
  myNodeId: string | null | undefined,
): RegionNodeDetails | null {
  if (!myNodeId) return null

  const regionWithNodes = activeRegionWithNodes(state)
  if (!regionWithNodes) return null

  const myNode = regionWithNodes.nodes.find((n) => n.node_id === myNodeId)
  return myNode ?? null
}

export type MyRegionState = {
  activeRegionId?: string | null
  all: RegionWithNodes[] | null
}

const regionsSlice = createSlice({
  name: "my_regions",
  initialState: { activeRegionId: null, all: null } as MyRegionState,
  reducers: {
    regionsLoaded: (state, action: PayloadAction<RegionWithNodes[]>) => {
      const regions = action.payload

      state.all = regions
      state.activeRegionId = regions.length > 0 ? regions[0].region.id : null

      return state
    },
    joinedRegion: (state, action: PayloadAction<RegionWithNodes>) => {
      const region = action.payload
      const existingRegion = state?.all?.find(
        (r) => r.region.id === region.region.id,
      )

      if (!existingRegion && state !== null) {
        state.all?.push(region)
        if (!state.activeRegionId) state.activeRegionId = region.region.id
      }

      return state
    },
    activeRegionChanged: (state, action: PayloadAction<string | null>) => {
      const newRegionId = action.payload

      if (newRegionId !== null && newRegionId != "") {
        const regionExists = state.all?.some((r) => r.region.id === newRegionId)

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
