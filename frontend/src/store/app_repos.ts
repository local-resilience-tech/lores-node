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
    appRepoUpdated: (state, action: PayloadAction<AppRepo>) => {
      state = state || []
      const updatedRepo = action.payload
      const index = state.findIndex((repo) => repo.name === updatedRepo.name)
      if (index !== -1) {
        state[index] = updatedRepo
      } else {
        state.push(updatedRepo)
      }
      return state
    },
  },
})

export const { appReposLoaded, appRepoUpdated } = appReposSlice.actions
export default appReposSlice.reducer
