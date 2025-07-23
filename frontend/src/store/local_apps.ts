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
  },
})

export const { localAppsLoaded } = localAppsSlice.actions
export default localAppsSlice.reducer
