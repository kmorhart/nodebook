import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useNavigate } from 'react-router'
import { CONFIG } from '../config/config'
import { meQueryOptions } from '../query/authQueries'

export function useLogout() {
  const queryClient = useQueryClient()
  const navigate = useNavigate()

  return useMutation({
    mutationFn: () =>
      fetch(CONFIG.AUTH_URL + '/logout', { method: 'POST', credentials: 'include' }),
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: meQueryOptions.queryKey })
      navigate('/login')
    },
  })
}