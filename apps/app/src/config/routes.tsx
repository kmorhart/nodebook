import { createBrowserRouter } from 'react-router'
import ProtectedLayout from '../layouts/ProtectedLayout'
import { authenticate } from '../utils/authentication'
import Login from '../pages/Login'

export let router = createBrowserRouter([
    {
        path: '/',
        children: [
            {
                path: '/login',
                element: <Login />
            },
            {
                path: 'flows/:flowId',
                middleware: [authenticate],
                element: <ProtectedLayout />,
                // loader: async ({ request, params }) => {
                //     return await fetch(`/api/flow/${params.flowId}`,
                //         { signal: request.signal }
                //     )
                // }
            }
        ]
    },
    {
        path: '*',
        element: <div>home</div>,
        errorElement: <div>error</div>
    }
])