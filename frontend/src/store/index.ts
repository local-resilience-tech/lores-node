import { useDispatch, useSelector, useStore } from "react-redux"
import { configureStore } from "@reduxjs/toolkit"
import regionReducer from "./region"
import nodesReducer from "./nodes"
import thisNodeReducer from "./this_node"

const store = configureStore({
  reducer: {
    region: regionReducer,
    nodes: nodesReducer,
    thisNode: thisNodeReducer,
  },
})

export type AppStore = typeof store
export type RootState = ReturnType<AppStore["getState"]>
export type AppDispatch = AppStore["dispatch"]

export const useAppDispatch = useDispatch.withTypes<AppDispatch>()
export const useAppSelector = useSelector.withTypes<RootState>()
export const useAppStore = useStore.withTypes<AppStore>()

export default store
