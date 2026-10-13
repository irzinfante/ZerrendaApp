import React from "react"
import Button from "../../components/ui/Button/Button"

type UseButton = (
  text: string,
  isDisabled: boolean,
  handleClick: React.MouseEventHandler
) => {
  Button: React.JSX.Element
}

const useButton: UseButton = (text, isDisabled, handleClick) => {
  return {
    Button: <Button
      text={text}
      isDisabled={isDisabled}
      handleClick={handleClick}
    />
  }
}

export default useButton