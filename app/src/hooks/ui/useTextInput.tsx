import React, { useState } from "react"
import TextInput from "../../components/ui/TextInput/TextInput"

type UseTextInput = (
  label: string,
  placeholder: string,
  validationRegex: RegExp
) => {
  TextInput: React.JSX.Element
  value: string
  isInvalid: boolean
}

const useTextInput: UseTextInput = (label, placeholder, regex) => {
  const [value, setValue] = useState<string>("")
  const [isInvalid, setIsInvalid] = useState<boolean>(false)

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue(event.target.value)
    setIsInvalid(!regex.test(event.target.value.toLowerCase()))
  }
  const handleFocusOut = (event: React.FocusEvent<HTMLInputElement>) => {setIsInvalid(!regex.test(event.target.value.toLowerCase()))}

  return {
    TextInput: <TextInput
      label={label}
      placeholder={placeholder}
      value={value}
      isInvalid={isInvalid}
      handleChange={handleChange}
      handleFocusOut={handleFocusOut}
    />,
    value,
    isInvalid
  }
}

export default useTextInput