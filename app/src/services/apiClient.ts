import { API_URL } from "../constants/Environment"

export interface ApiError {
  type: "ValidationError" | "AuthError" | "InternalError"
  message: string
}

export interface ApiResponse<T> {
  success: boolean
  error: ApiError | null
  data: T | null
}

export type ApiResult<T> =
  | { isNetworkError: false, status: number, response: ApiResponse<T> }
  | { isNetworkError: true }

const isApiResponse = <T>(json: unknown): json is ApiResponse<T> =>
  typeof json === "object"
  && json !== null
  && "success" in json
  && typeof json.success === "boolean"
  && "error" in json
  && typeof json.error === "object"
  && "data" in json

export const post = async <Res>(
  path: string,
  body: object,
  sessionToken: string | null = null
): Promise<ApiResult<Res>> => {
  const headers: HeadersInit = sessionToken === null
    ? { "Content-Type": "application/json" }
    : { "Content-Type": "application/json", Authorization: `Bearer ${sessionToken}` }

  // Network failures and bodies that are not JSON or not an ApiResponse end up as a network error.
  try {
    const response = await fetch(`${API_URL}${path}`, { method: "POST", headers, body: JSON.stringify(body) })
    const json: unknown = await response.json()
    if (!isApiResponse<Res>(json)) {
      return { isNetworkError: true }
    }
    return { isNetworkError: false, status: response.status, response: json }
  } catch {
    return { isNetworkError: true }
  }
}
