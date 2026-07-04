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
