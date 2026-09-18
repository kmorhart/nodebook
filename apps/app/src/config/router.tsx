import { createBrowserRouter } from 'react-router'
import PublicLayout from '../layouts/PublicLayout'
import ProtectedLayout from '../layouts/ProtectedLayout'
import { verifySession } from '../utils/auth'
import Login from '../pages/Login'
import Register from '../pages/Register'
import Canvas from '../pages/Canvas'


export let router = createBrowserRouter([
    {
        path: '/',
        children: [
            { element: <PublicLayout />, children: [
                { path: 'login', element: <Login /> },
                { path: 'register', element: <Register /> },
            ]},
            { middleware: [verifySession], element: <ProtectedLayout />, children: [
                { path: 'new', element: <Canvas /> },
                { path: 'flows/:flowId', element: <Canvas /> }
            ]},
        ]
    },
    { path: '*', element: <div>any route</div> }
])