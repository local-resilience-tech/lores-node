import { createTheme, Input, InputWrapper } from "@mantine/core"

export const theme = createTheme({
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
