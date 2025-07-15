import { Navigate, RouterProvider, createBrowserRouter } from "react-router-dom"
import Layout from "./pages/Layout"
import { ChakraProvider } from "@chakra-ui/react"
import { ColorModeProvider } from "./components/ui/color-mode"
import { themeSystem } from "./theme"
import { EnsureNode } from "./contexts/this_node"
import { ThisP2PandaNode } from "./contexts/this_p2panda_node"
import { EnsureRegion, Nodes } from "./contexts/this_region"

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
    <ChakraProvider value={themeSystem}>
      <ColorModeProvider>
        <RouterProvider router={router} />
      </ColorModeProvider>
    </ChakraProvider>
  )
}

export default App
