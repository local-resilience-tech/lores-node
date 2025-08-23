import { Api } from "./Api"

function getApiUrl(): string {
  const apiUrl = import.meta.env.VITE_API_HOST || "/"
  return apiUrl
}

export function getApi() {
  const apiUrl = getApiUrl()
  return new Api({
    baseURL: apiUrl,
    withCredentials: true,
  })
}

export function getSocketUrl(): string {
  const apiUrl = getApiUrl()

  const withWsProtocol = apiUrl.replace(/^https?/, "ws")
  const ensureTrailingSlash = withWsProtocol.endsWith("/")
    ? withWsProtocol
    : `${withWsProtocol}/`

  return ensureTrailingSlash + "ws"
}
