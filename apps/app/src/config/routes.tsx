import { createBrowserRouter } from 'react-router'
import ProtectedLayout from '../layouts/ProtectedLayout'
import { authenticate } from '../utils/authentication'
import Login from '../pages/Login'
import Register from '../pages/Register'

export let router = createBrowserRouter([
    {
        path: '/',
        children: [
            {
                path: '/login',
                middleware: [authenticate],
                element: <Login />
            },
            {
                path: '/register',
                middleware: [authenticate],
                element: <Register />
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