import {render, screen, fireEvent} from '@testing-library/react';
import EmailForm from '../EmailForm';
import {describe, it, expect, vi} from 'vitest';
import '@testing-library/jest-dom';


describe('EmailForm', () => {
    it('renders input and submit button', () => {
        render(<EmailForm/>);
        expect(screen.getByLabelText(/email address/i)).toBeInTheDocument();
        expect(screen.getByRole('button', {name: /submit/i})).toBeInTheDocument();
    });

    it('submits email when valid', () => {
        render(<EmailForm/>);
        const input = screen.getByLabelText(/email address/i);
        fireEvent.change(input, {target: {value: 'test@example.com'}});

        const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {
        });
        fireEvent.click(screen.getByRole('button', {name: /submit/i}));

        expect(consoleSpy).toHaveBeenCalledWith('Submitted email:', 'test@example.com');
        consoleSpy.mockRestore();
    });
});
