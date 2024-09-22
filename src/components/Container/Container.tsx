import { ReactNode } from "react"
import "./Container.css"

interface ContainerProps {
  id?: string
  children?: ReactNode
}

const Container = ({id, children}: ContainerProps) => {
  return (
    <div id={id} className="container">
      {children}
    </div>
  )
}

export default Container
