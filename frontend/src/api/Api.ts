/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

export enum RegionNodeStatus {
  RequestedToJoin = "RequestedToJoin",
  Member = "Member",
}

export enum NodeStewardStatus {
  Enabled = "Enabled",
  Disabled = "Disabled",
  Invited = "Invited",
  TokenExpired = "TokenExpired",
}

export enum NodeStewardSetPasswordError {
  InvalidId = "InvalidId",
  InvalidToken = "InvalidToken",
  TokenExpired = "TokenExpired",
  InvalidNewPassword = "InvalidNewPassword",
  InternalServerError = "InternalServerError",
}

export enum NodeStewardLoginError {
  InvalidCredentials = "InvalidCredentials",
  NoPasswordSet = "NoPasswordSet",
  AccountDisabled = "AccountDisabled",
  InternalServerError = "InternalServerError",
}

export enum GetCurrentNodeStewardError {
  InternalServerError = "InternalServerError",
  AdminNotFound = "AdminNotFound",
}

export enum AdminLoginError {
  InvalidCredentials = "InvalidCredentials",
  NoPasswordSet = "NoPasswordSet",
  InternalServerError = "InternalServerError",
}

export interface AdminCredentials {
  password: string;
}

export interface AppInstallation {
  app_name: string;
  /** @format int64 */
  region_node_id: number;
  version: string;
}

export interface AppRegionReference {
  app: LocalApp;
  region_id: string;
}

export interface ApproveJoinRequestData {
  node_id: string;
  region_id: string;
}

export interface BootstrapNodeRequest {
  node_id: string;
}

export type ClientEvent =
  | {
      NodeJoinedRegion: RegionWithNodes;
    }
  | {
      RegionNodeUpdated: RegionNodeDetails;
    }
  | {
      RegionAppUpdated: RegionAppWithInstallations;
    }
  | {
      RegionUpdated: Region;
    };

export interface CreateRegionData {
  name: string;
  node_steward_conduct_url?: string | null;
  organisation_name?: string | null;
  organisation_url?: string | null;
  slug: string;
  user_conduct_url?: string | null;
  user_privacy_url?: string | null;
}

export interface DockerService {
  current_state: string;
  current_state_duration: string;
  id: string;
  image: string;
  name: string;
  node_name: string;
}

export interface DockerStackWithServices {
  name: string;
  services: DockerService[];
}

export interface JoinRegionRequestData {
  about_your_node: string;
  about_your_stewards: string;
  agreed_node_steward_conduct_url?: string | null;
  region_id: string;
}

export interface LatLng {
  /** @format double */
  lat: number;
  /** @format double */
  lng: number;
}

export interface LocalApp {
  name: string;
  url?: null | NodeAppUrl;
  version: string;
}

export interface Network {
  name: string;
  node: NetworkNode;
}

export interface NetworkNode {
  id: string;
}

export interface NodeAppUrl {
  internet_url?: string | null;
  local_network_url?: string | null;
}

export interface NodeSteward {
  created_at: string;
  id: string;
  name: string;
  status: NodeStewardStatus;
}

export interface NodeStewardCreationData {
  name: string;
}

export interface NodeStewardCreationResult {
  node_steward: NodeSteward;
  password_reset_token: string;
}

export interface NodeStewardCredentials {
  id: string;
  password: string;
}

export interface NodeStewardSetPasswordRequest {
  id: string;
  new_password: string;
  token: string;
}

export interface NodeStewardUser {
  id: string;
  name: string;
}

export interface P2PandaLogCount {
  node_id: string;
  /** @format int64 */
  total: number;
}

export interface P2PandaLogCounts {
  counts: P2PandaLogCount[];
}

export interface P2PandaNodeDetails {
  panda_node_id: string;
}

export interface Region {
  creator_node_id?: string | null;
  id: string;
  map?: null | RegionMap;
  name?: string | null;
  node_steward_conduct_url?: string | null;
  organisation_name?: string | null;
  organisation_url?: string | null;
  slug?: string | null;
  user_conduct_url?: string | null;
  user_privacy_url?: string | null;
}

export interface RegionAppWithInstallations {
  installations: AppInstallation[];
  name: string;
}

export interface RegionMap {
  map_data_url?: string | null;
  max_latlng?: null | LatLng;
  min_latlng?: null | LatLng;
}

