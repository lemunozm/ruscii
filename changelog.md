# Changelog

## Current

### Added
- Added Debug trait to Color and Style

### Fixed
- Improved speed.
- Fixed some blink in the top-left corner produced when a key was pressed.

### Removed
- Scaled pencils. Rationale:
  The management is very little intuitive by the user:
  ¿How can reduce the scale is currently is one?,
  ¿How this should be represented in characters?.
  The user should control this behaviour.

## v0.2.0

### Added
- `get_keys_down` method from `keyboard` now gives an ordered time event list of keys.
- `Drawable` trait for custom painting.
- The pencil can now be scaled.
- Default fps to 30 (most terminals render to 30 fps)

### Changes
- Disable Styles, some terminals show issues by the way the styles are computed.
  (This will be fixed in a near future)

## v0.1.2

### Fixed
- Issue with key events. Sometimes the key release event was not computed.

## v0.1.1

### Added
- Added MacOS support

## v0.1.0
- First release
