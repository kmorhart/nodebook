import { redirect, type MiddlewareFunction } from 'react-router';
import { type User } from './types';
import { userContext } from './context';
import { CONFIG } from './config';

export const authenticate: MiddlewareFunction = async (args, next) => {
    if (args.context.get(userContext)) {
        return await next();
    }

    try {
        const response = await fetch(CONFIG.AUTH_URL + "/me", {
            method: "GET",
            credentials: 'include',
            headers: {
                "Content-Type": "application/json"
            },
        })

        if (!response.ok) {
            throw new Error();
        }
        
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
        
        if (args.request.url.includes('/login') || args.request.url.includes('/register')) {
            return redirect("/new");
        }

        return await next();

    } catch (error) {
        if (args.request.url.includes('/login') || args.request.url.includes('/register')) {
            return await next();
        }
        return redirect("/login");
    }
}