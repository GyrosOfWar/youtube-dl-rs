# Unreleased
- Removed `yt-dlp` compile-time feature and made it a runtime switch (`use_yt_dlp()`)
- Implement downloading for both `yt-dlp` and `youtube-dl`

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