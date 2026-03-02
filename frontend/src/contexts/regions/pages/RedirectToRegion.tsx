import { Outlet, useNavigate, useParams } from "react-router-dom"
import { useAppDispatch, useAppSelector } from "../../../store"
import { activeRegion, activeRegionChanged } from "../../../store/my_regions"
import { useEffect } from "react"

interface RedirectToRegionProps {
  children?: React.ReactNode
}

export default function RedirectToRegion({ children }: RedirectToRegionProps) {
  const currentActiveRegion = useAppSelector((state) =>
    activeRegion(state.my_regions),
  )
  const firstRegion = useAppSelector(
    (state) => state.my_regions.all?.[0]?.region,
  )

  const dispatch = useAppDispatch()
  const navigate = useNavigate()

  useEffect(() => {
    if (!currentActiveRegion) {
      if (firstRegion) {
        console.log(
          `No active region set, but regions exist. Setting active region to first region (${firstRegion.slug})`,
        )
        dispatch(activeRegionChanged(firstRegion.id))
        navigate(`/regions/${firstRegion.slug}`)
      } else {
        navigate("/regions/setup")
      }
    } else {
      navigate(`/regions/${currentActiveRegion.slug}`)
    }
  }, [currentActiveRegion, firstRegion])

  return children ? children : <Outlet />
}
