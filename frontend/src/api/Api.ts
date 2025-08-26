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

export enum UpgradeLocalAppError {
  AppNotFound = "AppNotFound",
  InUse = "InUse",
  ServerError = "ServerError",
}

export enum LocalAppInstallStatus {
  Installed = "Installed",
  StackDeployed = "StackDeployed",
}

export enum InstallLocalAppError {
  InUse = "InUse",
  ServerError = "ServerError",
}

export interface AdminCredentials {
  password: string;
}

export interface AppDefinition {
  latest_version?: string | null;
  name: string;
  versions: string[];
}

export interface AppInstallation {
  app_name: string;
  node_id: string;
  version: string;
}

export interface AppReference {
  app_name: string;
}

export interface AppRepo {
  apps: AppDefinition[];
  git_url: string;
  name: string;
}

export interface AppRepoAppReference {
  app_name: string;
  repo_name: string;
  version: string;
}

export interface AppRepoSource {
  git_url: string;
  name: string;
}

export interface BootstrapNodeData {
  network_name: string;
  node_id?: string | null;
}

export type ClientEvent =
  | {
      NodeUpdated: NodeDetails;
    }
  | {
      RegionAppUpdated: RegionAppWithInstallations;
    }
  | {
      AppRepoUpdated: AppRepo;
    }
  | {
      LocalAppUpdated: LocalApp;
    };

export interface CreateNodeDetails {
  name: string;
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

export interface IrohNodeAddr {
  direct_addresses: string[];
  node_id: string;
  relay_url?: string | null;
}

export interface LocalApp {
  name: string;
  repo_name?: string | null;
  status: LocalAppInstallStatus;
  version: string;
}

export interface LocalAppUpgradeParams {
  target_version: string;
}

export interface LogCount {
  node_id: string;
  /** @format int64 */
  total: number;
}

export interface Node {
  id: string;
  name: string;
}

export interface NodeDetails {
  id: string;
  name: string;
  public_ipv4?: string | null;
  state?: string | null;
  status_text?: string | null;
}

export interface NodeStatusData {
  state?: string | null;
  text?: string | null;
}

export interface NodeSteward {
  active: boolean;
  id: string;
  name: string;
}

export interface NodeStewardCreationData {
  name: string;
}

export interface NodeStewardCreationResult {
  node_steward: NodeSteward;
  password_reset_token: string;
}

export interface P2PandaLogCounts {
  counts: LogCount[];
}

export interface P2PandaNodeDetails {
  iroh_node_addr: IrohNodeAddr;
  panda_node_id: string;
  peers: PandaNodeAddress[];
}

export interface PandaNodeAddress {
  direct_addresses: string[];
  public_key: string;
  relay_url?: string | null;
}

export interface Region {
  network_id: string;
}

export interface RegionAppWithInstallations {
  installations: AppInstallation[];
  name: string;
}

export interface UpdateNodeDetails {
  name: string;
  public_ipv4: string;
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
 * @version 0.11.3
 * @license
 */
export class Api<
  SecurityDataType extends unknown,
> extends HttpClient<SecurityDataType> {
  adminApi = {
    /**
     * No description
     *
     * @name ShowThisNode
     * @request GET:/admin_api/node
     */
    showThisNode: (params: RequestParams = {}) =>
      this.request<null | Node, string>({
        path: `/admin_api/node`,
        method: "GET",
        format: "json",
        ...params,
      }),

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
      this.request<UserRef, string>({
        path: `/auth_api/admin/login`,
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
     * @name ListAppRepos
     * @request GET:/public_api/app_repos
     */
    listAppRepos: (params: RequestParams = {}) =>
      this.request<AppRepo[], any>({
        path: `/public_api/app_repos`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name CreateAppRepo
     * @request POST:/public_api/app_repos
     */
    createAppRepo: (data: AppRepoSource, params: RequestParams = {}) =>
      this.request<any, any>({
        path: `/public_api/app_repos`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ReloadAppRepo
     * @request GET:/public_api/app_repos/reload/{repo_name}
     */
    reloadAppRepo: (repoName: string, params: RequestParams = {}) =>
      this.request<AppRepo, any>({
        path: `/public_api/app_repos/reload/${repoName}`,
        method: "GET",
        format: "json",
        ...params,
      }),

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
     * @name DeployLocalApp
     * @request POST:/public_api/local_apps/app/{app_name}/deploy
     */
    deployLocalApp: (appName: string, params: RequestParams = {}) =>
      this.request<any, string>({
        path: `/public_api/local_apps/app/${appName}/deploy`,
        method: "POST",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name RemoveDeploymentOfLocalApp
     * @request DELETE:/public_api/local_apps/app/{app_name}/deploy
     */
    removeDeploymentOfLocalApp: (appName: string, params: RequestParams = {}) =>
      this.request<any, string>({
        path: `/public_api/local_apps/app/${appName}/deploy`,
        method: "DELETE",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name UpgradeLocalApp
     * @request POST:/public_api/local_apps/app/{app_name}/upgrade
     */
    upgradeLocalApp: (
      appName: string,
      data: LocalAppUpgradeParams,
      params: RequestParams = {},
    ) =>
      this.request<any, UpgradeLocalAppError>({
        path: `/public_api/local_apps/app/${appName}/upgrade`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name InstallAppDefinition
     * @request POST:/public_api/local_apps/definitions
     */
    installAppDefinition: (
      data: AppRepoAppReference,
      params: RequestParams = {},
    ) =>
      this.request<any, InstallLocalAppError>({
        path: `/public_api/local_apps/definitions`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name RegisterApp
     * @request POST:/public_api/local_apps/register
     */
    registerApp: (data: AppReference, params: RequestParams = {}) =>
      this.request<any, any>({
        path: `/public_api/local_apps/register`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name ListNodes
     * @request GET:/public_api/nodes
     */
    listNodes: (params: RequestParams = {}) =>
      this.request<NodeDetails[], any>({
        path: `/public_api/nodes`,
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
     * @name ShowThisNode
     * @request GET:/public_api/this_node
     */
    showThisNode: (params: RequestParams = {}) =>
      this.request<null | Node, string>({
        path: `/public_api/this_node`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name UpdateThisNode
     * @request PUT:/public_api/this_node
     */
    updateThisNode: (data: UpdateNodeDetails, params: RequestParams = {}) =>
      this.request<Node, string>({
        path: `/public_api/this_node`,
        method: "PUT",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name CreateThisNode
     * @request POST:/public_api/this_node
     */
    createThisNode: (data: CreateNodeDetails, params: RequestParams = {}) =>
      this.request<Node, string>({
        path: `/public_api/this_node`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name PostNodeStatus
     * @request POST:/public_api/this_node/status
     */
    postNodeStatus: (data: NodeStatusData, params: RequestParams = {}) =>
      this.request<any, string>({
        path: `/public_api/this_node/status`,
        method: "POST",
        body: data,
        type: ContentType.Json,
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

    /**
     * No description
     *
     * @name ShowRegion
     * @request GET:/public_api/this_region
     */
    showRegion: (params: RequestParams = {}) =>
      this.request<null | Region, any>({
        path: `/public_api/this_region`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @name Bootstrap
     * @request POST:/public_api/this_region/bootstrap
     */
    bootstrap: (data: BootstrapNodeData, params: RequestParams = {}) =>
      this.request<any, string>({
        path: `/public_api/this_region/bootstrap`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        format: "json",
        ...params,
      }),
  };
}
