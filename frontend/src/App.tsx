import {
  LoaderFunction,
  Navigate,
  RouterProvider,
  createBrowserRouter,
} from "react-router-dom"
import { Layout } from "./pages"
import { EnsureNode } from "./contexts/this_node"
import { ThisP2PandaNode } from "./contexts/this_p2panda_node"
import { EnsureRegion, Nodes } from "./contexts/this_region"
import { MantineProvider } from "@mantine/core"
import { Provider as ReduxProvider } from "react-redux"
import { theme } from "./mantine-theme"

// Import styles of packages that you've installed.
// All packages except `@mantine/hooks` require styles imports
import "@mantine/core/styles.css"

import store, { AppStore, loadInitialData } from "./store"

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
          { path: "", element: <Navigate to="nodes" replace /> },
          { path: "nodes", element: <Nodes /> },
          { path: "this_node", element: <EnsureNode /> },
        ],
      },
      { path: "p2panda_node", element: <ThisP2PandaNode /> },
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
