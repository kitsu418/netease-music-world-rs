# netease-music-world-rs

<div align="center">
	<img src="images/ferris-ncm.svg" width="283" alt="Ferris NCM ver." />
</div>

> ~~Chrome extension~~ A CLI tool for unlocking overseas access to NetEase Music

This is a minimal CLI tool written in Rust that helps overseas users accessing NetEase Music.

The main approach is inspired by [NetEaseMusicWorldNext](https://github.com/kogamitora/NetEaseMusicWorldNext). It works great, but relying on a browser sucks, especially when you are on a road trip without a laptop and can't use the extension. :)

## Strongly Recommended: Automate with GitHub Actions

Run the CLI from GitHub on a schedule instead of keeping a machine online:

1. **Fork this repository** so the workflow runs under your GitHub account.
2. Log in to [music.163.com](https://music.163.com) in your browser, open DevTools (option + command + I / F12), switch to **Application -> Storage -> Cookies -> https://music.163.com**, and copy the `MUSIC_U` value.
3. In your fork, go to **Settings -> Secrets and variables -> Actions** and create a repository secret named `MUSIC_U` with that value.
4. Keep `.github/workflows/auto-fetch.yml` enabled. The **NetEase Music Auto Fetch** workflow uses `ubuntu-latest`, installs Rust, and executes `cargo run --release -- --music-u="$MUSIC_U"` every eight hours.

**Notes:**
- GitHub pauses scheduled workflows after 90 days without manual runs; re-enable or re-run the workflow occasionally to keep the schedule firing.
- I am not sure how long `MUSIC_U` will expire. If requests start failing, grab a fresh cookie and update the secret. (GitHub will send you an email notification to inform you that the workflow has failed)

## Usage

```bash
# Pass MUSIC_U via CLI argument:
/path/to/netease-music-world-rs --music-u '...'

# Or via environment variable:
export MUSIC_U='...' /path/to/netease-music-world-rs
```

## Build & Run

Make sure that you have installed the Rust toolchains.

```bash
git clone https://github.com/kitsu418/netease-music-world-rs.git
cd netease-music-world-rs
cargo run --release -- --music-u="$MUSIC_U"
```
