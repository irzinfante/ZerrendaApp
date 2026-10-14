import { Button as ChakraUiButton } from "@chakra-ui/react"

export type ButtonVariant = "primary" | "secondary"

interface ButtonProps {
  text: string
  isDisabled: boolean
  isLoading: boolean
  variant: ButtonVariant
  handleClick: React.MouseEventHandler<HTMLButtonElement>
}

const Button = ({ text, isDisabled, isLoading, variant, handleClick }: ButtonProps) => (
  <ChakraUiButton
    isDisabled={isDisabled}
    isLoading={isLoading}
    variant={variant}
    onClick={handleClick}
  >
    {text}
  </ChakraUiButton>
)

export default Button
