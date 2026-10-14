import { useEffect, useRef } from "react"
import { emailPattern, passwordPattern } from "../../constants/Patterns"
import { AuthAction } from "../../hooks/useAuth"
import useTextInput from "../../hooks/ui/useTextInput"
import useButton from "../../hooks/ui/useButton"
import Container from "../Container/Container"
import "./Login.css"

interface LoginProps {
  pendingAction: AuthAction | null
  errorMessage: string | null
  login: (email: string, password: string) => Promise<void>
  register: (email: string, password: string) => Promise<void>
  clearError: () => void
}

const Login = ({ pendingAction, errorMessage, login, register, clearError }: LoginProps) => {
  const { TextInput: EmailInput, value: emailValue, isInvalid: isEmailInvalid } = useTextInput("Email", "name@example.org", emailPattern)
  const { TextInput: PasswordInput, value: passwordValue, isInvalid: isPasswordInvalid } = useTextInput("Password", "", passwordPattern, "password")

  const isDisabled = emailValue === "" || isEmailInvalid || passwordValue === "" || isPasswordInvalid || pendingAction !== null

  const handleLogin = () => {
    void login(emailValue, passwordValue)
  }
  const handleRegister = () => {
    void register(emailValue, passwordValue)
  }

  const { Button: LoginButton } = useButton("Log in", isDisabled, handleLogin, pendingAction === "login", "primary")
  const { Button: SignUpButton } = useButton("Sign up", isDisabled, handleRegister, pendingAction === "register", "secondary")

  // Compared with the previous values, not run on mount, so the message of an expired session survives the remount.
  const credentials = useRef({ emailValue, passwordValue })
  useEffect(() => {
    if (credentials.current.emailValue !== emailValue || credentials.current.passwordValue !== passwordValue) {
      credentials.current = { emailValue, passwordValue }
      clearError()
    }
  }, [emailValue, passwordValue, clearError])

  return (
    <section id="login-section">
      <Container id="login-container">
        <h5>Log in to continue</h5>
        {EmailInput}
        {PasswordInput}
        <div className="login-buttons">
          {LoginButton}
          {SignUpButton}
        </div>
        {errorMessage !== null && <span className="login-error">{errorMessage}</span>}
      </Container>
    </section>
  )
}

export default Login
