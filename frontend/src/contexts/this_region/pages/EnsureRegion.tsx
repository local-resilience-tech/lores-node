import { useEffect, useState } from "react"
import { Container } from "@mantine/core"
import SetRegion from "../components/SetRegion"
import { Outlet } from "react-router-dom"
import { Loading, useLoading } from "../../shared"
import { getApi } from "../../../api"
import { BootstrapNodeData, Region } from "../../../api/Api"
import { regionLoaded } from "../../../store/region"
import { useAppDispatch, useAppSelector } from "../../../store"

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
  const region = useAppSelector((state) => state.region)
  const dispatch = useAppDispatch()

  const [loading, withLoading] = useLoading(true)

  const fetchRegion = async () => {
    withLoading(async () => {
      const newRegion = await getRegion()
      console.log("EFFECT: fetchRegion", newRegion)
      dispatch(regionLoaded(newRegion))
    })
  }

  const onSubmit = async (data: BootstrapNodeData) => {
    getApi()
      .api.bootstrap(data)
      .then((result) => {
        if (result.status === 200) {
          console.log("Successfully bootstrapped", result)
          const newRegion: Region = {
            network_id: data.network_name,
          }
          dispatch(regionLoaded(newRegion))
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
    <Container>
      {!region && <SetRegion onSubmit={onSubmit} />}
      {region && (children || <Outlet />)}
    </Container>
  )
}
