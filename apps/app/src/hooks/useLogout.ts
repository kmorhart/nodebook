import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useNavigate } from 'react-router'
import { meQueryOptions } from '../query/authQueries'

export function useLogout() {
  const queryClient = useQueryClient()
  const navigate = useNavigate()

  return useMutation({
    mutationFn: () =>
      fetch('http://localhost:7005/logout', { method: 'POST', credentials: 'include' }),
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: meQueryOptions.queryKey })
      navigate('/login')
    },
  })
}