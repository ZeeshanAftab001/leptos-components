use leptos::prelude::*;

pub enum BadgeColor {
    Blue,
    Green,
    Yellow,
    Red,
    Gray,
}

#[component]
pub fn Badge(
    #[prop(default=BadgeColor::Blue)] variant: BadgeColor,
    #[prop(into)] text: String,
) -> impl IntoView {
    let (bg, fg) = match &variant {
        BadgeColor::Blue => ("#dbeafe", "#1e40af"),
        BadgeColor::Green => ("#dcfce7", "#166534"),
        BadgeColor::Yellow => ("#fef9c3", "#854d0e"),
        BadgeColor::Red => ("#fee2e2", "#991b1b"),
        BadgeColor::Gray => ("#f3f4f6", "#374151"),
    };

    view! {
        <span style=format!(
                    "display: inline-block; padding: 2px 8px; border-radius: 999px; \
                     background: {bg}; color: {fg}; font-size: 0.78rem; font-weight: 500; margin: 2px;"
                )>
            {text}
        </span>
    }
}
