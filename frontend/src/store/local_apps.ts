import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { LocalApp } from "../api/Api"

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
  },
})

export const { localAppsLoaded, localAppUpdated } = localAppsSlice.actions
export default localAppsSlice.reducer
