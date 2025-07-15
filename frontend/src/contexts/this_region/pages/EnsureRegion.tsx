import { useEffect, useState } from "react"
import { Container } from "@chakra-ui/react"
import SetRegion from "../components/SetRegion"
import { NewRegionData } from "../components/NewRegion"
import { Outlet } from "react-router-dom"
import { RegionContext } from "../provider_contexts"
import { Loading, useLoading } from "../../shared"
import { getApi } from "../../../api"
import { Region } from "../../../api/Api"

const getRegion = async (): Promise<Region | null> => {
  const result = await getApi().api.showRegion()
  if (result.status === 200) return result.data
  return null
}

export default function EnsureRegion({
  children,
}: {
  children?: React.ReactNode
}) {
  const [regionDetails, setRegionDetails] = useState<Region | null>(null)
  const [loading, withLoading] = useLoading(true)

  const fetchRegion = async () => {
    withLoading(async () => {
      const newRegion = await getRegion()
      console.log("EFFECT: fetchRegion", newRegion)
      setRegionDetails(newRegion)
    })
  }

  const onSubmitNewRegion = (data: NewRegionData) => {
    getApi()
      .api.bootstrap({
        network_name: data.name,
        bootstrap_peer: null,
      })
      .then((result) => {
        if (result.status === 200) {
          console.log("Successfully bootstrapped", result)
          const newRegion: Region = {
            network_id: data.name,
          }
          setRegionDetails(newRegion)
        } else {
          console.log("Failed to bootstrap", result)
        }
      })
  }

  useEffect(() => {
    fetchRegion()
  }, [])

  if (loading) return <Loading />

  return (
    <Container maxWidth={"2xl"}>
      {regionDetails == null && (
        <SetRegion onSubmitNewRegion={onSubmitNewRegion} />
      )}
      <RegionContext.Provider value={regionDetails}>
        {regionDetails != null && (children || <Outlet />)}
      </RegionContext.Provider>
    </Container>
  )
}
