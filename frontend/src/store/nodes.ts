import { createSlice } from "@reduxjs/toolkit"
import type { Node } from "../api/Api"

export type NodesState = Node[]

const nodesSlice = createSlice({
  name: "nodes",
  initialState: {} as NodesState,
  reducers: {},
})

export const {} = nodesSlice.actions
export default nodesSlice.reducer
