import { useState } from "react";
import React from 'react';
import { useMutation } from "@tanstack/react-query";
import { CONFIG } from "../config/config";
import { registerMutateOptions } from "../query/authQueries";

export default function Register() {
    const { mutate, isPending, error, isError } = useMutation(registerMutateOptions);

    const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const credentials = Object.fromEntries(formData);

        mutate(credentials as { username: string; email: string; password: string });
    }

    
    return(
        <form onSubmit={handleSubmit}>
            <div>
                <label htmlFor="email" >Email Address</label>
                <input
                id="email"
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
                type="password"
                placeholder="••••••••"
                required
                disabled={isPending}
                />
            </div>

            <button 
                type="submit" 
                disabled={isPending} 
            >
                {isPending ? 'Creating Account...' : 'Create Account'}
            </button>
        </form>
    )
}