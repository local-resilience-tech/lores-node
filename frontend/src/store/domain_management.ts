import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import { RegisteredDomainDetail } from "../api/Api"

export type RegisteredDomainDetailState = RegisteredDomainDetail[] | null

const domainManagementSlice = createSlice({
  name: "domain_management",
  initialState: null as RegisteredDomainDetailState,
  reducers: {
    registeredDomainsLoaded: (state, action: PayloadAction<RegisteredDomainDetail[]>) => {
      state = action.payload

      return state
    },
    registeredDomainDeleted: (state, action: PayloadAction<string>) => {
      const registrationId = action.payload

      if (state) {
        state = state.filter((domain) => domain.registration_id !== registrationId)
      }

      return state
    },
    registeredDomainAdded: (state, action: PayloadAction<RegisteredDomainDetail>) => {
      const registeredDomain = action.payload

      if (!state) {
        state = [registeredDomain]
        return state
      }

      if (!state.find((domain) => domain.registration_id === registeredDomain.registration_id)) {
        state.push(registeredDomain)
      }

      return state
    },
  },
})
export const { registeredDomainsLoaded, registeredDomainDeleted, registeredDomainAdded } = domainManagementSlice.actions
export default domainManagementSlice.reducer
