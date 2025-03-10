//! `style` contains the primitives used to control how your user interface will look.
//!
//! There are two ways to set styles:
//! - Creating and using the [`Style`] struct. (e.g. `Style::new().fg(Color::Red)`).
//! - Using style shorthands. (e.g. `"hello".red()`).
//!
//! # Using the `Style` struct
//!
//! This is the original approach to styling and likely the most common. This is useful when
//! creating style variables to reuse, however the shorthands are often more convenient and
//! readable for most use cases.
//!
//! ## Example
//!
//! ```
//! use ratatui::prelude::*;
//!
//! let heading_style = Style::new()
//!     .fg(Color::Black)
//!     .bg(Color::Green)
//!     .add_modifier(Modifier::ITALIC | Modifier::BOLD);
//! let span = Span::styled("hello", heading_style);
//! ```
//!
//! # Using style shorthands
//!
//! Originally Ratatui only had the ability to set styles using the `Style` struct. This is still
//! supported, but there are now shorthands for all the styles that can be set. These save you from
//! having to create a `Style` struct every time you want to set a style.
//!
//! The shorthands are implemented in the [`Stylize`] trait which is automatically implemented for
//! many types via the [`Styled`] trait. This means that you can use the shorthands on any type
//! that implements [`Styled`]. E.g.:
//! - Strings and string slices when styled return a [`Span`]
//! - [`Span`]s can be styled again, which will merge the styles.
//! - Many widget types can be styled directly rather than calling their style() method.
//!
//! See the [`Stylize`] and [`Styled`] traits for more information. These traits are re-exported in
//! the [`prelude`] module for convenience.
//!
//! ## Example
//!
//! ```
//! use ratatui::{prelude::*, widgets::*};
//!
//! assert_eq!(
//!     "hello".red().on_blue().bold(),
//!     Span::styled(
//!         "hello",
//!         Style::default()
//!             .fg(Color::Red)
//!             .bg(Color::Blue)
//!             .add_modifier(Modifier::BOLD)
//!     )
//! );
//!
//! assert_eq!(
//!     Paragraph::new("hello").red().on_blue().bold(),
//!     Paragraph::new("hello").style(
//!         Style::default()
//!             .fg(Color::Red)
//!             .bg(Color::Blue)
//!             .add_modifier(Modifier::BOLD)
//!     )
//! );
//! ```
//!
//! [`prelude`]: crate::prelude
//! [`Span`]: crate::text::Span

use std::fmt::{self, Debug};

use bitflags::bitflags;

mod stylize;
pub use stylize::{Styled, Stylize};
mod color;
pub use color::Color;

