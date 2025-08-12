import {
  LoaderFunction,
  Navigate,
  RouterProvider,
  createBrowserRouter,
} from "react-router-dom"
import { Layout } from "./pages"
import { EnsureNode, EditNode } from "./contexts/this_node"
import { EventLog, ThisP2PandaNode } from "./contexts/this_p2panda_node"
import { LocalApp, LocalApps, RegionApps } from "./contexts/apps"
import { EnsureRegion, Nodes } from "./contexts/this_region"
import { MantineProvider } from "@mantine/core"
import { Provider as ReduxProvider } from "react-redux"
import { theme } from "./mantine-theme"
import store, { AppStore, loadInitialData } from "./store"
import NewLocalApp from "./contexts/apps/pages/NewLocalApp"
import { Stacks } from "./contexts/stacks"
import { ShowAppRepo, NewAppRepo, AppRepos } from "./contexts/app_repos"

// Import styles of packages that you've installed.
// All packages except `@mantine/hooks` require styles imports
import "@mantine/core/styles.css"

function withStore(
  func: (store: AppStore) => any,
  store: AppStore
): LoaderFunction<any> {
  const wrappedFunc: LoaderFunction<any> = async () => {
    return func(store)
  }
  return wrappedFunc
}

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    loader: withStore(loadInitialData, store),
    children: [
      {
        path: "",
        element: <EnsureRegion />,
        children: [
          { path: "", element: <Navigate to="/this_region/nodes" replace /> },
          {
            path: "this_node",
            element: <EnsureNode />,
            children: [
              { path: "", element: <EditNode /> },
              {
                path: "apps",
                children: [
                  { path: "", element: <LocalApps /> },
                  { path: "new", element: <NewLocalApp /> },
                  { path: "app/:appName", element: <LocalApp /> },
                ],
              },
              {
                path: "app_repos",
                children: [
                  { path: "", element: <AppRepos /> },
                  { path: "new", element: <NewAppRepo /> },
                  { path: "repo/:repoName", element: <ShowAppRepo /> },
                ],
              },
            ],
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
              { path: "p2panda_node", element: <ThisP2PandaNode /> },
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
      <ReduxProvider store={store}>
        <RouterProvider router={router} />
      </ReduxProvider>
    </MantineProvider>
  )
}

export default App
