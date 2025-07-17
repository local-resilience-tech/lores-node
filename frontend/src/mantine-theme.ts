import { createTheme } from "@mantine/core"

export const theme = createTheme({
  components: {
    InputWrapper: {
      vars: (_theme: any, props: any) => {
        if (!props.size) {
          return {
            root: {
              "--input-label-size": "18px",
              "--input-description-size": "14px",
              "--input-fz": "16px",
            },
          }
        }

        return {}
      },
    },
  },
})
