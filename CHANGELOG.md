v0.7.0
  * Fix interpretation of `raquote` data with alpha channel.
  * Add support for showing overlays on top of images.
  * Use `euclid` for (public) geometric types.
  * Fold consecutive mouse move events to reduce number of events.

v0.6.2
  * Add support for `raqote::DrawTarget` and `raqote::Image`.

v0.6.1
  * Update keyboard-types dependency to 0.5.0.
  * Update sdl2 dependency to 0.33.0.

v0.6.0
  * Add support for handling mouse events.
  * Replace (data, info, name) tuple for displayed images with a struct.

v0.5.1
  * Add `window.add_key_handler`to register asynchronous key handlers.
  * Ignore key events that happened while a window was out of focus.

v0.5.0
  * Add `stop()` function to cleanly stop the background thread.
  * Add `window.get_image()` to retrieve the displayed image.
  * Associate a name with displayed images.
  * Expose `save_image()` and `promp_save_image()`.
  * Fix `window.set_image()` for windows on other workspaces.
  * Fix handling Ctrl+S with modifiers like numlock, capslock, etc.

v0.4.3
  * Add readme to Cargo manifest.

v0.4.2
  * Fix example.
  * Use `assert2` for tests.

v0.4.1
  * Allow end-users to save displayed images.
  * Fix display of color images without alpha channel.

v0.4.0
  * Remove access to `Context` to simplify API.

v0.3.0
  * Change `ImageData` trait to allow consuming images.
  * Implement `ImageData` for tuples of data and `ImageInfo`.
  * Add support for `tch::Tensor`.

v0.2.0
  * Rename `make_window` functions to favor the simple functions.

v0.1.1
  * Support 8-bit grayscale image data.
  * Preserve aspect ratio of images, if requested.
  * Add easy to use API that uses global context.

v0.1.0:
  * Initial release.
