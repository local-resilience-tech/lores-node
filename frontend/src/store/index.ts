import { useDispatch, useSelector, useStore } from "react-redux"
import { configureStore } from "@reduxjs/toolkit"
import regionReducer from "./region"
import nodesReducer, { nodeUpdated } from "./nodes"
import localAppsReducer from "./local_apps"
import regionAppsReducer from "./region_apps"
import thisNodeReducer from "./this_node"
import { ClientEvent } from "../api/Api"

const store = configureStore({
  reducer: {
    region: regionReducer,
    nodes: nodesReducer,
    localApps: localAppsReducer,
    regionApps: regionAppsReducer,
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

export { loadInitialData } from "./loaders"

export async function handleClientEvent(event: ClientEvent) {
  console.log("Handling client event:", event)

  if (event.NodeUpdated) {
    store.dispatch(nodeUpdated(event.NodeUpdated))
  } else {
    console.warn("Unhandled event type:", event)
  }
}
