use iced::{application, Color};

use crate::gui::theme::color::ColorExt;

pub mod button;
pub mod color;
pub mod container;
pub mod icon;
pub mod text;
pub mod textinput;
pub mod widget;
pub mod picklist;
pub mod menu;
pub mod scrollable;
mod canvas;
pub mod color_picker;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Transparent
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub primary: Color,
    pub primary_container: Color,
    pub on_primary: Color,
    pub on_primary_container: Color,
    pub inverse_primary: Color,
    pub secondary: Color,
    pub secondary_container: Color,
    pub on_secondary: Color,
    pub on_secondary_container: Color,
    pub tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary: Color,
    pub on_tertiary_container: Color,
    pub surface: Color,
    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub surface_variant: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,
    pub background: Color,
    pub on_background: Color,
    pub error: Color,
    pub error_container: Color,
    pub on_error: Color,
    pub on_error_container: Color,
    pub success: Color,
    pub on_success: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub shadow: Color,
    pub surface_tint: Color,
    pub scrim: Color,
    pub button_color_primary: Color,
    pub button_color_danger: Color,
    pub button_color_default: Color,
}

impl Palette {
    pub fn light() -> Self {
        Self {
            primary: Color::from_hex("6750A4"),
            primary_container: Color::from_hex("EADDFF"),
            on_primary: Color::from_hex("FFFFFF"),
            on_primary_container: Color::from_hex("21005E"),
            inverse_primary: Color::from_hex("D0BCFF"),
            secondary: Color::from_hex("625B71"),
            secondary_container: Color::from_hex("E8DEF8"),
            on_secondary: Color::from_hex("FFFFFF"),
            on_secondary_container: Color::from_hex("1E192B"),
            tertiary: Color::from_hex("7D5260"),
            tertiary_container: Color::from_hex("FFD8E4"),
            on_tertiary: Color::from_hex("FFFFFF"),
            on_tertiary_container: Color::from_hex("370B1E"),
            surface: Color::from_hex("FEF7FF"),
            surface_dim: Color::from_hex("DED8E1"),
            surface_bright: Color::from_hex("FEF7FF"),
            surface_container_lowest: Color::from_hex("FFFFFF"),
            surface_container_low: Color::from_hex("F7F2FA"),
            surface_container: Color::from_hex("F3EDF7"),
            surface_container_high: Color::from_hex("ECE6F0"),
            surface_container_highest: Color::from_hex("E6E0E9"),
            surface_variant: Color::from_hex("E7E0EC"),
            on_surface: Color::from_hex("1C1B1F"),
            on_surface_variant: Color::from_hex("49454E"),
            inverse_surface: Color::from_hex("313033"),
            inverse_on_surface: Color::from_hex("F4EFF4"),
            background: Color::from_hex("FEF7FF"),
            on_background: Color::from_hex("1C1B1F"),
            error: Color::from_hex("B3261E"),
            error_container: Color::from_hex("F9DEDC"),
            on_error: Color::from_hex("FFFFFF"),
            on_error_container: Color::from_hex("410E0B"),
            success: Color::from_hex("486726"),
            on_success: Color::from_hex("FFFFFF"),
            outline: Color::from_hex("79747E"),
            outline_variant: Color::from_hex("C4C7C5"),
            shadow: Color::from_hex("000000"),
            surface_tint: Color::from_hex("6750A4"),
            scrim: Color::from_hex("000000"),
            //ours
            button_color_primary: Color::from_rgb(0.87, 0.76, 0.91),
            button_color_danger: Color::from_rgb(255.0, 0.0, 0.0),
            button_color_default: Color::from_rgb(0.83, 0.83, 0.83),
        }
    }
    pub fn transparent() -> Self {
        Self {
            primary: Color::from_hex("D0BCFF"),
            primary_container: Color::from_hex("4F378B"),
            on_primary: Color::from_hex("371E73"),
            on_primary_container: Color::from_hex("EADDFF"),
            inverse_primary: Color::from_hex("6750A4"),
            secondary: Color::from_hex("CCC2DC"),
            secondary_container: Color::from_hex("4A4458"),
            on_secondary: Color::from_hex("332D41"),
            on_secondary_container: Color::from_hex("E8DEF8"),
            tertiary: Color::from_hex("EFB8C8"),
            tertiary_container: Color::from_hex("633B48"),
            on_tertiary: Color::from_hex("492532"),
            on_tertiary_container: Color::from_hex("FFD8E4"),
            surface: Color::from_hex("141218"),
            surface_dim: Color::from_hex("141218"),
            surface_bright: Color::from_hex("3B383E"),
            surface_container_lowest: Color::from_hex("0F0D13"),
            surface_container_low: Color::from_hex("1D1B20"),
            surface_container: Color::from_hex("211F26"),
            surface_container_high: Color::from_hex("2B2930"),
            surface_container_highest: Color::from_hex("36343B"),
            surface_variant: Color::from_hex("49454F"),
            on_surface: Color::from_hex("E6E1E5"),
            on_surface_variant: Color::from_hex("CAC4D0"),
            inverse_surface: Color::from_hex("E6E1E5"),
            inverse_on_surface: Color::from_hex("313033"),

            // Rendiamo la finestra trasparente
            background: Color::from_rgba(0.0, 0.0, 0.0, 0.0),  // Trasparente
            on_background: Color::from_hex("E6E1E5"),

            error: Color::from_hex("F2B8B5"),
            error_container: Color::from_hex("8C1D18"),
            on_error: Color::from_hex("601410"),
            on_error_container: Color::from_hex("F9DEDC"),
            success: Color::from_hex("ADD284"),
            on_success: Color::from_hex("1D3700"),
            outline: Color::from_hex("938F99"),
            outline_variant: Color::from_hex("444746"),
            shadow: Color::from_hex("000000"),
            surface_tint: Color::from_hex("D0BCFF"),
            scrim: Color::from_hex("000000"),
            button_color_primary: Color::from_rgb(0.73, 0.56, 0.76),
            button_color_danger: Color::from_rgb(255.0, 0.0, 0.0),
            button_color_default: Color::from_rgb(0.53, 0.53, 0.53),
        }
    }
    pub fn dark() -> Self {
        Self {
            primary: Color::from_hex("D0BCFF"),
            primary_container: Color::from_hex("4F378B"),
            on_primary: Color::from_hex("371E73"),
            on_primary_container: Color::from_hex("EADDFF"),
            inverse_primary: Color::from_hex("6750A4"),
            secondary: Color::from_hex("CCC2DC"),
            secondary_container: Color::from_hex("4A4458"),
            on_secondary: Color::from_hex("332D41"),
            on_secondary_container: Color::from_hex("E8DEF8"),
            tertiary: Color::from_hex("EFB8C8"),
            tertiary_container: Color::from_hex("633B48"),
            on_tertiary: Color::from_hex("492532"),
            on_tertiary_container: Color::from_hex("FFD8E4"),
            surface: Color::from_hex("141218"),
            surface_dim: Color::from_hex("141218"),
            surface_bright: Color::from_hex("3B383E"),
            surface_container_lowest: Color::from_hex("0F0D13"),
            surface_container_low: Color::from_hex("1D1B20"),
            surface_container: Color::from_hex("211F26"),
            surface_container_high: Color::from_hex("2B2930"),
            surface_container_highest: Color::from_hex("36343B"),
            surface_variant: Color::from_hex("49454F"),
            on_surface: Color::from_hex("E6E1E5"),
            on_surface_variant: Color::from_hex("CAC4D0"),
            inverse_surface: Color::from_hex("E6E1E5"),
            inverse_on_surface: Color::from_hex("313033"),
            background: Color::from_hex("141218"),
            on_background: Color::from_hex("E6E1E5"),
            error: Color::from_hex("F2B8B5"),
            error_container: Color::from_hex("8C1D18"),
            on_error: Color::from_hex("601410"),
            on_error_container: Color::from_hex("F9DEDC"),
            success: Color::from_hex("ADD284"),
            on_success: Color::from_hex("1D3700"),
            outline: Color::from_hex("938F99"),
            outline_variant: Color::from_hex("444746"),
            shadow: Color::from_hex("000000"),
            surface_tint: Color::from_hex("D0BCFF"),
            scrim: Color::from_hex("000000"),
            button_color_primary: Color::from_rgb(0.73, 0.56, 0.76),
            button_color_danger: Color::from_rgb(255.0, 0.0, 0.0),
            button_color_default: Color::from_rgb(0.53, 0.53, 0.53),
        }
    }

