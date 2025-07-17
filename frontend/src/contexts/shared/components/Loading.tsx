import { Container, Loader, Center } from "@mantine/core"

import { useState } from "react"

type WithLoadingType = (fn: () => Promise<void>) => Promise<void>

export const useLoading = (
  initialState: boolean = true,
): [loading: boolean, withLoading: WithLoadingType] => {
  const [loading, setLoading] = useState(initialState)

  const withLoading: WithLoadingType = async (fn: () => Promise<void>) => {
    setLoading(true)
    await fn()
    setLoading(false)
  }

  return [loading, withLoading]
}

export default function Loading() {
  return (
    <Container>
      <Center>
        <Loader />
      </Center>
    </Container>
  )
}
