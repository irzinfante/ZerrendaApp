import { useState } from "react"
import { dummy } from "../services/dummyService"
import { Authorize } from "./useAuth"

type UseDummy = (authorize: Authorize) => {
  isChecking: boolean
  checkMessage: string | null
  isCheckError: boolean
  checkSession: () => Promise<void>
}

const activeSessionMessage = "Session is active"
const networkErrorMessage = "Could not connect to the server"
const unexpectedErrorMessage = "Something went wrong, try again later"

const useDummy: UseDummy = (authorize) => {
  const [isChecking, setIsChecking] = useState<boolean>(false)
  const [checkMessage, setCheckMessage] = useState<string | null>(null)
  const [isCheckError, setIsCheckError] = useState<boolean>(false)

  const checkSession = async () => {
    setCheckMessage(null)
    setIsCheckError(false)
    setIsChecking(true)
    try {
      const result = await authorize((sessionToken) => dummy({ dummy_string: "ping" }, sessionToken))
      if (result.isNetworkError) {
        setCheckMessage(networkErrorMessage)
        setIsCheckError(true)
      } else if (result.status === 200 && result.response.success) {
        setCheckMessage(activeSessionMessage)
      } else if (result.status !== 401) {
        setCheckMessage(unexpectedErrorMessage)
        setIsCheckError(true)
      }
    } finally {
      setIsChecking(false)
    }
  }

  return { isChecking, checkMessage, isCheckError, checkSession }
}

export default useDummy
