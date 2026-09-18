import { redirect, type MiddlewareFunction } from 'react-router';
import { type Me } from './types';
import { userContext } from './context';
import { queryClient } from '../query/queryClient';
import { meQueryOptions, sessionQueryOptions } from '../query/authQueries';

export const verifySession: MiddlewareFunction = async (args, next) => {
    const isAuthPage = args.request.url.includes('/login') || args.request.url.includes('/register')

    try {
        const userUuid = await queryClient.query({ ...sessionQueryOptions, staleTime: 30 * 1000 });

        if(isAuthPage && userUuid) return redirect("/new");

        return await next();
    } catch (error) {
        if (isAuthPage) return await next()
        
        return redirect("/login");
    }
}

export const verifyMe: MiddlewareFunction = async (args, next) => {
    const isAuthPage = args.request.url.includes('/login') || args.request.url.includes('/register')

    try {
        const me = await queryClient.query({ ...meQueryOptions, staleTime: 5 * 60 * 1000 });

        args.context.set(userContext, me as Me);

        if(isAuthPage && me) return redirect("/new");

        return await next();
    } catch (error) {
        if (isAuthPage) return await next()
        
        return redirect("/login");
    }
}