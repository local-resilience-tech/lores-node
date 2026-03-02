import { createSlice } from "@reduxjs/toolkit"
import type { Network } from "../api/Api"

export type NetworkState = Network | null

const networkSlice = createSlice({
  name: "network",
  initialState: null as NetworkState,
  reducers: {
    networkLoaded: (_state, action) => {
      return action.payload as NetworkState
    },
  },
})

export const { networkLoaded } = networkSlice.actions
export default networkSlice.reducer
