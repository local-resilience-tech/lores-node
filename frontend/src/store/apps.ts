import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { LocalApp } from "../api/Api"

export type AppsState = LocalApp[] | null

const appsSlice = createSlice({
  name: "apps",
  initialState: null as AppsState,
  reducers: {
    appsLoaded: (_state, action: PayloadAction<AppsState>) => {
      return action.payload
    },
  },
})

export const { appsLoaded } = appsSlice.actions
export default appsSlice.reducer
