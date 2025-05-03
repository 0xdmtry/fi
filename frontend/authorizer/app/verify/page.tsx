'use client';

import { useSearchParams, useRouter } from 'next/navigation';
import PasscodeForm from '../components/PasscodeForm';

export default function VerifyPage() {
    const searchParams = useSearchParams();
    const router = useRouter();
    const email = searchParams.get('email') || '';

    if (!email) {
        router.push('/join');
        return null; // avoid rendering the page with missing email
    }

    return (
        <main className="min-h-screen flex items-center justify-center bg-gray-100">
            <PasscodeForm
                email={email}
                onVerify={(code) => console.log('Verifying code', code)}
                onResend={() => console.log('Resending passcode')}
                onUseAnotherEmail={() => router.push('/join')}
            />
        </main>
    );
}