export interface RegionNodeDetails {
  about_your_node?: string | null;
  about_your_stewards?: string | null;
  agreed_node_steward_conduct_url?: string | null;
  domain_on_internet?: string | null;
  domain_on_local_network?: string | null;
  /** @format int64 */
  id: number;
  name?: string | null;
  node_id: string;
  public_ipv4?: string | null;
  region_id: string;
  state?: string | null;
  status?: null | RegionNodeStatus;
  status_text?: string | null;
}

export interface RegionNodeStatusData {
  state?: string | null;
  text?: string | null;
}

export interface RegionWithNodes {
  nodes: RegionNodeDetails[];
  region: Region;
}

export interface UpdateMapData {
  image_data_url: string;
  max_latlng: LatLng;
  min_latlng: LatLng;
  region_id: string;
}

export interface UpdateNodeDetails {
  domain_on_internet?: string | null;
  domain_on_local_network?: string | null;
  name: string;
  public_ipv4?: string | null;
}

export interface UserRef {
  user_id: string;
}

import type {
  AxiosInstance,
  AxiosRequestConfig,
  AxiosResponse,
  HeadersDefaults,
  ResponseType,
} from "axios";
import axios from "axios";

export type QueryParamsType = Record<string | number, any>;

export interface FullRequestParams
  extends Omit<AxiosRequestConfig, "data" | "params" | "url" | "responseType"> {
  /** set parameter to `true` for call `securityWorker` for this request */
  secure?: boolean;
  /** request path */
  path: string;
  /** content type of request body */
  type?: ContentType;
  /** query params */
  query?: QueryParamsType;
  /** format of response (i.e. response.json() -> format: "json") */
  format?: ResponseType;
  /** request body */
  body?: unknown;
}

export type RequestParams = Omit<
  FullRequestParams,
  "body" | "method" | "query" | "path"
>;

export interface ApiConfig<SecurityDataType = unknown>
  extends Omit<AxiosRequestConfig, "data" | "cancelToken"> {
  securityWorker?: (
    securityData: SecurityDataType | null,
  ) => Promise<AxiosRequestConfig | void> | AxiosRequestConfig | void;
  secure?: boolean;
  format?: ResponseType;
}

export enum ContentType {
  Json = "application/json",
  JsonApi = "application/vnd.api+json",
  FormData = "multipart/form-data",
  UrlEncoded = "application/x-www-form-urlencoded",
  Text = "text/plain",
}

export class HttpClient<SecurityDataType = unknown> {
  public instance: AxiosInstance;
  private securityData: SecurityDataType | null = null;
  private securityWorker?: ApiConfig<SecurityDataType>["securityWorker"];
  private secure?: boolean;
  private format?: ResponseType;

  constructor({
    securityWorker,
    secure,
    format,
    ...axiosConfig
  }: ApiConfig<SecurityDataType> = {}) {
    this.instance = axios.create({
      ...axiosConfig,
      baseURL: axiosConfig.baseURL || "",
    });
    this.secure = secure;
    this.format = format;
    this.securityWorker = securityWorker;
  }

  public setSecurityData = (data: SecurityDataType | null) => {
    this.securityData = data;
  };

  protected mergeRequestParams(
    params1: AxiosRequestConfig,
    params2?: AxiosRequestConfig,
  ): AxiosRequestConfig {
    const method = params1.method || (params2 && params2.method);

    return {
      ...this.instance.defaults,
      ...params1,
      ...(params2 || {}),
      headers: {
        ...((method &&
          this.instance.defaults.headers[
            method.toLowerCase() as keyof HeadersDefaults
          ]) ||
          {}),
        ...(params1.headers || {}),
        ...((params2 && params2.headers) || {}),
      },
    };
  }

  protected stringifyFormItem(formItem: unknown) {
    if (typeof formItem === "object" && formItem !== null) {
      return JSON.stringify(formItem);
    } else {
      return `${formItem}`;
    }
  }

  protected createFormData(input: Record<string, unknown>): FormData {
    if (input instanceof FormData) {
      return input;
    }
    return Object.keys(input || {}).reduce((formData, key) => {
      const property = input[key];
      const propertyContent: any[] =
        property instanceof Array ? property : [property];

      for (const formItem of propertyContent) {
        const isFileType = formItem instanceof Blob || formItem instanceof File;
        formData.append(
          key,
          isFileType ? formItem : this.stringifyFormItem(formItem),
        );
      }

      return formData;
    }, new FormData());
  }

