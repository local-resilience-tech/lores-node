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
    registeredDomainUpdated: (state, action: PayloadAction<RegisteredDomainDetail>) => {
      const registeredDomain = action.payload

      if (!state) {
        state = [registeredDomain]
        return state
      }

      const registeredDomainIdx = state.findIndex((domain) => domain.registration_id === registeredDomain.registration_id)

      if (registeredDomainIdx === -1) {
        state.push(registeredDomain)
      } else {
        state[registeredDomainIdx] = registeredDomain
      }

      return state
    },
  },
})

export const findDomainProviderDetail = (
  state: RegisteredDomainDetailState,
  domainRegistrationId: string | undefined,
): RegisteredDomainDetail | undefined => state?.find((domain) => domain.registration_id === domainRegistrationId)

export const { registeredDomainsLoaded, registeredDomainDeleted, registeredDomainAdded, registeredDomainUpdated } =
  domainManagementSlice.actions
export default domainManagementSlice.reducer
