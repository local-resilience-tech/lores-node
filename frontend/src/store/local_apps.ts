import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { LocalAppInstallation } from "../api/Api"
import { regionAppUpdated } from "./region_apps"

export type AppsState = LocalAppInstallation[] | null

const localAppsSlice = createSlice({
  name: "local_apps",
  initialState: null as AppsState,
  reducers: {
    localAppsLoaded: (
      _state,
      action: PayloadAction<LocalAppInstallation[]>,
    ) => {
      return action.payload
    },
  },
  extraReducers: (builder) => {
    builder.addCase(regionAppUpdated, (state, action) => {
      if (!state) return
      const { name, region_id } = action.payload
      for (const installation of state) {
        if (installation.app.name === name) {
          installation.region_id = region_id
        }
      }
    })
  },
})

export const { localAppsLoaded } = localAppsSlice.actions
export default localAppsSlice.reducer
