import React from 'react';
import { useMutation } from "@tanstack/react-query";
import { registerMutateOptions } from "../query/authQueries";
import { useNavigate } from 'react-router';

export default function Register() {
    const { mutateAsync, isPending, error } = useMutation(registerMutateOptions);
    const navigate = useNavigate();

    const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const credentials = Object.fromEntries(formData);

        const me = await mutateAsync(credentials as { email: string; username: string; password: string });
        if(me) {
            return navigate('/new');
        }
    }
    
    return(
        <form onSubmit={handleSubmit}>
            <div>
                <label htmlFor="email" >Email Address</label>
                <input
                id="email"
                name="email"
                type="text"
                placeholder="you@domain.com"
                required
                disabled={isPending}
                />
            </div>
            <div>
                <label htmlFor="username">Username</label>
                <input
                id="username"
                name="username"
                type="text"
                placeholder="username"
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
                {isPending ? 'Creating Account...' : 'Create Account'}
            </button>
        </form>
    )
}