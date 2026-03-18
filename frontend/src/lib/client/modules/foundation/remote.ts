import { env } from '$env/dynamic/public';

type APISuccess<T> = {
  tag: 'success';
  data: T;
  status: number;
};

type APIError = {
  tag: 'error';
  message: string;
  status: number;
  body: unknown;
};

type APIResult<T> = APISuccess<T> | APIError;

type RequestConfig = {
  method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
  path: string;
  body?: Record<string, unknown> | null;
  headers?: Record<string, string>;
  params?: Record<string, string>;
};

function buildUrl(path: string, params?: Record<string, string>): string {
  const base = env.PUBLIC_API_BASE_URL ?? 'http://localhost:3000';
  const url = new URL(path, base);

  if (params) {
    for (const [key, value] of Object.entries(params)) {
      url.searchParams.set(key, value);
    }
  }

  return url.toString();
}

async function request<T>(
  config: RequestConfig,
  decoder: (raw: unknown) => T
): Promise<APIResult<T>> {
  const url = buildUrl(config.path, config.params);

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...config.headers
  };

  const init: RequestInit = {
    method: config.method,
    headers,
    credentials: 'same-origin'
  };

  if (config.body !== null && config.body !== undefined) {
    init.body = JSON.stringify(config.body);
  }

  try {
    const response = await fetch(url, init);
    const raw: unknown = await response.json().catch(() => null);

    if (!response.ok) {
      return {
        tag: 'error',
        message: extractErrorMessage(raw, response.statusText),
        status: response.status,
        body: raw
      };
    }

    const data = decoder(raw);
    return {
      tag: 'success',
      data,
      status: response.status
    };
  } catch (err) {
    return {
      tag: 'error',
      message: err instanceof Error ? err.message : 'Network error',
      status: 0,
      body: null
    };
  }
}

function extractErrorMessage(body: unknown, fallback: string): string {
  if (body !== null && typeof body === 'object') {
    const record = body as Record<string, unknown>;
    if (typeof record['message'] === 'string') {
      return record['message'];
    }
    if (typeof record['error'] === 'string') {
      return record['error'];
    }
  }
  return fallback;
}

class APICaller {
  async call<T>(config: RequestConfig, decoder: (raw: unknown) => T): Promise<APIResult<T>> {
    return request(config, decoder);
  }

  async get<T>(path: string, decoder: (raw: unknown) => T, params?: Record<string, string>): Promise<APIResult<T>> {
    return this.call({ method: 'GET', path, params }, decoder);
  }

  async post<T>(
    path: string,
    body: Record<string, unknown>,
    decoder: (raw: unknown) => T
  ): Promise<APIResult<T>> {
    return this.call({ method: 'POST', path, body }, decoder);
  }

  async put<T>(
    path: string,
    body: Record<string, unknown>,
    decoder: (raw: unknown) => T
  ): Promise<APIResult<T>> {
    return this.call({ method: 'PUT', path, body }, decoder);
  }

  async patch<T>(
    path: string,
    body: Record<string, unknown>,
    decoder: (raw: unknown) => T
  ): Promise<APIResult<T>> {
    return this.call({ method: 'PATCH', path, body }, decoder);
  }

  async delete<T>(path: string, decoder: (raw: unknown) => T): Promise<APIResult<T>> {
    return this.call({ method: 'DELETE', path }, decoder);
  }
}

const apiCaller = new APICaller();

export { apiCaller, buildUrl };
export type { APIResult, APISuccess, APIError, RequestConfig };
