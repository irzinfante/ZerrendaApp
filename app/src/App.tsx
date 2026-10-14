import { useEffect, useState } from "react"
import { emailPattern } from "./constants/Patterns"
import Container from "./components/Container/Container"
import useTextInput from "./hooks/ui/useTextInput"
import useButton from "./hooks/ui/useButton"
import Footer from "./components/Footer/Footer"
import "./App.css"

const App = () => {
  const [isLoginSuccess, setIsLoginSuccess] = useState<boolean>(false)
  const [isLoginError, setIsLoginError] = useState<boolean>(false)

  const login = () => {
    setIsLoginSuccess(false)
    setIsLoginError(false)
    if (Math.random() < 0.5) {
      setIsLoginSuccess(true)
    } else {
      setIsLoginError(true)
    }
  }

  const { TextInput: EmailInput, value: emailValue, isInvalid: isEmailInvalid } = useTextInput("Email", "name@example.org", emailPattern)
  const { Button: LoginButton } = useButton("Log in", emailValue === "" || isEmailInvalid, login)

  useEffect(() => {
    setIsLoginSuccess(false)
    setIsLoginError(false)
  }, [emailValue])

  return (
    <div id="app-container">
      <div id="app-logo-container">
        <h1 id="app-logo">Zerrenda<span className="charcoal-gray">App</span></h1>
      </div>
      <section id="login-section">
        <Container id="login-container">
          <h5>Log in to continue</h5>
          {EmailInput}
          {LoginButton}
          {isLoginSuccess && <span className="login-success">Login link sent by email</span>}
          {isLoginError && <span className="login-error">Login attempt failed</span>}
        </Container>
      </section>
      <Footer />
    </div>
  )
}

export default App
