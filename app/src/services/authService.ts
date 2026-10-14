import { ApiResult, post } from "./apiClient"

export interface RegisterRequest {
  email: string
  password: string
}

export interface RegisterResponse {
  session_token: string
}

export interface LoginRequest {
  email: string
  password: string
}

export interface LoginResponse {
  session_token: string
}

export const register = (request: RegisterRequest): Promise<ApiResult<RegisterResponse>> =>
  post<RegisterResponse>("/auth/register", request)

export const login = (request: LoginRequest): Promise<ApiResult<LoginResponse>> =>
  post<LoginResponse>("/auth/login", request)
