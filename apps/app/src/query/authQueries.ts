import { queryOptions } from '@tanstack/react-query';
import { queryClient } from './queryClient';
import { CONFIG } from '../config/config';
import type { Me } from '../utils/types';

export async function fetchRegister(email: string, username: string, password: string): Promise<Me> {
  const response = await fetch(CONFIG.AUTH_URL + '/register', {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, username, password }),
  })

  if (!response.ok) throw new Error('An error occurred while creating the account. Please try again.')

  const data = await response.json()

  queryClient.setQueryData(['auth', 'session'], data.uuid)
  queryClient.setQueryData(['auth', 'me'], data)
  
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
    fetchRegister(credentials.email, credentials.username, credentials.password),
}

export async function fetchLogin(identifier: string, password: string): Promise<Me> {
  const response = await fetch(CONFIG.AUTH_URL + '/login', {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ identifier, password }),
  })

  if (!response.ok) throw new Error('An error occurred while logging in. Please check your credentials and try again.')

  const data = await response.json()

  queryClient.setQueryData(['auth', 'session'], data.uuid)
  queryClient.setQueryData(['auth', 'me'], data)
  
  return {
    uuid: data.uuid,
    username: data.username,
    email: data.email,
    isActive: data.isActive,
    isVerified: data.isVerified,
    createdAt: new Date(data.createdAt),
  }
}

export const loginMutateOptions = {
  mutationFn: (credentials: { identifier: string; password: string }) =>
    fetchLogin(credentials.identifier, credentials.password),
}

export async function fetchSession(): Promise<Me> {
  const response = await fetch(CONFIG.AUTH_URL + '/session', {
    method: 'GET',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
  })

  if (!response.ok) throw new Error('Not authenticated')

  const data = await response.json()
  
  return data
}

export const sessionQueryOptions = queryOptions({
  queryKey: ['auth', 'session'],
  queryFn: fetchSession,
  staleTime: 1000 * 60 * 10,
})

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
  staleTime: 1000 * 60 * 10,
})