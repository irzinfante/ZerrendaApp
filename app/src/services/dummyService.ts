import { ApiResult, post } from "./apiClient"

export interface DummyRequest {
  dummy_string: string
}

export interface DummyResponse {
  dummy_message: string
}

export const dummy = (request: DummyRequest, sessionToken: string): Promise<ApiResult<DummyResponse>> =>
  post<DummyResponse>("/dummy", request, sessionToken)
