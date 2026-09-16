import { createBrowserRouter } from 'react-router'
import ProtectedLayout from '../layouts/ProtectedLayout'
import { fetchMe } from '../utils/auth'
import Login from '../pages/Login'
import Register from '../pages/Register'

export let router = createBrowserRouter([
    {
        path: '/',
        children: [
            { path: 'login', middleware: [fetchMe], element: <Login /> },
            { path: 'register', middleware: [fetchMe], element: <Register /> },
            { path: 'new', middleware: [fetchMe], element: <ProtectedLayout /> },
            { path: 'flows/:flowId', middleware: [fetchMe], element: <ProtectedLayout /> }
        ]
    },
    { path: '*', element: <div>any route</div> }
])