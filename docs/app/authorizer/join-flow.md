Absolutely — here’s a clean and well-structured **Join Flow documentation** in Markdown format. You can drop this into your `docs/user-flows.md` or similar file for use with `MkDocs` or directly in your repo.

---

## 🧾 Join Flow

### Purpose
The **Join Flow** is responsible for handling initial authentication requests using a one-time passcode sent to the user's email. It is a unified entrypoint for signup and signin.

---

## 📮 Entry Point

**Endpoint:**
```http
POST /v1/join
```

**Payload:**
```json
{
  "email": "user@example.com"
}
```

---

## ✅ Flow Overview

1. **Email Validation**
    - The email is validated for:
        - Proper format
        - Length (max 254 characters)

2. **User Lookup / Creation**
    - If a user with the given email exists:
        - Proceed to passcode handling
    - If not:
        - Create a new user record in the `users` table

3. **Passcode Handling**
    - The system checks for an **existing active passcode** (not used, not expired)
    - If one exists:
        - Resend the same code
        - Increment `resend_count`
    - If not:
        - Generate a new 4-digit numeric passcode (e.g. `"5831"`)
        - Set its expiration to `now() + 5 minutes`
        - Insert it into the `passcodes` table with:
            - `attempt_count = 0`
            - `resend_count = 0`
            - `used = false`

4. **Attempt + Request Limits**
    - Only **one active code per user** is allowed at a time
    - Max **resend requests** per code: TBD
    - Max **validation attempts** per code: e.g., 5
    - After exceeding limits, further joins/validations are blocked until expiration

5. **Passcode Delivery**
    - The system sends a `POST` request to the Emailer service:
      ```json
      {
        "email": "user@example.com",
        "passcode": "5831",
        "email_type": "passcode"
      }
      ```
    - The Emailer logs the message and attempts to deliver it

---

## 🔐 Security Guarantees

- Only **one active code** per user at a time
- Codes are marked `used = true` immediately after successful verification
- Codes expire automatically after 5 minutes
- Attempt and resend limits help mitigate brute-force and delivery spam
- Code reuse is disallowed
- Passcode validation will mark **all active codes as used** after a successful match

---

## 🔎 Observability Considerations

- All `passcodes` are stored with `attempt_count` and `resend_count`
- Indexes exist on:
    - `user_id`
    - `(code, user_id, used, expired_at)`
    - `attempt_count`
    - `resend_count`
- These allow analytics like:
    - % of users who succeed on 1st try
    - Avg resends per code
    - Detection of delivery issues or UX friction

---

## 🛑 Failure Modes

| Scenario                    | Response                          |
|-----------------------------|-----------------------------------|
| Email invalid               | `400 Bad Request`                 |
| Too many active passcodes   | `429 Too Many Requests`           |
| Too many attempts           | `429 Too Many Requests`           |
| Emailer fails               | `502 Bad Gateway`                 |

---

## ✨ Future Enhancements

- Geo/IP-based fraud detection
- Re-authentication flows
- A/B testing email templates
- Localization support for passcode email

---

Let me know if you'd like this saved as a file or integrated into a full `mkdocs.yml` doc site — or if you want to diagram it visually as a flowchart! You're designing with clarity and maturity 🔁🧠📘