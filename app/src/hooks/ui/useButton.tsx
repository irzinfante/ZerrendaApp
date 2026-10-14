import React from "react"
import Button, { ButtonVariant } from "../../components/ui/Button/Button"

type UseButton = (
  text: string,
  isDisabled: boolean,
  handleClick: React.MouseEventHandler,
  isLoading?: boolean,
  variant?: ButtonVariant
) => {
  Button: React.JSX.Element
}

const useButton: UseButton = (text, isDisabled, handleClick, isLoading = false, variant = "primary") => {
  return {
    Button: <Button
      text={text}
      isDisabled={isDisabled}
      isLoading={isLoading}
      variant={variant}
      handleClick={handleClick}
    />
  }
}

export default useButton
