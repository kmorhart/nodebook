import { redirect, type MiddlewareFunction } from 'react-router';
import { type Me } from './types';
import { userContext } from './context';
import { queryClient } from '../query/queryClient';
import { meQueryOptions } from '../query/authQueries';

export const fetchMe: MiddlewareFunction = async (args, next) => {
    console.log('fetchMe middleware called for', args.request.url);
    const isAuthPage = args.request.url.includes('/login') || args.request.url.includes('/register')
    console.log('fetchMe middleware called for', args.request.url, 'isAuthPage:', isAuthPage);

    try {
        const me = await queryClient.query({ ...meQueryOptions, staleTime: 'static' });

        args.context.set(userContext, me as Me);

        if(isAuthPage) return redirect("/new");

        return await next();
    } catch (error) {
        if (isAuthPage) return await next()
        
        return redirect("/login");
    }
}