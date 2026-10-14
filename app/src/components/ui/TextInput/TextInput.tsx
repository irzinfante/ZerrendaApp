import { FormControl, FormLabel, Input } from "@chakra-ui/react"

export type TextInputType = "text" | "password"

interface TextInputProps {
  label: string
  placeholder: string
  value: string
  isInvalid: boolean
  type: TextInputType
  handleChange: React.ChangeEventHandler<HTMLInputElement>
  handleFocusOut: React.FocusEventHandler<HTMLInputElement>
}

const TextInput = ({ label, placeholder, value, isInvalid, type, handleChange, handleFocusOut }: TextInputProps) => (
  <FormControl isInvalid={isInvalid}>
    <FormLabel>{label}</FormLabel>
    <Input
      type={type}
      placeholder={placeholder}
      value={value}
      onChange={handleChange}
      onBlur={handleFocusOut}
    />
  </FormControl>
)

export default TextInput
