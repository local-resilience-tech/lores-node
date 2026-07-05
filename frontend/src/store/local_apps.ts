import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { LocalApp } from "../api/Api"

export type AppsState = LocalApp[] | null

const localAppsSlice = createSlice({
  name: "local_apps",
  initialState: null as AppsState,
  reducers: {
    localAppsLoaded: (_state, action: PayloadAction<LocalApp[]>) => {
      return action.payload
    },
    localAppCreated: (state, action: PayloadAction<LocalApp>) => {
      if (!state) return [action.payload]
      state.push(action.payload)
    },
    localAppUpdated: (state, action: PayloadAction<LocalApp>) => {
      if (!state) return
      const idx = state.findIndex(
        (app) =>
          app.name === action.payload.name &&
          app.instance_id === action.payload.instance_id,
      )
      if (idx !== -1) state[idx] = action.payload
      else state.push(action.payload)
    },
  },
})

export const { localAppsLoaded, localAppCreated, localAppUpdated } =
  localAppsSlice.actions
export default localAppsSlice.reducer
