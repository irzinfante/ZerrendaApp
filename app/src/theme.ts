import { extendTheme } from "@chakra-ui/react"

// Colors come from the CSS variables in index.css, the single source of the palette.
const theme = extendTheme({
  fonts: {
    heading: `"Work Sans", sans-serif`,
    body: `"Source Sans 3", sans-serif`,
  },
  components: {
    Button: {
      baseStyle: {
        fontFamily: "heading",
        fontWeight: 600,
        border: "none",
        borderRadius: "10px",
        color: "var(--white)",
        _hover: { filter: "brightness(85%)" },
        _disabled: {
          bg: "var(--ash-gray)",
          filter: "brightness(85%)",
          opacity: 1,
          cursor: "not-allowed",
        },
      },
      sizes: {
        md: { h: "auto", px: "1.5rem", py: "0.5rem", fontSize: "1rem" },
      },
      variants: {
        primary: {
          bg: "var(--striped-blue)",
          _hover: { _disabled: { bg: "var(--ash-gray)" } },
        },
        secondary: {
          bg: "var(--sage-green)",
          _hover: { _disabled: { bg: "var(--ash-gray)" } },
        },
      },
      defaultProps: { variant: "primary", size: "md" },
    },
    Input: {
      baseStyle: {
        field: { boxSizing: "border-box", fontFamily: "body" },
      },
      sizes: {
        md: {
          field: { h: "auto", px: "0.5rem", py: "0.5rem", fontSize: "1rem", borderRadius: "10px" },
        },
      },
      variants: {
        outline: {
          field: {
            bg: "var(--off-white)",
            border: "1px solid",
            borderColor: "var(--ash-gray)",
            _hover: { borderColor: "var(--ash-gray)" },
            _focusVisible: { borderColor: "var(--striped-blue)", boxShadow: "none" },
            _invalid: {
              borderColor: "var(--terracotta-red)",
              color: "var(--terracotta-red)",
              boxShadow:
                "inset 0 0 5px 5px rgba(from var(--terracotta-red) r g b / 0.2), 0 0 5px 5px rgba(from var(--terracotta-red) r g b / 0.2)",
            },
          },
        },
      },
    },
    FormLabel: {
      baseStyle: {
        fontFamily: `"Poppins", sans-serif`,
        fontWeight: 400,
        fontSize: "1rem",
        color: "var(--charcoal-gray)",
        mb: 0,
      },
    },
  },
})

export default theme