bitflags! {
    /// Modifier changes the way a piece of text is displayed.
    ///
    /// They are bitflags so they can easily be composed.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ratatui::{prelude::*};
    ///
    /// let m = Modifier::BOLD | Modifier::ITALIC;
    /// ```
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const UNDERLINED        = 0b0000_0000_1000;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

/// Implement the `Debug` trait for `Modifier` manually.
///
/// This will avoid printing the empty modifier as 'Borders(0x0)' and instead print it as 'NONE'.
impl fmt::Debug for Modifier {
    /// Format the modifier as `NONE` if the modifier is empty or as a list of flags separated by
    /// `|` otherwise.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Style lets you control the main characteristics of the displayed elements.
///
/// ```rust
/// use ratatui::prelude::*;
///
/// Style::default()
///     .fg(Color::Black)
///     .bg(Color::Green)
///     .add_modifier(Modifier::ITALIC | Modifier::BOLD);
/// ```
///
/// Styles can also be created with a [shorthand notation](crate::style#using-style-shorthands).
///
/// ```rust
/// # use ratatui::prelude::*;
/// Style::new().black().on_green().italic().bold();
/// ```
///
/// For more information about the style shorthands, see the [`Stylize`] trait.
///
/// Styles represents an incremental change. If you apply the styles S1, S2, S3 to a cell of the
/// terminal buffer, the style of this cell will be the result of the merge of S1, S2 and S3, not
/// just S3.
///
/// ```rust
/// use ratatui::prelude::*;
///
/// let styles = [
///     Style::default()
///         .fg(Color::Blue)
///         .add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::default()
///         .bg(Color::Red)
///         .add_modifier(Modifier::UNDERLINED),
///     #[cfg(feature = "underline-color")]
///     Style::default().underline_color(Color::Green),
///     Style::default()
///         .fg(Color::Yellow)
///         .remove_modifier(Modifier::ITALIC),
/// ];
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
/// for style in &styles {
///     buffer.get_mut(0, 0).set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Red),
///         #[cfg(feature = "underline-color")]
///         underline_color: Some(Color::Green),
///         add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
///         sub_modifier: Modifier::empty(),
///     },
///     buffer.get(0, 0).style(),
/// );
/// ```
///
/// The default implementation returns a `Style` that does not modify anything. If you wish to
/// reset all properties until that point use [`Style::reset`].
///
/// ```
/// use ratatui::prelude::*;
///
/// let styles = [
///     Style::default()
///         .fg(Color::Blue)
///         .add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::reset().fg(Color::Yellow),
/// ];
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
/// for style in &styles {
///     buffer.get_mut(0, 0).set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Reset),
///         #[cfg(feature = "underline-color")]
///         underline_color: Some(Color::Reset),
///         add_modifier: Modifier::empty(),
///         sub_modifier: Modifier::empty(),
///     },
///     buffer.get(0, 0).style(),
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    #[cfg(feature = "underline-color")]
    pub underline_color: Option<Color>,
    pub add_modifier: Modifier,
    pub sub_modifier: Modifier,
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

impl Styled for Style {
    type Item = Style;

    fn style(&self) -> Style {
        *self
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.patch(style)
    }
}
impl Style {
    pub const fn new() -> Style {
        Style {
            fg: None,
            bg: None,
            #[cfg(feature = "underline-color")]
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    /// Returns a `Style` resetting all properties.
    pub const fn reset() -> Style {
        Style {
            fg: Some(Color::Reset),
            bg: Some(Color::Reset),
            #[cfg(feature = "underline-color")]
            underline_color: Some(Color::Reset),
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::all(),
        }
    }

    /// Changes the foreground color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::default().fg(Color::Blue);
    /// let diff = Style::default().fg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().fg(Color::Red));
    /// ```
    #[must_use = "`fg` returns the modified style without modifying the original"]
    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    /// Changes the background color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::default().bg(Color::Blue);
    /// let diff = Style::default().bg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().bg(Color::Red));
    /// ```
    #[must_use = "`bg` returns the modified style without modifying the original"]
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    /// Changes the underline color. The text must be underlined with a modifier for this to work.
    ///
    /// This uses a non-standard ANSI escape sequence. It is supported by most terminal emulators,
    /// but is only implemented in the crossterm backend and enabled by the `underline-color`
    /// feature flag.
    ///
    /// See
    /// [Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
    /// code `58` and `59` for more information.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::default()
    ///     .underline_color(Color::Blue)
    ///     .add_modifier(Modifier::UNDERLINED);
    /// let diff = Style::default()
    ///     .underline_color(Color::Red)
    ///     .add_modifier(Modifier::UNDERLINED);
    /// assert_eq!(
    ///     style.patch(diff),
    ///     Style::default()
    ///         .underline_color(Color::Red)
    ///         .add_modifier(Modifier::UNDERLINED)
    /// );
    /// ```
    #[cfg(feature = "underline-color")]
    #[must_use = "`underline_color` returns the modified style without modifying the original"]
    pub const fn underline_color(mut self, color: Color) -> Style {
        self.underline_color = Some(color);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it adds the given modifier to the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::default().add_modifier(Modifier::BOLD);
    /// let diff = Style::default().add_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD | Modifier::ITALIC);
    /// assert_eq!(patched.sub_modifier, Modifier::empty());
    /// ```
    #[must_use = "`add_modifier` returns the modified style without modifying the original"]
    pub const fn add_modifier(mut self, modifier: Modifier) -> Style {
        self.sub_modifier = self.sub_modifier.difference(modifier);
        self.add_modifier = self.add_modifier.union(modifier);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it removes the given modifier from the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC);
    /// let diff = Style::default().remove_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD);
    /// assert_eq!(patched.sub_modifier, Modifier::ITALIC);
    /// ```
    #[must_use = "`remove_modifier` returns the modified style without modifying the original"]
    pub const fn remove_modifier(mut self, modifier: Modifier) -> Style {
        self.add_modifier = self.add_modifier.difference(modifier);
        self.sub_modifier = self.sub_modifier.union(modifier);
        self
    }

    /// Results in a combined style that is equivalent to applying the two individual styles to
    /// a style one after the other.
    ///
    /// ## Examples
    /// ```
    /// # use ratatui::prelude::*;
    /// let style_1 = Style::default().fg(Color::Yellow);
    /// let style_2 = Style::default().bg(Color::Red);
    /// let combined = style_1.patch(style_2);
    /// assert_eq!(
    ///     Style::default().patch(style_1).patch(style_2),
    ///     Style::default().patch(combined)
    /// );
    /// ```
    #[must_use = "`patch` returns the modified style without modifying the original"]
    pub fn patch(mut self, other: Style) -> Style {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        #[cfg(feature = "underline-color")]
        {
            self.underline_color = other.underline_color.or(self.underline_color);
        }

        self.add_modifier.remove(other.sub_modifier);
        self.add_modifier.insert(other.add_modifier);
        self.sub_modifier.remove(other.add_modifier);
        self.sub_modifier.insert(other.sub_modifier);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn styles() -> Vec<Style> {
        vec![
            Style::default(),
            Style::default().fg(Color::Yellow),
            Style::default().bg(Color::Yellow),
            Style::default().add_modifier(Modifier::BOLD),
            Style::default().remove_modifier(Modifier::BOLD),
            Style::default().add_modifier(Modifier::ITALIC),
            Style::default().remove_modifier(Modifier::ITALIC),
            Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD),
            Style::default().remove_modifier(Modifier::ITALIC | Modifier::BOLD),
        ]
    }

    #[test]
    fn combined_patch_gives_same_result_as_individual_patch() {
        let styles = styles();
        for &a in &styles {
            for &b in &styles {
                for &c in &styles {
                    for &d in &styles {
                        let combined = a.patch(b.patch(c.patch(d)));

                        assert_eq!(
                            Style::default().patch(a).patch(b).patch(c).patch(d),
                            Style::default().patch(combined)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn combine_individual_modifiers() {
        use crate::{buffer::Buffer, layout::Rect};

        let mods = vec![
            Modifier::BOLD,
            Modifier::DIM,
            Modifier::ITALIC,
            Modifier::UNDERLINED,
            Modifier::SLOW_BLINK,
            Modifier::RAPID_BLINK,
            Modifier::REVERSED,
            Modifier::HIDDEN,
            Modifier::CROSSED_OUT,
        ];

        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));

        for m in &mods {
            buffer.get_mut(0, 0).set_style(Style::reset());
            buffer
                .get_mut(0, 0)
                .set_style(Style::default().add_modifier(*m));
            let style = buffer.get(0, 0).style();
            assert!(style.add_modifier.contains(*m));
            assert!(!style.sub_modifier.contains(*m));
        }
    }

    #[test]
    fn modifier_debug() {
        assert_eq!(format!("{:?}", Modifier::empty()), "NONE");
        assert_eq!(format!("{:?}", Modifier::BOLD), "BOLD");
        assert_eq!(format!("{:?}", Modifier::DIM), "DIM");
        assert_eq!(format!("{:?}", Modifier::ITALIC), "ITALIC");
        assert_eq!(format!("{:?}", Modifier::UNDERLINED), "UNDERLINED");
        assert_eq!(format!("{:?}", Modifier::SLOW_BLINK), "SLOW_BLINK");
        assert_eq!(format!("{:?}", Modifier::RAPID_BLINK), "RAPID_BLINK");
        assert_eq!(format!("{:?}", Modifier::REVERSED), "REVERSED");
        assert_eq!(format!("{:?}", Modifier::HIDDEN), "HIDDEN");
        assert_eq!(format!("{:?}", Modifier::CROSSED_OUT), "CROSSED_OUT");
        assert_eq!(
            format!("{:?}", Modifier::BOLD | Modifier::DIM),
            "BOLD | DIM"
        );
        assert_eq!(
            format!("{:?}", Modifier::all()),
            "BOLD | DIM | ITALIC | UNDERLINED | SLOW_BLINK | RAPID_BLINK | REVERSED | HIDDEN | CROSSED_OUT"
        );
    }

    #[test]
    fn style_can_be_const() {
        const RED: Color = Color::Red;
        const BLACK: Color = Color::Black;
        const BOLD: Modifier = Modifier::BOLD;
        const ITALIC: Modifier = Modifier::ITALIC;

        const _RESET: Style = Style::reset();
        const _RED_FG: Style = Style::new().fg(RED);
        const _BLACK_BG: Style = Style::new().bg(BLACK);
        const _ADD_BOLD: Style = Style::new().add_modifier(BOLD);
        const _REMOVE_ITALIC: Style = Style::new().remove_modifier(ITALIC);
        const ALL: Style = Style::new()
            .fg(RED)
            .bg(BLACK)
            .add_modifier(BOLD)
            .remove_modifier(ITALIC);
        assert_eq!(
            ALL,
            Style::new()
                .fg(Color::Red)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::ITALIC)
        )
    }

    #[test]
    fn style_can_be_stylized() {
        // foreground colors
        assert_eq!(Style::new().black(), Style::new().fg(Color::Black));
        assert_eq!(Style::new().red(), Style::new().fg(Color::Red));
        assert_eq!(Style::new().green(), Style::new().fg(Color::Green));
        assert_eq!(Style::new().yellow(), Style::new().fg(Color::Yellow));
        assert_eq!(Style::new().blue(), Style::new().fg(Color::Blue));
        assert_eq!(Style::new().magenta(), Style::new().fg(Color::Magenta));
        assert_eq!(Style::new().cyan(), Style::new().fg(Color::Cyan));
        assert_eq!(Style::new().white(), Style::new().fg(Color::White));
        assert_eq!(Style::new().gray(), Style::new().fg(Color::Gray));
        assert_eq!(Style::new().dark_gray(), Style::new().fg(Color::DarkGray));
        assert_eq!(Style::new().light_red(), Style::new().fg(Color::LightRed));
        assert_eq!(
            Style::new().light_green(),
            Style::new().fg(Color::LightGreen)
        );
        assert_eq!(
            Style::new().light_yellow(),
            Style::new().fg(Color::LightYellow)
        );
        assert_eq!(Style::new().light_blue(), Style::new().fg(Color::LightBlue));
        assert_eq!(
            Style::new().light_magenta(),
            Style::new().fg(Color::LightMagenta)
        );
        assert_eq!(Style::new().light_cyan(), Style::new().fg(Color::LightCyan));
        assert_eq!(Style::new().white(), Style::new().fg(Color::White));

        // Background colors
        assert_eq!(Style::new().on_black(), Style::new().bg(Color::Black));
        assert_eq!(Style::new().on_red(), Style::new().bg(Color::Red));
        assert_eq!(Style::new().on_green(), Style::new().bg(Color::Green));
        assert_eq!(Style::new().on_yellow(), Style::new().bg(Color::Yellow));
        assert_eq!(Style::new().on_blue(), Style::new().bg(Color::Blue));
        assert_eq!(Style::new().on_magenta(), Style::new().bg(Color::Magenta));
        assert_eq!(Style::new().on_cyan(), Style::new().bg(Color::Cyan));
        assert_eq!(Style::new().on_white(), Style::new().bg(Color::White));
        assert_eq!(Style::new().on_gray(), Style::new().bg(Color::Gray));
        assert_eq!(
            Style::new().on_dark_gray(),
            Style::new().bg(Color::DarkGray)
        );
        assert_eq!(
            Style::new().on_light_red(),
            Style::new().bg(Color::LightRed)
        );
        assert_eq!(
            Style::new().on_light_green(),
            Style::new().bg(Color::LightGreen)
        );
        assert_eq!(
            Style::new().on_light_yellow(),
            Style::new().bg(Color::LightYellow)
        );
        assert_eq!(
            Style::new().on_light_blue(),
            Style::new().bg(Color::LightBlue)
        );
        assert_eq!(
            Style::new().on_light_magenta(),
            Style::new().bg(Color::LightMagenta)
        );
        assert_eq!(
            Style::new().on_light_cyan(),
            Style::new().bg(Color::LightCyan)
        );
        assert_eq!(Style::new().on_white(), Style::new().bg(Color::White));

        // Add Modifiers
        assert_eq!(
            Style::new().bold(),
            Style::new().add_modifier(Modifier::BOLD)
        );
        assert_eq!(Style::new().dim(), Style::new().add_modifier(Modifier::DIM));
        assert_eq!(
            Style::new().italic(),
            Style::new().add_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::new().underlined(),
            Style::new().add_modifier(Modifier::UNDERLINED)
        );
        assert_eq!(
            Style::new().slow_blink(),
            Style::new().add_modifier(Modifier::SLOW_BLINK)
        );
        assert_eq!(
            Style::new().rapid_blink(),
            Style::new().add_modifier(Modifier::RAPID_BLINK)
        );
        assert_eq!(
            Style::new().reversed(),
            Style::new().add_modifier(Modifier::REVERSED)
        );
        assert_eq!(
            Style::new().hidden(),
            Style::new().add_modifier(Modifier::HIDDEN)
        );
        assert_eq!(
            Style::new().crossed_out(),
            Style::new().add_modifier(Modifier::CROSSED_OUT)
        );

        // Remove Modifiers
        assert_eq!(
            Style::new().not_bold(),
            Style::new().remove_modifier(Modifier::BOLD)
        );
        assert_eq!(
            Style::new().not_dim(),
            Style::new().remove_modifier(Modifier::DIM)
        );
        assert_eq!(
            Style::new().not_italic(),
            Style::new().remove_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::new().not_underlined(),
            Style::new().remove_modifier(Modifier::UNDERLINED)
        );
        assert_eq!(
            Style::new().not_slow_blink(),
            Style::new().remove_modifier(Modifier::SLOW_BLINK)
        );
        assert_eq!(
            Style::new().not_rapid_blink(),
            Style::new().remove_modifier(Modifier::RAPID_BLINK)
        );
        assert_eq!(
            Style::new().not_reversed(),
            Style::new().remove_modifier(Modifier::REVERSED)
        );
        assert_eq!(
            Style::new().not_hidden(),
            Style::new().remove_modifier(Modifier::HIDDEN)
        );
        assert_eq!(
            Style::new().not_crossed_out(),
            Style::new().remove_modifier(Modifier::CROSSED_OUT)
        );

        // reset
        assert_eq!(Style::new().reset(), Style::reset());
    }
}
