import { AppStore } from "."
import { getApi } from "../api"
import { App, NodeDetails, Region } from "../api/Api"
import { appsLoaded } from "./apps"
import { nodesLoaded } from "./nodes"
import { regionLoaded } from "./region"
import { thisNodeLoaded } from "./this_node"

export async function loadInitialData(store: AppStore) {
  const state = store.getState()
  if (state.region === null) loadRegion(store)
  if (state.nodes === null) loadNodes(store)
  if (state.apps === null) loadApps(store)
  if (state.thisNode === null) loadThisNode(store)
}

async function loadRegion(store: AppStore) {
  const result = await fetchApiData(() => getApi().api.showRegion())
  console.log("EFFECT: fetchRegion", result)
  if (result) store.dispatch(regionLoaded(result))
}

async function loadNodes(store: AppStore) {
  const result = await fetchApiData(() => getApi().api.showAllNodes())
  console.log("EFFECT: fetchNodes", result)
  if (result) store.dispatch(nodesLoaded(result))
}

async function loadApps(store: AppStore) {
  const result = await fetchApiData(() => getApi().api.showAllApps())
  console.log("EFFECT: fetchApps", result)
  if (result) store.dispatch(appsLoaded(result))
}

async function loadThisNode(store: AppStore) {
  const result = await fetchApiData(() => getApi().api.showThisNode())
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
