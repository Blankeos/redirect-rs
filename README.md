# redirect-rs

A simple redirecting tool (in Rust 😲😦🦀).
Based on: https://github.com/kodie/redirectmeto

## Use Case

Let's say you're trying to test a new OAuth connection on your local machine with something like Google but they require the redirect URL to have a domain with an approved TLD (because they don't allow things like localhost or .dev). You can use a RedirectMeTo url to redirect to your localhost.

I personally use this for Google OAuth. But I just want it locally.
