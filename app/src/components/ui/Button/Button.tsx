import { Button as ChakraUiButton } from "@chakra-ui/react"

interface ButtonProps {
  text: string
  isDisabled: boolean
  handleClick: React.MouseEventHandler<HTMLButtonElement>
}

const Button = ({text, isDisabled, handleClick }: ButtonProps) => (
  <ChakraUiButton
    isDisabled={isDisabled}
    onClick={handleClick}
  >
    {text}
  </ChakraUiButton>
)

export default Button