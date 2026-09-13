import { redirect, type MiddlewareFunction } from 'react-router'
import { type User } from './types'
import { userContext } from './context'

export const authenticate: MiddlewareFunction = async (args, next) => {
    try {
        const response = await fetch("http://localhost:7005/me", {
            method: "POST",
            credentials: 'include',
            headers: {
                "Content-Type": "application/json"
            },
        })
        
        const data = await response.json();

        const user: User = {
            uuid: data.uuid,
            username: data.username,
            email: data.email,
            isActive: data.isActive,
            isVerified: data.isVerified,
            createdAt: new Date(data.createdAt)
        }

        args.context.set(userContext, user);

        return await next();

    } catch (error) {
        return redirect("/login");
    }
}