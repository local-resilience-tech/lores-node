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

const cssVariablesResolver: CSSVariablesResolver = (theme) => ({
  variables: {
    "--lores-burnt-orange6": "#6e2114",
    "--lores-burnt-orange8": "#36100c",
    "--lores-header-gradient": `linear-gradient(to bottom, ${theme.colors.loresOrange[3]}, ${theme.colors.loresOrange[5]})`,
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
