import {
  LoaderFunction,
  Navigate,
  RouterProvider,
  createBrowserRouter,
} from "react-router-dom"
import { Layout } from "./pages"
import {
  EditRegionNode,
  EnsureRegionNode,
  ManageStatus,
  ThisRegionNode,
} from "./contexts/this_region_node"
import { EventLog } from "./contexts/this_p2panda_node"
import { ShowLocalApp, LocalApps, RegionApps } from "./contexts/apps"
import { EnsureRegion, Nodes } from "./contexts/this_region"
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
      {
        path: "",
        element: <EnsureRegion />,
        children: [
          { path: "", element: <Navigate to="/this_region_node" replace /> },
          {
            path: "this_region_node",
            element: <EnsureRegionNode />,
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
              {
                path: "status",
                element: (
                  <RequireNodeSteward>
                    <ManageStatus />
                  </RequireNodeSteward>
                ),
              },
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
            path: "network",
            children: [{ path: "node", element: <P2PandaNode /> }],
          },
          {
            path: "this_region",
            children: [
              { path: "", element: <Navigate to="nodes" replace /> },
              { path: "nodes", element: <Nodes /> },
              { path: "apps", element: <RegionApps /> },
            ],
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
