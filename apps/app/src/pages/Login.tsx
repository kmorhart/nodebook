import { useState } from "react";
import React from 'react';
import { CONFIG } from "../utils/config";

export default function Login() {
    const [identifier, setIdentifier] = useState('');
    const [password, setPassword] = useState('');
    
    // Status states
    const [isLoading, setIsLoading] = useState(false);
    const [message, setMessage] = useState('');

    const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault();

        // Reset status messages on new submission
        setMessage('');
        setIsLoading(true);

        try {
            const response = await fetch(CONFIG.AUTH_URL + '/login', {
                method: 'POST',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ 
                    identifier: identifier,
                    password: password
                }),
            });

            const data = await response.json();

            if (!response.ok) {
                throw new Error(data.message || 'Something went wrong. Please try again.');
            }


        } catch (error: Error | any) {
            setMessage(error.message || 'Something went wrong. Please try again.');
        } finally {
            setIsLoading(false);
        }
    }

    
    return(
        <>
            <div>
                <div>
                    <h2>Welcome Back</h2>
                    
                    {message && <p>{message}</p>}

                    <form onSubmit={handleSubmit}>
                    <div>
                        <label htmlFor="identifier" >Email Address</label>
                        <input
                        id="identifier"
                        type="text"
                        value={identifier}
                        onChange={(e) => setIdentifier(e.target.value)}
                        placeholder="you@domain.com"
                        required
                        disabled={isLoading}
                        />
                    </div>

                    <div>
                        <label htmlFor="password">Password</label>
                        <input
                        id="password"
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                        placeholder="••••••••"
                        required
                        disabled={isLoading}
                        />
                    </div>

                    <button 
                        type="submit" 
                        disabled={isLoading} 
                    >
                        {isLoading ? 'Signing In...' : 'Sign In'}
                    </button>
                    </form>
                </div>
                </div>
        </>
    )
}