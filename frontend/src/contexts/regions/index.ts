import { Region } from "../../api/Api"
export { RegionSelector } from "./components/RegionSelector"
export { default as SetupRegion } from "./pages/SetupRegion"
export { default as SetActiveRegion } from "./pages/SetActiveRegion"
export { default as RedirectToRegion } from "./pages/RedirectToRegion"
export { default as EnsureJoinedRegion } from "./pages/EnsureJoinedRegion"
export { default as ShowRegion } from "./pages/ShowRegion"
export { default as EditRegionMap } from "./pages/EditRegionMap"

export function regionDisplayName(region: Region): string {
  return (
    [region.name, region.slug, region.id.slice(0, 12)].filter(Boolean)[0] ||
    "Unnamed"
  )
}

export function changeRegionInPath(
  newRegionSlug: string | null | undefined,
  currentPath: string,
): string {
  if (!newRegionSlug) return currentPath

  // If path starts with /regions/<slug>, replace the slug with newRegionSlug
  const regionPathRegex = /^\/regions\/([^/]+)(\/.*)?$/
  const match = currentPath.match(regionPathRegex)
  if (match) {
    const [, , restOfPath] = match
    return `/regions/${newRegionSlug}${restOfPath || ""}`
  }
  // If path does not start with /regions/<slug>, return the current path unchanged
  return currentPath
}
