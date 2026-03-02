import { createSlice, PayloadAction, WritableDraft } from "@reduxjs/toolkit"
import type { Region, RegionNodeDetails, RegionWithNodes } from "../api/Api"

export type MyRegionState = {
  activeRegionId?: string | null
  all: RegionWithNodes[] | null
}

export type NodesMap = Map<number, RegionNodeDetails>

export function hashById(nodes: RegionNodeDetails[] | null): NodesMap {
  if (!nodes) return new Map<number, RegionNodeDetails>()

  return nodes.reduce((acc, node) => {
    acc.set(node.id, node)
    return acc
  }, new Map<number, RegionNodeDetails>())
}

const regionsSlice = createSlice({
  name: "my_regions",
  initialState: { activeRegionId: null, all: null } as MyRegionState,
  reducers: {
    regionsLoaded: (state, action: PayloadAction<RegionWithNodes[]>) => {
      const regions = action.payload

      state.all = regions
      state.activeRegionId = regions.length > 0 ? regions[0].region.id : null

      return ensureRegionSlugs(state)
    },
    nodeJoinedRegion: (state, action: PayloadAction<RegionWithNodes>) => {
      const region = action.payload
      const existingRegion = state.all?.find(
        (r) => r.region.id === region.region.id,
      )

      if (!state.all) state.all = []

      if (!existingRegion) {
        state.all.push(region)
        if (!state.activeRegionId) state.activeRegionId = region.region.id
      } else {
        const index = state.all.findIndex(
          (r) => r.region.id === region.region.id,
        )
        state.all[index] = region
      }

      return ensureRegionSlugs(state)
    },
    activeRegionChanged: (state, action: PayloadAction<string | null>) => {
      const newRegionId = action.payload

      if (newRegionId !== null && newRegionId != "") {
        const regionExists = state.all?.some((r) => r.region.id === newRegionId)

        if (regionExists) {
          state.activeRegionId = newRegionId
        } else {
          console.warn(
            `Attempted to set active region to ${newRegionId}, but it does not exist in the regions list.`,
          )
        }
      }

      return state
    },
    regionNodeUpdated: (state, action: PayloadAction<RegionNodeDetails>) => {
      const updatedNode = action.payload
      const regionIndex = findRegionIndex(state, updatedNode.region_id)

      if (regionIndex === -1) {
        console.warn(
          `Received node update for region ID ${updatedNode.region_id}, but that region is not in the state.`,
        )
        return state
      }

      const region = state.all![regionIndex]
      const nodeIndex = region.nodes.findIndex(
        (n) => n.node_id === updatedNode.node_id,
      )

      if (nodeIndex === -1) {
        // Node not found, add it to the list
        region.nodes.push(updatedNode)
      } else {
        // Node found, update it
        region.nodes[nodeIndex] = updatedNode
      }

      return state
    },
  },
})

function ensureRegionSlugs(
  state: WritableDraft<MyRegionState>,
): WritableDraft<MyRegionState> {
  if (!state.all) return state

  state.all.forEach((regionWithNodes) => {
    if (!regionWithNodes.region.slug) {
      regionWithNodes.region.slug = regionWithNodes.region.id
    }
  })

  return state
}

export function activeRegionWithNodes(
  state: MyRegionState,
): RegionWithNodes | null {
  if (!state.activeRegionId || !state.all) return null
  return state.all.find((r) => r.region.id === state.activeRegionId) ?? null
}

export function activeRegion(state: MyRegionState): Region | null {
  const regionWithNodes = activeRegionWithNodes(state)
  return regionWithNodes ? regionWithNodes.region : null
}

export function myActiveRegionNode(
  state: MyRegionState,
  myNodeId: string | null | undefined,
): RegionNodeDetails | null {
  if (!myNodeId) return null

  const regionWithNodes = activeRegionWithNodes(state)
  if (!regionWithNodes) return null

  const myNode = regionWithNodes.nodes.find((n) => n.node_id === myNodeId)
  return myNode ?? null
}

export function nodeName(node: RegionNodeDetails): string {
  return node.name || `unnamed ${node.node_id.slice(0, 8)}`
}

function findRegionIndex(state: MyRegionState, regionId: string): number {
  return state.all?.findIndex((r) => r.region.id === regionId) ?? -1
}

export const {
  regionsLoaded,
  nodeJoinedRegion,
  activeRegionChanged,
  regionNodeUpdated,
} = regionsSlice.actions
export default regionsSlice.reducer
