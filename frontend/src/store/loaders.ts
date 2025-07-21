import { AppStore } from "."
import { getApi } from "../api"
import { NodeDetails, Region } from "../api/Api"
import { nodesLoaded } from "./nodes"
import { regionLoaded } from "./region"

export async function loadInitialData(store: AppStore) {
  const state = store.getState()
  if (state.region === null) loadRegion(store)
  if (state.nodes === null) loadNodes(store)
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

const getRegion = async (): Promise<Region | null> => {
  const result = await getApi().api.showRegion()
  if (result.status === 200) return result.data
  return null
}

const getNodes = async (): Promise<NodeDetails[] | null> => {
  const result = await getApi().api.showRegionNodes()
  if (result.status === 200) return result.data
  return null
}
