const baseURL = import.meta.env.VITE_API_ADDR;

interface RequestOptions extends RequestInit {
  headers?: HeadersInit;
  params?: Record<string, string | number>;
}

function buildQuery(params?: Record<string, string | number>): string {
  if (!params) return "";
  const query = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    query.append(key, value.toString());
  }
  return `?${query.toString()}`;
}

export async function apiFetch<T>(
  path: string,
  options: RequestOptions = {}
): Promise<T> {
  const url = `${baseURL}${path}${buildQuery(options.params)}`;
  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...(options.headers || {}),
  };

  const response = await fetch(url, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP error ${response.status}`);
  }

  return response.json();
}
