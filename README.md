# Bug

Rocket resets the connection instead of sending a 413 response.

# Steps to Reproduce

In one terminal, run `cargo run` and in another terminal, run `cargo run --bin client`.

# Expected Behavior

Rocket responds with a 413 error to every request.

# Actual Behavior

Rocket sometimes responds with a 413 error and
sometimes sends a TCP RST without any response.
