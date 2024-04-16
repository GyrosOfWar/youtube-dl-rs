# Unreleased

# 0.10.0
- feat: Upgrade `reqwest` dependency to 0.12
- fix: Hide `cmd` window on Windows
- feat: Add support for `--cookies-from-browser` and `--max-downloads` options


# 0.9.0
- feat: Add `ignore_errors` to builder and options
- fix: Add niconico_dmc to formats
- BREAKING CHANGE: Split `downloader` feature flag into two (`downloader-native-tls` and `downloader-rustls-tls`)
- feat: Add support for `heatmap` output data
- feat: Add new protocol to enum: `m3u8_native+https` and a fallback value in case none of the variants match.
- feat: Add additional yt-dlp flags (`playlist_reverse`, `date`, `dateafter`, `datebefore`)
- feat: Add new methods `download_to` and `download_to_async` to just download the video(s) to a destination.
- feat: Add new methods `run_raw` and `run_raw_async` to get the JSON output as `serde_json::Value`
- BREAKING CHANGE: Removed `download()` method on `YoutubeDl`, replaced with `download_to()` and `download_to_async()`

# 0.8.1
- feat: add support for `rustls-tls` feature of the transitive reqwest dependency

# 0.8.0
- BREAKING CHANGE: Removed support for youtube-dl. Now only supports `yt-dlp`
- feat: Add helper to download `yt-dlp` programatically with `reqwest`.
- ci: set up GitHub Actions

# 0.7.0
- Added async support via `tokio`, disabled per default. You can opt-in via the `tokio` feature.
- Add feature `yt-dlp` to support yt-dlp

# 0.6.3
- Added cookies + custom args settings

# 0.6.2
- Allow missing `acodec`/`vcodec` fields.
- Added custom parser for format codec fields
- Changed file size to be a float.

# 0.6.1
- Fixed type mismatch for `episode_number` and `season_number`.

# 0.6.0
- Added `search_for` method to utilize the search feature of youtube-dl. Allows specifying the search provider and the number
of desired results.
