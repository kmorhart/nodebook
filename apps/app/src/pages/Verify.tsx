import { useLoaderData } from 'react-router'
import type { verify } from '../utils/loaders'

export default function Verify() {
    const { success } = useLoaderData<typeof verify>()

    return (
        <p>
            {success
                ? 'Your account has been verified. You can now log in.'
                : 'We could not verify your account. The link may have expired.'}
        </p>
    )
}