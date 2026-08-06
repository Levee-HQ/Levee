# Security Policy

## Scope

This policy covers the Levee smart contracts (`contracts/`), the TypeScript SDK (`packages/sdk/`), and the frontend application (`app/`).

## Reporting a Vulnerability

If you discover a security vulnerability in Levee, please report it responsibly.

**Email:** security@levee.finance

**What to include:**
- Description of the vulnerability
- Steps to reproduce
- Impact assessment
- Suggested fix (if any)

**Response timeline:**
- Acknowledgment within 48 hours
- Initial assessment within 5 business days
- Fix timeline communicated within 10 business days

## Current Status

Levee v0 is **pre-alpha and unaudited**. The contracts have not undergone a formal security audit. Do not deposit real funds.

### Known Limitations

1. **Trigger manipulation:** A parametric trigger is itself an attack surface. Reference feed selection and sustain windows need adversarial review.
2. **Oracle dependency:** Levee relies on Reflector price feeds. A compromised oracle could trigger or prevent payouts.
3. **Admin key centralization:** v0 uses a single admin key for all contracts. Multi-sig governance is planned for v1.
4. **No circuit breakers:** There is no mechanism to pause the system in an emergency beyond the admin key.

## Audit Plan

A formal security audit through the Soroban Security Audit Bank is planned for v1, prior to any mainnet deployment.

## Bug Bounty

No formal bug bounty program exists for v0. Responsible disclosures are appreciated and will be credited.
