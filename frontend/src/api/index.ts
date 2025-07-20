import { Api } from "./Api"

function getApiUrl(): string {
  const apiUrl = import.meta.env.VITE_API_HOST || "/"
  return apiUrl
}

export function getApi() {
  const apiUrl = getApiUrl()
  return new Api({
    baseURL: apiUrl,
    withCredentials: false,
  })
}

export function getSocketUrl(): string {
  const apiUrl = getApiUrl()
  return apiUrl.replace(/^https?/, "ws") + "ws"
}
