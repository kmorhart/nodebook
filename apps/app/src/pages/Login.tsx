import React from 'react';
import { useMutation } from "@tanstack/react-query";
import { loginMutateOptions } from "../query/authQueries";
import { useNavigate } from 'react-router';

export default function Login() {
    const { mutateAsync, isPending, error } = useMutation(loginMutateOptions);
    const navigate = useNavigate();

    const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const credentials = Object.fromEntries(formData);

        const me = await mutateAsync(credentials as { identifier: string; password: string });
        if(me) {
            return navigate('/new');
        }
    }

    return(
        <form onSubmit={handleSubmit}>
            <div>
                <label htmlFor="identifier" >Identifier</label>
                <input
                id="identifier"
                name="identifier"
                type="text"
                placeholder="Username or email"
                required
                disabled={isPending}
                />
            </div>
            <div>
                <label htmlFor="password">Password</label>
                <input
                id="password"
                name="password"
                type="password"
                placeholder="••••••••"
                required
                disabled={isPending}
                />
            </div>
            <p>{error?.message}</p>
            <button 
                type="submit"
                disabled={isPending} 
            >
                {isPending ? 'Logging in...' : 'Login'}
            </button>
        </form>
    )
}