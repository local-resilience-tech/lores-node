import { createContext } from "react"
import { Region } from "../../api/Api"

export const RegionContext = createContext<Region | null>(null)
