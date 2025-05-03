'use client';

import {useState} from 'react';

type Props = {
    email: string;
    onVerify?: (passcode: string) => void;
    onResend?: () => void;
    onUseAnotherEmail?: () => void;
};

export default function PasscodeForm({
                                         email,
                                         onVerify,
                                         onResend,
                                         onUseAnotherEmail,
                                     }: Props) {
    const [passcode, setPasscode] = useState('');

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (passcode.length === 4 && /^\d+$/.test(passcode)) {
            onVerify?.(passcode);
        } else {
            alert('Please enter a valid 4-digit passcode');
        }
    };

    return (
        <form
            onSubmit={handleSubmit}
            className="max-w-md mx-auto mt-10 p-4 bg-white shadow rounded space-y-4"
        >
            <div className="text-sm text-gray-600">We sent a passcode to:</div>
            <div className="text-md font-medium text-gray-800">{email}</div>

            <label htmlFor="passcode" className="block text-sm font-medium text-gray-700 mt-4">
                Enter 4-digit passcode
            </label>
            <input
                type="text"
                inputMode="numeric"
                pattern="\d{4}"
                maxLength={4}
                required
                id="passcode"
                name="passcode"
                value={passcode}
                onChange={(e) => setPasscode(e.target.value.replace(/\D/g, ''))}
                className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="1234"
            />

            <button
                type="submit"
                className="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700 transition"
            >
                Verify
            </button>

            <div className="flex flex-col gap-2 text-sm text-blue-600 mt-2">
                <button
                    type="button"
                    onClick={onResend}
                    className="hover:underline"
                >
                    Resend the passcode
                </button>
                <button
                    type="button"
                    onClick={onUseAnotherEmail}
                    className="hover:underline"
                >
                    Use another email
                </button>
            </div>
        </form>
    );
}
