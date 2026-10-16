import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useNavigate } from 'react-router'
import { meQueryOptions } from '../query/authQueries'


const authUrl = import.meta.env.VITE_AUTH_URL

export function useLogout() {
  const queryClient = useQueryClient()
  const navigate = useNavigate()

  return useMutation({
    mutationFn: () =>
      fetch(authUrl + 'logout', { method: 'POST', credentials: 'include' }),
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: meQueryOptions.queryKey })
      navigate('/login')
    },
  })
}