import { AppStore } from "."
import { getApi } from "../api"
import { localAppsLoaded } from "./local_apps"
import { regionsLoaded } from "./my_regions"
import { regionAppsLoaded } from "./region_apps"
import { meLoaded } from "./me"
import { redirect } from "react-router-dom"
import { GetCurrentNodeStewardError } from "../api/Api"
import { networkLoaded } from "./network"

export async function loadInitialData(store: AppStore) {
  const state = store.getState()

  try {
    await getApi()
      .authApi.getCurrentUser()
      .then((result) => {
        console.log("EFFECT: fetchUser", result)
        if (result) store.dispatch(meLoaded(result.data))
      })
      .catch((error) => {
        console.error("Error fetching current user:", error)
        if (error.status === 500) {
          return Promise.reject(error.response.data)
        }
      })
  } catch (error_type: any) {
    switch (error_type as GetCurrentNodeStewardError) {
      case "AdminNotFound":
        throw redirect("/setup")
      default:
        console.error("Unknown error:", error_type)
    }
  }

  if (state.network === null) loadNetwork(store)
  if (state.my_regions.all === null) loadRegions(store)
  if (state.localApps === null) loadLocalApps(store)
  if (state.regionApps === null) loadRegionApps(store)
}

async function loadUser(store: AppStore) {
  getApi()
    .authApi.getCurrentUser()
    .then((result) => {
      console.log("EFFECT: fetchUser", result)
      if (result) store.dispatch(meLoaded(result))
    })
    .catch((error) => {
      console.error("Error fetching current user:", error)
      return Promise.reject(redirect("/login"))
    })
}

async function loadNetwork(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.showNetwork())
  console.log("EFFECT: fetchNetwork", result)
  if (result) store.dispatch(networkLoaded(result))
}

async function loadRegions(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.listRegions())
  console.log("EFFECT: fetchRegions", result)
  if (result) store.dispatch(regionsLoaded(result))
}

async function loadLocalApps(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.listLocalApps())
  console.log("EFFECT: fetchApps", result)
  if (result) store.dispatch(localAppsLoaded(result))
}

async function loadRegionApps(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.listRegionApps())
  console.log("EFFECT: fetchRegionApps", result)
  if (result) store.dispatch(regionAppsLoaded(result))
}

const fetchApiData = async <T>(
  apiCall: () => Promise<{ status: number; data: T }>,
): Promise<T | null> => {
  const result = await apiCall()
  if (result.status >= 200 && result.status < 300) return result.data
  return null
}
