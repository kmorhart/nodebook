import { createBrowserRouter, Navigate } from 'react-router'
import PublicLayout from '../layouts/PublicLayout'
import ProtectedLayout from '../layouts/ProtectedLayout'
import { verifySession } from '../utils/middleware'
import { logout, verify } from '../utils/loaders'
import Login from '../pages/Login'
import Register from '../pages/Register'
import Verify from '../pages/Verify'
import Canvas from '../components/Canvas'


export let router = createBrowserRouter([
    {
        path: '/',
        children: [
            { element: <PublicLayout />, children: [
                { path: 'login', element: <Login /> },
                { path: 'register', element: <Register /> },
                { path: 'verify/:token', loader: verify, element: <Verify /> }
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