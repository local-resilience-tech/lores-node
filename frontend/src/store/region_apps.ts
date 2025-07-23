import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { RegionAppWithInstallations } from "../api/Api"

export type AppsState = RegionAppWithInstallations[] | null

const regionAppsSlice = createSlice({
  name: "region_apps",
  initialState: null as AppsState,
  reducers: {
    regionAppsLoaded: (_state, action: PayloadAction<AppsState>) => {
      return action.payload
    },
    regionAppUpdated: (
      state,
      action: PayloadAction<RegionAppWithInstallations>
    ) => {
      const updatedApp = action.payload

      if (state) {
        const index = state.findIndex((app) => app.name === updatedApp.name)

        if (index !== -1) {
          state[index] = updatedApp
        } else {
          state.push(updatedApp)

          return state
        }
      } else {
        return [updatedApp]
      }
    },
  },
})

export const { regionAppsLoaded, regionAppUpdated } = regionAppsSlice.actions
export default regionAppsSlice.reducer
