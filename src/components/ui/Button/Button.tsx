import { Button as ChakraUiButton } from "@chakra-ui/react"
import "./Button.css"

interface ButtonProps {
    text: string
    isDisabled: boolean
    handleClick: React.MouseEventHandler<HTMLButtonElement>
}

const Button = ({text, isDisabled, handleClick }: ButtonProps) => (
    <ChakraUiButton
        className="ui-button"
        isDisabled={isDisabled}
        onClick={handleClick}
    >
        {text}
    </ChakraUiButton>
)

export default Button