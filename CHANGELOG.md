## Current 0.3.1
- Improvement key support with arrow keys.
- Fixed issue with F keys.

## Release 0.3.0
- Added `Debug` trait to `Color` and `Style`
- Added `draw_at` and `draw` versions for `Drawable` elements.
- Improved speed.
- Fixed some blink in the top-left corner produced when a key was pressed.
- Modified `new` function of Vec2 by `zero` with more meaning.
- Removed scale option from pencils. Rationale:
  The management is very little intuitive by the user:
  How can reduce the scale if it is currently one?,
  How this should be represented in characters?.
  The user should control this behaviour.

## Release 0.2.0
- Added `get_keys_down` method from `keyboard` now gives an ordered time event list of keys.
- Added `Drawable` trait for custom painting.
- Added scale option to pencils.
- Change default fps to 30 (most terminals render to 30 fps)
- Disable Styles, some terminals show issues by the way the styles are computed.
  (This will be fixed in a near future)

## Release 0.1.2
- Fixed issue with key events. Sometimes the key release event was not computed.

## Release 0.1.1
- Added MacOS support

## Release 0.1.0
- First release
