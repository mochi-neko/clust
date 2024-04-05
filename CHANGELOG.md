# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support `Future` for `clust::messages::AsyncTool` by `clust::attributes::clust_tool`.

## [0.9.0] - 2024-04-XX

### Added

- Add the beta feature header for the `clust::Client`.

## [0.8.0] - 2024-04-04

### Added

- Add builder pattern for `clust::messages::MessagesRequestBody`: `clust::messages::MessagesRequestBodyBuilder`.
- Add `clust::attributes` for attribute macros.

### Changed

- Rename builder methods of `clust::ClientBuilder`.

## [0.7.0] - 2024-04-03

### Added

- Support function calling.
- Add the attribute macro `clust::clust_macros::clust_tool` to use function calling easily via `clust::messages::Tool`
  or `clust::messages::AsyncTool`.

### Changed

- Remove type aliases for `std::result::Result` and explicitly specify generics.

## [0.6.0] - 2024-03-24

### Added

- Add text flattening method for `Content`: `Content::flatten_into_text()`. <- (#1)
- Add image source flattening method for `Content`: `Content::flatten_into_image_source()`.
- Add assistant message creation method for `MessagesResponseBody`: `MessagesResponseBody::create_message()`.
- Add builder pattern for `Client`: `ClientBuilder`.
- Add an example to create a message with vision.
- Add an example to converse with the assistant.

### Changed

- Rename `StreamChunk` to `MessageChunk`.
- Rename `Content::MultipleBlock` to `Content::MultipleBlocks`.
- Improve initialization and conversion methods for `Content` and `Message`.

### Fixed

- Fix default value of the `role` field in the `MessagesResponseBody` from `Role::user` to `Role::assistant`.

## [0.5.0] - 2024-03-18

### Changed

- Improve crate dependencies.

### Removed

- Abolish the feature flag: `tokio_stream`.

## [0.4.0] - 2024-03-15

### Added

- Support streaming API with `tokio` backend by optional.

## [0.3.0] - 2024-03-14

### Added

- Add the Claude 3 Haiku model: `claude-3-haiku-20240307`.

## [0.2.0] - 2024-03-13

### Added

- Add the `Streaming Messages` API.

### Changed

- Improve type annotation of the `type` field in the `MessagesRespponseBody`.

## [0.1.0] - 2024-03-12

### Added

- Add the `Create a Message` API.

[unreleased]: https://github.com/mochi-neko/clust/compare/v0.8.0...HEAD

[0.8.0]: https://github.com/mochi-neko/clust/compare/v0.7.0...v0.8.0

[0.7.0]: https://github.com/mochi-neko/clust/compare/v0.6.0...v0.7.0

[0.6.0]: https://github.com/mochi-neko/clust/compare/v0.5.0...v0.6.0

[0.5.0]: https://github.com/mochi-neko/clust/compare/v0.4.0...v0.5.0

[0.4.0]: https://github.com/mochi-neko/clust/compare/v0.3.0...v0.4.0

[0.3.0]: https://github.com/mochi-neko/clust/compare/v0.2.0...v0.3.0

[0.2.0]: https://github.com/mochi-neko/clust/compare/v0.1.0...v0.2.0

[0.1.0]: https://github.com/mochi-neko/clust/releases/tag/v0.1.0
