# redirect-rs

A simple redirecting tool (in Rust 😲😦🦀).
Based on: https://github.com/kodie/redirectmeto

## Use Case

Let's say you're trying to test a new OAuth connection on your local machine with something like Google but they require the redirect URL to have a domain with an approved TLD (because they don't allow things like localhost or .dev). You can use a RedirectMeTo url to redirect to your localhost.

I personally use this for Google OAuth. But I just want it locally.

### Two Deployments:

For this to be useful unfortunately (for my usecase), it has to be deployed somewhere (except your local public IP).

- In src, you can host it on a server (your own). The binary is `redirect-rs`. it uses Actix.
- In api, you can host this repo on Vercel. Automatically will deploy. It uses vercel-rust, instead of Actix.
  - When developing, I was using `vercel dev`.
