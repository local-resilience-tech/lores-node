import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { RegionApp } from "../api/Api"

export type AppsState = RegionApp[] | null

const regionAppsSlice = createSlice({
  name: "region_apps",
  initialState: null as AppsState,
  reducers: {
    regionAppsLoaded: (_state, action: PayloadAction<AppsState>) => {
      return action.payload
    },
  },
})

export const { regionAppsLoaded } = regionAppsSlice.actions
export default regionAppsSlice.reducer
