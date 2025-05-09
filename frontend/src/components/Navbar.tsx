import {
  Box,
  Flex,
  Stack,
  Button,
  Link as ExternalLink,
} from "@chakra-ui/react"
import { Link } from "react-router-dom"
import { ColorModeButton, useColorModeValue } from "./ui/color-mode"
import packageJson from "../../package.json"

export default function Navbar() {
  return (
    <>
      <Box bg={useColorModeValue("gray.100", "gray.900")} px={4}>
        <Flex alignItems={"center"} justifyContent={"space-between"}>
          <Stack direction="row" alignItems="center">
            <Box>
              <Link to="/">LoRes Mesh</Link>{" "}
              <ExternalLink
                variant="underline"
                href={
                  packageJson.homepage + "/releases/tag/v" + packageJson.version
                }
              >
                v{packageJson.version}
              </ExternalLink>
            </Box>
            <Box>
              <Link to="/node">
                <Button colorPalette="blue" variant="ghost">
                  P2Panda
                </Button>
              </Link>
              <Link to="/sites">
                <Button colorPalette="blue" variant="ghost">
                  Sites
                </Button>
              </Link>
              <Link to="/this_node">
                <Button colorPalette="blue" variant="ghost">
                  This Node
                </Button>
              </Link>
            </Box>
          </Stack>

          <Flex alignItems={"center"}>
            <Stack direction={"row"}>
              <ColorModeButton />
            </Stack>
          </Flex>
        </Flex>
      </Box>
    </>
  )
}
