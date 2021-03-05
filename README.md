# bazingabot
Discord bot for community of irl friends

# Instructions

- Install rust compiler (https://www.rust-lang.org/tools/install)
- Checkout main branch
- execute _cargo build release_ in the root directory in order to build release
  - Optionally  build _cargo build_ for debug
- Set necessary tokens to .env
  - DISCORD_TOKEN is mandatory
- Configure logging to .env (https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html)
- run target/release/bazingabot, or target/debug/bazingabot depending on the build type
