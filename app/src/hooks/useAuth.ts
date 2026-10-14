import { useCallback, useState } from "react"
import { ApiResult } from "../services/apiClient"
import { login as loginRequest, register as registerRequest } from "../services/authService"

export type AuthAction = "login" | "register"

export type Authorize = <T>(
  request: (sessionToken: string) => Promise<ApiResult<T>>
) => Promise<ApiResult<T>>

interface Session {
  sessionToken: string
  email: string
  action: AuthAction
}

type UseAuth = () => {
  isLoggedIn: boolean
  email: string | null
  sessionAction: AuthAction | null
  pendingAction: AuthAction | null
  errorMessage: string | null
  login: (email: string, password: string) => Promise<void>
  register: (email: string, password: string) => Promise<void>
  authorize: Authorize
  clearError: () => void
}

const incorrectCredentialsMessage = "Incorrect email or password"
const invalidSignUpMessage = "Could not sign up with this email and password"
const networkErrorMessage = "Could not connect to the server"
const unexpectedErrorMessage = "Something went wrong, try again later"
const sessionExpiredMessage = "Your session has expired, log in again"

const getErrorMessage = (action: AuthAction, result: ApiResult<unknown>): string => {
  if (result.isNetworkError) {
    return networkErrorMessage
  }
  const errorType = result.response.error?.type
  if (action === "login" && result.status === 401 && errorType === "AuthError") {
    return incorrectCredentialsMessage
  }
  if (action === "register" && result.status === 400 && errorType === "ValidationError") {
    return invalidSignUpMessage
  }
  return unexpectedErrorMessage
}

const useAuth: UseAuth = () => {
  const [session, setSession] = useState<Session | null>(null)
  const [pendingAction, setPendingAction] = useState<AuthAction | null>(null)
  const [errorMessage, setErrorMessage] = useState<string | null>(null)

  const openSession = async (action: AuthAction, email: string, password: string) => {
    setErrorMessage(null)
    setPendingAction(action)
    try {
      const request = action === "login" ? loginRequest : registerRequest
      const result = await request({ email, password })
      if (!result.isNetworkError && result.status === 200 && result.response.success && result.response.data !== null) {
        setSession({ sessionToken: result.response.data.session_token, email, action })
      } else {
        setErrorMessage(getErrorMessage(action, result))
      }
    } finally {
      setPendingAction(null)
    }
  }

  const login = (email: string, password: string) => openSession("login", email, password)
  const register = (email: string, password: string) => openSession("register", email, password)

  // Every request to a protected route goes through here, so a 401 closes the session in a single place.
  const authorize: Authorize = async (request) => {
    if (session === null) {
      return { isNetworkError: true }
    }
    const result = await request(session.sessionToken)
    if (!result.isNetworkError && result.status === 401) {
      setSession(null)
      setErrorMessage(sessionExpiredMessage)
    }
    return result
  }

  const clearError = useCallback(() => {
    setErrorMessage(null)
  }, [])

  return {
    isLoggedIn: session !== null,
    email: session?.email ?? null,
    sessionAction: session?.action ?? null,
    pendingAction,
    errorMessage,
    login,
    register,
    authorize,
    clearError,
  }
}

export default useAuth