  public request = async <T = any, _E = any>({
    secure,
    path,
    type,
    query,
    format,
    body,
    ...params
  }: FullRequestParams): Promise<AxiosResponse<T>> => {
    const secureParams =
      ((typeof secure === "boolean" ? secure : this.secure) &&
        this.securityWorker &&
        (await this.securityWorker(this.securityData))) ||
      {};
    const requestParams = this.mergeRequestParams(params, secureParams);
    const responseFormat = format || this.format || undefined;

    if (
      type === ContentType.FormData &&
      body &&
      body !== null &&
      typeof body === "object"
    ) {
      body = this.createFormData(body as Record<string, unknown>);
    }

    if (
      type === ContentType.Text &&
      body &&
      body !== null &&
      typeof body !== "string"
    ) {
      body = JSON.stringify(body);
    }

    return this.instance.request({
      ...requestParams,
      headers: {
        ...(requestParams.headers || {}),
        ...(type ? { "Content-Type": type } : {}),
      },
      params: query,
      responseType: responseFormat,
      data: body,
      url: path,
    });
  };
}

/**
 * @title lores-node
 * @version 0.15.4
 * @license
 */
export class Api<
  SecurityDataType extends unknown,
> extends HttpClient<SecurityDataType> {
  adminApi = {
    /**
     * No description
     *
     * @name ListNodeStewards
     * @request GET:/admin_api/node_stewards
     */
    listNodeStewards: (params: RequestParams = {}) =>
      this.request<NodeSteward[], any>({
        path: `/admin_api/node_stewards`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name CreateNodeSteward
     * @request POST:/admin_api/node_stewards
     */
    createNodeSteward: (
      data: NodeStewardCreationData,
      params: RequestParams = {},
    ) =>
      this.request<NodeStewardCreationResult, string>({
        path: `/admin_api/node_stewards`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name DisableNodeSteward
     * @request POST:/admin_api/node_stewards/disable/{steward_id}
     */
    disableNodeSteward: (stewardId: string, params: RequestParams = {}) =>
      this.request<NodeSteward, any>({
        path: `/admin_api/node_stewards/disable/${stewardId}`,
        method: "POST",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name EnableNodeSteward
     * @request POST:/admin_api/node_stewards/enable/{steward_id}
     */
    enableNodeSteward: (stewardId: string, params: RequestParams = {}) =>
      this.request<NodeSteward, any>({
        path: `/admin_api/node_stewards/enable/${stewardId}`,
        method: "POST",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ResetNodeStewardToken
     * @request POST:/admin_api/node_stewards/reset_token/{steward_id}
     */
    resetNodeStewardToken: (stewardId: string, params: RequestParams = {}) =>
      this.request<NodeStewardCreationResult, any>({
        path: `/admin_api/node_stewards/reset_token/${stewardId}`,
        method: "POST",
        format: "json",
        ...params,
      }),
  };
  authApi = {
    /**
     * No description
     *
     * @name HasAdminPassword
     * @request GET:/auth_api/admin
     */
    hasAdminPassword: (params: RequestParams = {}) =>
      this.request<boolean, any>({
        path: `/auth_api/admin`,
        method: "GET",
        ...params,
      }),

    /**
     * No description
     *
     * @name GenerateAdminPassword
     * @request POST:/auth_api/admin
     */
    generateAdminPassword: (params: RequestParams = {}) =>
      this.request<string, string>({
        path: `/auth_api/admin`,
        method: "POST",
        ...params,
      }),

    /**
     * No description
     *
     * @name AdminLogin
     * @request POST:/auth_api/admin/login
     */
    adminLogin: (data: AdminCredentials, params: RequestParams = {}) =>
      this.request<UserRef, AdminLoginError | string>({
        path: `/auth_api/admin/login`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name GetCurrentUser
     * @request GET:/auth_api/node_steward
     */
    getCurrentUser: (params: RequestParams = {}) =>
      this.request<null | NodeStewardUser, GetCurrentNodeStewardError>({
        path: `/auth_api/node_steward`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name NodeStewardLogin
     * @request POST:/auth_api/node_steward/login
     */
    nodeStewardLogin: (
      data: NodeStewardCredentials,
      params: RequestParams = {},
    ) =>
      this.request<UserRef, NodeStewardLoginError>({
        path: `/auth_api/node_steward/login`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name NodeStewardSetPassword
     * @request POST:/auth_api/node_steward/set_password
     */
    nodeStewardSetPassword: (
      data: NodeStewardSetPasswordRequest,
      params: RequestParams = {},
    ) =>
      this.request<any, NodeStewardSetPasswordError>({
        path: `/auth_api/node_steward/set_password`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),
  };
  nodeStewardApi = {
    /**
     * No description
     *
     * @name RegisterApp
     * @request POST:/node_steward_api/local_apps/register
     */
    registerApp: (data: AppRegionReference, params: RequestParams = {}) =>
      this.request<any, any>({
        path: `/node_steward_api/local_apps/register`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name UpdateThisRegionNode
     * @request PUT:/node_steward_api/my_region_nodes/{region_id_string}/my_node
     */
    updateThisRegionNode: (
      regionIdString: string,
      data: UpdateNodeDetails,
      params: RequestParams = {},
    ) =>
      this.request<void, string>({
        path: `/node_steward_api/my_region_nodes/${regionIdString}/my_node`,
        method: "PUT",
        body: data,
        type: ContentType.Json,
        ...params,
      }),

    /**
     * No description
     *
     * @name PostRegionNodeStatus
     * @request POST:/node_steward_api/my_region_nodes/{region_id_string}/status
     */
    postRegionNodeStatus: (
      regionIdString: string,
      data: RegionNodeStatusData,
      params: RequestParams = {},
    ) =>
      this.request<any, string>({
        path: `/node_steward_api/my_region_nodes/${regionIdString}/status`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ApproveJoinRequest
     * @request PUT:/node_steward_api/my_regions/approve_join_request
     */
    approveJoinRequest: (
      data: ApproveJoinRequestData,
      params: RequestParams = {},
    ) =>
      this.request<any, string>({
        path: `/node_steward_api/my_regions/approve_join_request`,
        method: "PUT",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name CreateRegion
     * @request POST:/node_steward_api/my_regions/create
     */
    createRegion: (data: CreateRegionData, params: RequestParams = {}) =>
      this.request<any, string>({
        path: `/node_steward_api/my_regions/create`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name JoinRegion
     * @request POST:/node_steward_api/my_regions/join
     */
    joinRegion: (data: JoinRegionRequestData, params: RequestParams = {}) =>
      this.request<any, string>({
        path: `/node_steward_api/my_regions/join`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name UpdateMap
     * @request POST:/node_steward_api/my_regions/map
     */
    updateMap: (data: UpdateMapData, params: RequestParams = {}) =>
      this.request<any, string>({
        path: `/node_steward_api/my_regions/map`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name AddBootstrapNode
     * @request POST:/node_steward_api/network/bootstrap
     */
    addBootstrapNode: (
      data: BootstrapNodeRequest,
      params: RequestParams = {},
    ) =>
      this.request<any, string>({
        path: `/node_steward_api/network/bootstrap`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),
  };
  publicApi = {
    /**
     * No description
     *
     * @name DummyEvent
     * @request GET:/public_api/dummy_event
     */
    dummyEvent: (params: RequestParams = {}) =>
      this.request<null | ClientEvent, any>({
        path: `/public_api/dummy_event`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ListLocalApps
     * @request GET:/public_api/local_apps
     */
    listLocalApps: (params: RequestParams = {}) =>
      this.request<LocalApp[], any>({
        path: `/public_api/local_apps`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ListRegions
     * @request GET:/public_api/my_regions
     */
    listRegions: (params: RequestParams = {}) =>
      this.request<RegionWithNodes[], any>({
        path: `/public_api/my_regions`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ShowNetwork
     * @request GET:/public_api/network
     */
    showNetwork: (params: RequestParams = {}) =>
      this.request<Network, string>({
        path: `/public_api/network`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ListRegionApps
     * @request GET:/public_api/region_apps
     */
    listRegionApps: (params: RequestParams = {}) =>
      this.request<RegionAppWithInstallations[], any>({
        path: `/public_api/region_apps`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ListStacks
     * @request GET:/public_api/stacks
     */
    listStacks: (params: RequestParams = {}) =>
      this.request<DockerStackWithServices[], any>({
        path: `/public_api/stacks`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ShowThisPandaNode
     * @request GET:/public_api/this_p2panda_node
     */
    showThisPandaNode: (params: RequestParams = {}) =>
      this.request<P2PandaNodeDetails, string>({
        path: `/public_api/this_p2panda_node`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name P2PandaLogCounts
     * @request GET:/public_api/this_p2panda_node/event_log
     */
    p2PandaLogCounts: (params: RequestParams = {}) =>
      this.request<P2PandaLogCounts, any>({
        path: `/public_api/this_p2panda_node/event_log`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
}
