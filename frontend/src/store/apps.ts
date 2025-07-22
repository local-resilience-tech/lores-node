import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { App } from "../api/Api"

export type AppsState = App[] | null

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
