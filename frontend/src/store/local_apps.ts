import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { AppReference, LocalApp } from "../api/Api"

export type AppsState = LocalApp[] | null

const localAppsSlice = createSlice({
  name: "local_apps",
  initialState: null as AppsState,
  reducers: {
    localAppsLoaded: (_state, action: PayloadAction<AppsState>) => {
      return action.payload
    },
    localAppUpdated: (state, action: PayloadAction<LocalApp>) => {
      if (state === null) return state
      const updatedApp = action.payload
      const index = state.findIndex((app) => app.name === updatedApp.name)
      if (index !== -1) {
        state[index] = updatedApp
      } else {
        state.push(updatedApp)
      }
      return state
    },
    localAppDeleted: (state, action: PayloadAction<AppReference>) => {
      if (state === null) return state
      const { app_name } = action.payload
      return state.filter((app) => app.name !== app_name)
    },
  },
})

export const { localAppsLoaded, localAppUpdated, localAppDeleted } =
  localAppsSlice.actions
export default localAppsSlice.reducer
