import { AppStore } from "."
import { getApi } from "../api"
import { appReposLoaded } from "./app_repos"
import { localAppsLoaded } from "./local_apps"
import { nodesLoaded } from "./nodes"
import { regionLoaded } from "./region"
import { regionAppsLoaded } from "./region_apps"
import { thisNodeLoaded } from "./this_node"
import { meLoaded } from "./me"
import { redirect } from "react-router-dom"
import { GetCurrentNodeStewardError } from "../api/Api"

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

  if (state.region === null) loadRegion(store)
  if (state.nodes === null) loadNodes(store)
  if (state.localApps === null) loadLocalApps(store)
  if (state.thisNode === null) loadThisNode(store)
  if (state.regionApps === null) loadRegionApps(store)
  if (state.appRepos === null) loadAppRepos(store)
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

async function loadRegion(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.showRegion())
  console.log("EFFECT: fetchRegion", result)
  if (result) store.dispatch(regionLoaded(result))
}

async function loadNodes(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.listNodes())
  console.log("EFFECT: fetchNodes", result)
  if (result) store.dispatch(nodesLoaded(result))
}

async function loadAppRepos(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.listAppRepos())
  console.log("EFFECT: fetchAppRepos", result)
  if (result) store.dispatch(appReposLoaded(result))
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

async function loadThisNode(store: AppStore) {
  const result = await fetchApiData(() => getApi().publicApi.showThisNode())
  console.log("EFFECT: fetchThisNode", result)
  if (result) store.dispatch(thisNodeLoaded(result))
}

const fetchApiData = async <T>(
  apiCall: () => Promise<{ status: number; data: T }>
): Promise<T | null> => {
  const result = await apiCall()
  if (result.status >= 200 && result.status < 300) return result.data
  return null
}
