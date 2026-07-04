import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { LocalAppInstallation } from "../api/Api"

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
})

export const { localAppsLoaded } = localAppsSlice.actions
export default localAppsSlice.reducer
