import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { LocalApp, LocalAppInstallation } from "../api/Api"

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
    localAppCreated: (state, action: PayloadAction<LocalApp>) => {
      const installation: LocalAppInstallation = {
        app: action.payload,
        region_id: null,
      }
      if (!state) return [installation]
      state.push(installation)
    },
    localAppUpdated: (state, action: PayloadAction<LocalApp>) => {
      if (!state) return
      const idx = state.findIndex(
        (i) =>
          i.app.name === action.payload.name &&
          i.app.instance_id === action.payload.instance_id,
      )
      if (idx !== -1) state[idx].app = action.payload
      else state.push({ app: action.payload, region_id: null })
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

export const { localAppsLoaded, localAppCreated, localAppUpdated } =
  localAppsSlice.actions
export default localAppsSlice.reducer
