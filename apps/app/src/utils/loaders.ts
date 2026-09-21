import { logoutMutationOptions } from '../query/authQueries'
import { type LoaderFunction } from 'react-router'

export const logout: LoaderFunction = async () => {
    try {
        await logoutMutationOptions.mutationFn()
        logoutMutationOptions.onSuccess?.()
    } catch (error) {
        throw new Error('An error occurred while logging out. Please try again.')
    }
}