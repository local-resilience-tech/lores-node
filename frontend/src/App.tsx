import {
  LoaderFunction,
  Navigate,
  RouterProvider,
  createBrowserRouter,
} from "react-router-dom"
import { Layout } from "./pages"
import {
  EditRegionNode,
  ManageStatus,
  ThisRegionNode,
} from "./contexts/this_region_node"
import { EventLog } from "./contexts/this_p2panda_node"
import { ShowLocalApp, LocalApps, RegionApps } from "./contexts/apps"
import { Nodes } from "./contexts/region_nodes"
import { MantineProvider } from "@mantine/core"
import { ModalsProvider } from "@mantine/modals"
import { Notifications } from "@mantine/notifications"
import { Provider as ReduxProvider } from "react-redux"
import { theme } from "./mantine-theme"
import store, { AppStore, loadInitialData } from "./store"
import { Stacks } from "./contexts/stacks"
import { AdminLogin, AuthLayout, SetupAdmin } from "./contexts/auth/admin_auth"
import { AdminLayout, NewNodeSteward, AllNodeStewards } from "./contexts/admin"
import {
  NodeStewardLogin,
  NodeStewardSetPassword,
  RequireNodeSteward,
} from "./contexts/auth/node_steward_auth"

// Import styles of packages that you've installed.
// All packages except `@mantine/hooks` require styles imports
import "@mantine/core/styles.css"
import "@mantine/notifications/styles.css"
import { P2PandaNode } from "./contexts/network"
import { SetActiveRegion, SetupRegion } from "./contexts/regions"
import EnsureJoinedRegion from "./contexts/regions/pages/EnsureJoinedRegion"

function withStore(
  func: (store: AppStore) => any,
  store: AppStore,
): LoaderFunction<any> {
  const wrappedFunc: LoaderFunction<any> = async () => {
    return func(store)
  }
  return wrappedFunc
}

const router = createBrowserRouter([
  {
    path: "/auth",
    element: <AuthLayout />,
    children: [
      {
        path: "admin",
        children: [
          { path: "setup", element: <SetupAdmin /> },
          { path: "login", element: <AdminLogin /> },
        ],
      },
      {
        path: "node_steward",
        children: [
          { path: "login", element: <NodeStewardLogin /> },
          { path: "set_password", element: <NodeStewardSetPassword /> },
        ],
      },
    ],
  },
  {
    path: "/admin",
    element: <AdminLayout />,
    children: [
      {
        path: "node_stewards",
        children: [
          { path: "", element: <AllNodeStewards /> },
          { path: "new", element: <NewNodeSteward /> },
        ],
      },
    ],
  },
  {
    path: "/setup",
    element: <Navigate to="/auth/admin/setup" replace />,
  },
  {
    path: "/",
    element: <Layout />,
    loader: withStore(loadInitialData, store),
    children: [
      { path: "", element: <Navigate to="/node/apps" replace /> },
      {
        path: "this_region_node",
        children: [
          {
            path: "status",
            element: (
              <RequireNodeSteward>
                <ManageStatus />
              </RequireNodeSteward>
            ),
          },
        ],
      },
      {
        path: "node",
        children: [
          {
            path: "apps",
            children: [
              { path: "", element: <LocalApps /> },
              { path: "app/:appName", element: <ShowLocalApp /> },
            ],
          },
        ],
      },
      {
        path: "regions",
        children: [
          {
            path: "setup",
            element: (
              <RequireNodeSteward>
                <SetupRegion />
              </RequireNodeSteward>
            ),
          },
          {
            path: ":regionSlug",
            children: [
              { path: "", element: <Navigate to="nodes" replace /> },
              {
                path: "node",
                children: [
                  { path: "", element: <ThisRegionNode /> },
                  {
                    path: "edit",
                    element: (
                      <RequireNodeSteward>
                        <EditRegionNode />
                      </RequireNodeSteward>
                    ),
                  },
                ],
              },
              { path: "nodes", element: <Nodes /> },
              { path: "apps", element: <RegionApps /> },
            ],
            element: (
              <SetActiveRegion>
                <EnsureJoinedRegion />
              </SetActiveRegion>
            ),
          },
        ],
      },
      {
        path: "network",
        children: [{ path: "node", element: <P2PandaNode /> }],
      },
      {
        path: "debug",
        children: [
          { path: "event_log", element: <EventLog /> },
          { path: "stacks", element: <Stacks /> },
        ],
      },
    ],
  },
])

function App() {
  return (
    <MantineProvider defaultColorScheme="dark" theme={theme}>
      <ModalsProvider>
        <Notifications />
        <ReduxProvider store={store}>
          <RouterProvider router={router} />
        </ReduxProvider>
      </ModalsProvider>
    </MantineProvider>
  )
}

export default App
