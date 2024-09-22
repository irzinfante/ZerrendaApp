import { FormControl, FormLabel, Input } from "@chakra-ui/react"
import "./TextInput.css"

interface TextInputProps {
    label: string
    placeholder: string
    value: string
    isInvalid: boolean
    handleChange: React.ChangeEventHandler<HTMLInputElement>
    handleFocusOut: React.FocusEventHandler<HTMLInputElement>
}

const TextInput = ({ label, placeholder, value, isInvalid, handleChange, handleFocusOut }: TextInputProps) => (
    <FormControl isInvalid={isInvalid} >
        <FormLabel>{label}</FormLabel>
        <Input
            className="ui-textInput"
            placeholder={placeholder}
            value={value}
            onChange={handleChange}
            onBlur={handleFocusOut}
        />
    </FormControl>
)

export default TextInput