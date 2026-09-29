import { createBrowserRouter, Navigate } from 'react-router'
import PublicLayout from '../layouts/PublicLayout'
import ProtectedLayout from '../layouts/ProtectedLayout'
import { verifySession } from '../utils/middleware'
import { logout } from '../utils/loaders'
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
                { path: 'logout', loader: logout, element: <Navigate to="/login" /> },
                { path: 'new', element: <Canvas /> },
                { path: 'flows/:flowId', element: <Canvas /> },
            ]},
        ]
    },
    { path: '*', element: <Navigate to="/new" /> }
])