import { useDispatch, useSelector, useStore } from "react-redux"
import { configureStore } from "@reduxjs/toolkit"
import networkReducer from "./network"
import regionsReducer, { joinedRegion, regionNodeUpdated } from "./my_regions"
import localAppsReducer from "./local_apps"
import regionAppsReducer, { regionAppUpdated } from "./region_apps"
import meReducer from "./me"
import { ClientEvent } from "../api/Api"

const store = configureStore({
  reducer: {
    me: meReducer,
    network: networkReducer,
    my_regions: regionsReducer,
    localApps: localAppsReducer,
    regionApps: regionAppsReducer,
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

  if ("RegionNodeUpdated" in event) {
    store.dispatch(regionNodeUpdated(event.RegionNodeUpdated))
  } else if ("RegionAppUpdated" in event) {
    store.dispatch(regionAppUpdated(event.RegionAppUpdated))
  } else if ("JoinedRegion" in event) {
    store.dispatch(joinedRegion(event.JoinedRegion))
  } else {
    console.warn("Unhandled event type:", event)
  }
}