    pub fn get_palette_color(self, palette_color: &PaletteColor) -> Color {
        use PaletteColor::*;
        match palette_color {
            Primary => self.primary,
            PrimaryContainer => self.primary_container,
            OnPrimary => self.on_primary,
            OnPrimaryContainer => self.on_primary_container,
            InversePrimary => self.inverse_primary,
            Secondary => self.secondary,
            SecondaryContainer => self.secondary_container,
            OnSecondary => self.on_secondary,
            OnSecondaryContainer => self.on_secondary_container,
            Tertiary => self.tertiary,
            TertiaryContainer => self.tertiary_container,
            OnTertiary => self.on_tertiary,
            OnTertiaryContainer => self.on_tertiary_container,
            Surface => self.surface,
            SurfaceDim => self.surface_dim,
            SurfaceBright => self.surface_bright,
            SurfaceContainerLowest => self.surface_container_lowest,
            SurfaceContainerLow => self.surface_container_low,
            SurfaceContainer => self.surface_container,
            SurfaceContainerHigh => self.surface_container_high,
            SurfaceContainerHighest => self.surface_container_highest,
            SurfaceVariant => self.surface_variant,
            OnSurface => self.on_surface,
            OnSurfaceVariant => self.on_surface_variant,
            InverseSurface => self.inverse_surface,
            InverseOnSurface => self.inverse_on_surface,
            Background => self.background,
            OnBackground => self.on_background,
            Error => self.error,
            ErrorContainer => self.error_container,
            OnError => self.on_error,
            OnErrorContainer => self.on_error_container,
            Success => self.success,
            OnSuccess => self.on_success,
            Outline => self.outline,
            OutlineVariant => self.outline_variant,
            Shadow => self.shadow,
            SurfaceTint => self.surface_tint,
            Scrim => self.scrim,
        }
    }
}

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Self::Light => Palette::light(),
            Self::Dark => Palette::dark(),
            Self::Transparent => Palette::transparent()
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _: &Self::Style) -> application::Appearance {
        let palette = self.palette();

        application::Appearance {
            background_color: palette.background,
            text_color: palette.on_background,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaletteColor {
    Primary,
    PrimaryContainer,
    OnPrimary,
    OnPrimaryContainer,
    InversePrimary,
    Secondary,
    SecondaryContainer,
    OnSecondary,
    OnSecondaryContainer,
    Tertiary,
    TertiaryContainer,
    OnTertiary,
    OnTertiaryContainer,
    Surface,
    SurfaceDim,
    SurfaceBright,
    SurfaceContainerLowest,
    SurfaceContainerLow,
    SurfaceContainer,
    SurfaceContainerHigh,
    SurfaceContainerHighest,
    SurfaceVariant,
    OnSurface,
    OnSurfaceVariant,
    InverseSurface,
    InverseOnSurface,
    Background,
    OnBackground,
    Error,
    ErrorContainer,
    OnError,
    OnErrorContainer,
    Success,
    OnSuccess,
    Outline,
    OutlineVariant,
    Shadow,
    SurfaceTint,
    Scrim,
}

impl PaletteColor {
    pub fn on(self) -> PaletteColor {
        use PaletteColor::*;
        match self {
            Primary => OnPrimary,
            PrimaryContainer => OnPrimaryContainer,
            Secondary => OnSecondary,
            SecondaryContainer => OnSecondaryContainer,
            Tertiary => OnTertiary,
            TertiaryContainer => OnTertiaryContainer,
            Surface => OnSurface,
            SurfaceDim => OnSurface,
            SurfaceBright => OnSurface,
            SurfaceContainerLowest => OnSurface,
            SurfaceContainerLow => OnSurface,
            SurfaceContainer => OnSurface,
            SurfaceContainerHigh => OnSurface,
            SurfaceContainerHighest => OnSurface,
            SurfaceVariant => OnSurfaceVariant,
            InverseSurface => InverseOnSurface,
            Background => OnBackground,
            Error => OnError,
            ErrorContainer => OnErrorContainer,
            Success => OnSuccess,
            Outline => OutlineVariant,
            SurfaceTint => OnSurface,
            Scrim => OnBackground,
            _ => OnSurfaceVariant,
        }
    }
}
