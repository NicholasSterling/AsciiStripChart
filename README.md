# AsciiStripChart
Rust crate for ASCII strip chart facility

Degrees of freedom to consider:
 * the numeric type
   * u8, u16, u32, ...   (think small embedded systems)
   * f32, f64
 * how the range is interpreted; e.g. for a range of 0..10 do you want
   * -0.5 to 10.5   on 11 columns    0 1 2 3 .. 10        |_0__1__2__3__4__5__6__7__8__9__A_|
   * [0..10)        on 10 columns (does not include 10)   |0__1__2__3__4__5__6__7__8__9__|
   * (0..10]        on 10 columns (does not include 0)    |__1__2__3__4__5__6__7__8__9__A|
 * How out-of-bounds values are handled
   * special mark(s) at edge
   * special mark in gutter
 * How colliding marks are handled
   * last mark on top
   * special collision mark(s)
 * the display/canvas
   * vertically scrolling terminal emulator
   * horizontally scrolling terminal emulator
   * overwriting chart via curses (erase recent mark and draw new mark)
   * color
 * the mark type
   * a character
   * an image (if the display type supports it)
   * an enum value representing a mark to the display