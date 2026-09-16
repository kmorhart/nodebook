import { queryOptions } from '@tanstack/react-query'
import { CONFIG } from '../config/config'
import type { Me } from '../utils/types'

export async function fetchRegister(email: string, username: string, password: string): Promise<Me> {
  const response = await fetch(CONFIG.AUTH_URL + '/register', {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, email, password }),
  })

  if (!response.ok) throw new Error('Not authenticated')

  const data = await response.json()
  
  return {
    uuid: data.uuid,
    username: data.username,
    email: data.email,
    isActive: data.isActive,
    isVerified: data.isVerified,
    createdAt: new Date(data.createdAt),
  }
}

export const registerMutateOptions = {
  mutationFn: (credentials: { username: string; email: string; password: string }) =>
    fetchRegister(credentials.username, credentials.email, credentials.password),
}

async function fetchMe(): Promise<Me> {
  const response = await fetch(CONFIG.AUTH_URL + '/me', {
    method: 'GET',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
  })

  if (!response.ok) throw new Error('Not authenticated')

  const data = await response.json()
  
  return {
    uuid: data.uuid,
    username: data.username,
    email: data.email,
    isActive: data.isActive,
    isVerified: data.isVerified,
    createdAt: new Date(data.createdAt),
  }
}

export const meQueryOptions = queryOptions({
  queryKey: ['auth', 'me'],
  queryFn: fetchMe,
})