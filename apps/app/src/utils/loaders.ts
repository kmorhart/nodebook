import { logoutMutationOptions, verifyMutationOptions } from '../query/authQueries'
import { type LoaderFunction, type LoaderFunctionArgs } from 'react-router'

export const logout: LoaderFunction = async () => {
    try {
        await logoutMutationOptions.mutationFn()
        logoutMutationOptions.onSuccess?.()
    } catch (error) {
        throw new Error('An error occurred while logging out. Please try again.')
    }
}


export async function verify({ params }: LoaderFunctionArgs) {
    const token = params.token

    if (!token) {
        throw new Response('Missing verification token', { status: 400 })
    }

    try {
        await verifyMutationOptions.mutationFn(token)
        return { success: true }
    } catch {
        return { success: false }
    }
}