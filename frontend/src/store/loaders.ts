import { AppStore } from "."
import { getApi } from "../api"
import { App, NodeDetails, Region } from "../api/Api"
import { appsLoaded } from "./apps"
import { nodesLoaded } from "./nodes"
import { regionLoaded } from "./region"

export async function loadInitialData(store: AppStore) {
  const state = store.getState()
  if (state.region === null) loadRegion(store)
  if (state.nodes === null) loadNodes(store)
  if (state.apps === null) loadApps(store)
}

async function loadRegion(store: AppStore) {
  const result = await getRegion()
  console.log("EFFECT: fetchRegion", result)
  if (result) store.dispatch(regionLoaded(result))
}

async function loadNodes(store: AppStore) {
  const result = await getNodes()
  console.log("EFFECT: fetchNodes", result)
  if (result) store.dispatch(nodesLoaded(result))
}

async function loadApps(store: AppStore) {
  const result = await getApps()
  console.log("EFFECT: fetchApps", result)
  if (result) store.dispatch(appsLoaded(result))
}

const fetchApiData = async <T>(
  apiCall: () => Promise<{ status: number; data: T }>
): Promise<T | null> => {
  const result = await apiCall()
  if (result.status >= 200 && result.status < 300) return result.data
  return null
}

const getRegion = async (): Promise<Region | null> => {
  return fetchApiData(() => getApi().api.showRegion())
}

const getNodes = async (): Promise<NodeDetails[] | null> => {
  return fetchApiData(() => getApi().api.showAllNodes())
}

const getApps = async (): Promise<App[] | null> => {
  return fetchApiData(() => getApi().api.showAllApps())
}
