import { redirect, type MiddlewareFunction } from 'react-router';
import { queryClient } from '../query/queryClient';
import { sessionQueryOptions } from '../query/authQueries';

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