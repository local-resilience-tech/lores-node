import { Outlet, useParams } from "react-router-dom"
import { useAppDispatch, useAppSelector } from "../../../store"
import { activeRegion, activeRegionChanged } from "../../../store/my_regions"
import { useEffect } from "react"

export default function SetActiveRegion() {
  const currentActiveRegion = useAppSelector((state) =>
    activeRegion(state.my_regions),
  )
  const { regionSlug } = useParams<{ regionSlug: string }>()
  const slugRegion = useAppSelector((state) =>
    state.my_regions.all?.find((r) => r.slug === regionSlug),
  )
  const dispatch = useAppDispatch()

  useEffect(() => {
    if (slugRegion && currentActiveRegion?.id !== slugRegion?.id) {
      console.log(
        `Active region (${currentActiveRegion?.slug}) does not match URL slug (${regionSlug})`,
      )

      dispatch(activeRegionChanged(slugRegion.id))
    }
  }, [regionSlug, currentActiveRegion])

  return <Outlet />
}
