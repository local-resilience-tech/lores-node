import {
  createTheme,
  Input,
  InputWrapper,
  CSSVariablesResolver,
} from "@mantine/core"

// Lores website orange palette (--orange0 through --orange9)
const loresOrange: [
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
] = [
  "#fff4e1", // orange0
  "#ffe8cc", // orange1
  "#fed09b", // orange2
  "#fdb766", // orange3
  "#fca13a", // orange4
  "#fc931d", // orange5
  "#e17800", // orange6
  "#e17800", // orange7
  "#c86a00", // orange8
  "#af5a00", // orange9
]

// Lores website burnt-orange palette (--burnt-orange0 through --burnt-orange9)
const loresBurntOrange: [
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
] = [
  "#fdf0ed", // burnt-orange0
  "#f6bfb2", // burnt-orange1
  "#ee8e77", // burnt-orange2
  "#e65e3c", // burnt-orange3
  "#c63c1a", // burnt-orange4
  "#8b2b13", // burnt-orange5
  "#6e2114", // burnt-orange6
  "#521812", // burnt-orange7
  "#36100c", // burnt-orange8
  "#1e0400", // burnt-orange9
]

// Lores website blue palette (--blue0 through --blue9)
const loresBlue: [
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
  string,
] = [
  "#edf8fd", // blue0
  "#b8dff3", // blue1
  "#89c2e5", // blue2
  "#5ea3d1", // blue3
  "#3f80b2", // blue4
  "#365c7d", // blue5
  "#244f6f", // blue6
  "#16415d", // blue7
  "#0b3248", // blue8
  "#04212f", // blue9
]

const paletteVars = (name: string, shades: readonly string[]) =>
  Object.fromEntries(shades.map((value, i) => [`--lores-${name}${i}`, value]))

const cssVariablesResolver: CSSVariablesResolver = (theme) => ({
  variables: {
    ...paletteVars("orange", theme.colors.loresOrange),
    ...paletteVars("blue", theme.colors.loresBlue),
    ...paletteVars("burnt-orange", theme.colors.loresBurntOrange),
  },
  light: {},
  dark: {
    "--mantine-color-body": theme.colors["loresBlue"][5],
  },
})

export { cssVariablesResolver }

export const theme = createTheme({
  primaryColor: "loresOrange",
  colors: {
    loresBlue,
    loresOrange,
    loresBurntOrange,
  },
  components: {
    InputWrapper: InputWrapper.extend({
      vars: (_theme: any, props: any) => {
        var result: any = {
          description: {
            lineHeight: "1.3",
            paddingBottom: "0.2em",
          },
          root: {},
        }
        if (!props.size) {
          result.root = {
            ...result.root,
            "--input-label-size": "18px",
            "--input-description-size": "16px",
            "--input-fz": "16px",
          }
        }

        return result
      },
    }),
    Input: Input.extend({
      vars: (_theme: any, _props: any) => {
        return {
          wrapper: {},
          input: {
            "--input-bg": "var(--mantine-color-dark-8)",
          },
        }
      },
    }),
  },
})
