import { useDispatch, useSelector, useStore } from "react-redux"
import { configureStore } from "@reduxjs/toolkit"
import regionReducer from "./region"
import nodesReducer, { nodeUpdated } from "./nodes"
import localAppsReducer from "./local_apps"
import regionAppsReducer, { regionAppUpdated } from "./region_apps"
import meReducer from "./me"
import thisNodeReducer from "./this_node"
import { ClientEvent } from "../api/Api"

const store = configureStore({
  reducer: {
    me: meReducer,
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

  if ("NodeUpdated" in event) {
    store.dispatch(nodeUpdated(event.NodeUpdated))
  } else if ("RegionAppUpdated" in event) {
    store.dispatch(regionAppUpdated(event.RegionAppUpdated))
  } else {
    console.warn("Unhandled event type:", event)
  }
}
