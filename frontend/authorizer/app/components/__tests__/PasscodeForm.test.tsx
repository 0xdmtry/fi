import {render, screen, fireEvent} from '@testing-library/react';
import PasscodeForm from '../PasscodeForm';
import {describe, it, expect, vi} from 'vitest';
import '@testing-library/jest-dom';



describe('PasscodeForm', () => {
    const mockVerify = vi.fn();
    const email = 'user@example.com';

    beforeEach(() => {
        mockVerify.mockClear();
        render(<PasscodeForm email={email} onVerify={mockVerify}/>);
    });

    it('renders the email and buttons', () => {
        expect(screen.getByText(email)).toBeInTheDocument();
        expect(screen.getByRole('button', {name: /verify/i})).toBeInTheDocument();
        expect(screen.getByText(/resend the passcode/i)).toBeInTheDocument();
        expect(screen.getByText(/use another email/i)).toBeInTheDocument();
    });

    it('accepts only 4 digits and calls onVerify', () => {
        const input = screen.getByLabelText(/enter 4-digit passcode/i);
        fireEvent.change(input, {target: {value: '1234'}});

        fireEvent.click(screen.getByRole('button', {name: /verify/i}));
        expect(mockVerify).toHaveBeenCalledWith('1234');
    });

    it('rejects invalid passcode', () => {
        const input = screen.getByLabelText(/enter 4-digit passcode/i);
        fireEvent.change(input, {target: {value: 'abcd'}});

        const alertSpy = vi.spyOn(window, 'alert').mockImplementation(() => {
        });
        fireEvent.click(screen.getByRole('button', {name: /verify/i}));
        expect(alertSpy).toHaveBeenCalledWith('Please enter a valid 4-digit passcode');
        alertSpy.mockRestore();
    });
});
