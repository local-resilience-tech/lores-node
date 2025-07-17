import { Navigate, RouterProvider, createBrowserRouter } from "react-router-dom"
import { Layout } from "./pages"
import { ChakraProvider } from "@chakra-ui/react"
import { ColorModeProvider } from "./components/ui/color-mode"
import { themeSystem } from "./chakra-theme"
import { EnsureNode } from "./contexts/this_node"
import { ThisP2PandaNode } from "./contexts/this_p2panda_node"
import { EnsureRegion, Nodes } from "./contexts/this_region"
import { MantineProvider } from "@mantine/core"
import { theme } from "./mantine-theme"

// Import styles of packages that you've installed.
// All packages except `@mantine/hooks` require styles imports
import "@mantine/core/styles.css"

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
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
      <ChakraProvider value={themeSystem}>
        <ColorModeProvider>
          <RouterProvider router={router} />
        </ColorModeProvider>
      </ChakraProvider>
    </MantineProvider>
  )
}

export default App
