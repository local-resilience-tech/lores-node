import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import type { AppRepo } from "../api/Api"

export type AppsState = AppRepo[] | null

const appReposSlice = createSlice({
  name: "app_repos",
  initialState: null as AppsState,
  reducers: {
    appReposLoaded: (_state, action: PayloadAction<AppsState>) => {
      return action.payload
    },
  },
})

export const { appReposLoaded } = appReposSlice.actions
export default appReposSlice.reducer
