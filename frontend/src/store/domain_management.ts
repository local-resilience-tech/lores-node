import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import { DomainProvider, RegisteredDomainProviderDetail } from "../api/Api"

export type DomainManagementState = {
  registeredDomainDetails: RegisteredDomainProviderDetail[] | null
  availableDomainProviders: DomainProvider[] | null
}

const domainManagementSlice = createSlice({
  name: "domain_management",
  initialState: { registeredDomainDetails: null, availableDomainProviders: null } as DomainManagementState,
  reducers: {
    availableProvidersLoaded: (state, action: PayloadAction<DomainProvider[]>) => {
      state.availableDomainProviders = action.payload

      return state
    },
    registeredDomainsLoaded: (state, action: PayloadAction<RegisteredDomainProviderDetail[]>) => {
      state.registeredDomainDetails = action.payload

      return state
    },
    registeredDomainDeleted: (state, action: PayloadAction<string>) => {
      const registrationId = action.payload

      if (!state.registeredDomainDetails) {
        return
      }

      state.registeredDomainDetails = state.registeredDomainDetails.filter((domain) => domain.registration_id !== registrationId)

      return state
    },
    registeredDomainAdded: (state, action: PayloadAction<RegisteredDomainProviderDetail>) => {
      const registeredDomain = action.payload

      if (!state.registeredDomainDetails) {
        state.registeredDomainDetails = [registeredDomain]
        return state
      }

      if (!state.registeredDomainDetails.find((domain) => domain.registration_id === registeredDomain.registration_id)) {
        state.registeredDomainDetails.push(registeredDomain)
      }

      return state
    },
    registeredDomainUpdated: (state, action: PayloadAction<RegisteredDomainProviderDetail>) => {
      const registeredDomain = action.payload

      if (!state.registeredDomainDetails) {
        return
      }

      const registeredDomainIdx = state.registeredDomainDetails.findIndex(
        (domain) => domain.registration_id === registeredDomain.registration_id,
      )

      if (registeredDomainIdx === -1) {
        state.registeredDomainDetails.push(registeredDomain)
      } else {
        state.registeredDomainDetails[registeredDomainIdx] = registeredDomain
      }

      return state
    },
  },
})

export const findDomainProviderDetail = (
  state: DomainManagementState,
  domainRegistrationId: string | undefined,
): RegisteredDomainProviderDetail | undefined =>
  state?.registeredDomainDetails?.find((domain) => domain.registration_id === domainRegistrationId)

export const {
  availableProvidersLoaded,
  registeredDomainsLoaded,
  registeredDomainDeleted,
  registeredDomainAdded,
  registeredDomainUpdated,
} = domainManagementSlice.actions
export default domainManagementSlice.reducer
