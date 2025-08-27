import { createSlice } from "@reduxjs/toolkit"
import type { NodeStewardUser } from "../api/Api"

export type MeState = NodeStewardUser | null

const meSlice = createSlice({
  name: "me",
  initialState: null as MeState,
  reducers: {
    meLoaded: (_state, action) => {
      return action.payload as MeState
    },
  },
})

export const { meLoaded } = meSlice.actions
export default meSlice.reducer
