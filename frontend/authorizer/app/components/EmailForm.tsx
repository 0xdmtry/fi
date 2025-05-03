'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function EmailForm() {
    const [email, setEmail] = useState('');
    const router = useRouter();

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();

        if (!email.includes('@')) {
            alert('Please enter a valid email');
            return;
        }

        // Navigate to /verify with email as query param
        router.push(`/verify?email=${encodeURIComponent(email)}`);
    };

    return (
        <form onSubmit={handleSubmit} className="max-w-md mx-auto mt-10 p-4 bg-white shadow rounded space-y-4">
            <label htmlFor="email" className="block text-sm font-medium text-gray-700">
                Email Address
            </label>
            <input
                type="email"
                id="email"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-md"
                placeholder="you@example.com"
            />
            <button type="submit" className="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700">
                Submit
            </button>
        </form>
    );
}
