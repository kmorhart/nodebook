import { useState } from "react";
import React from 'react';
import { CONFIG } from "../utils/config";

export default function Register() {
    const [email, setEmail] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    
    // Status states
    const [message, setMessage] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
        e.preventDefault();
        setIsLoading(true);

        // Reset status messages on new submission
        setMessage('');

        try {
            const response = await fetch(CONFIG.AUTH_URL + '/register', {
                method: 'POST',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ 
                    email: email,
                    username: username,
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
                        <label htmlFor="email" >Email Address</label>
                        <input
                        id="email"
                        type="text"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                        placeholder="you@domain.com"
                        required
                        disabled={isLoading}
                        />
                    </div>
                    <div>
                        <label htmlFor="username">Username</label>
                        <input
                        id="username"
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        placeholder="your_username"
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
                        {isLoading ? 'Creating Account...' : 'Create Account'}
                    </button>
                    </form>
                </div>
                </div>
        </>
    )
}