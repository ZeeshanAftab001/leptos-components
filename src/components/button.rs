use leptos::prelude::*;


pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost
}

#[component]
pub fn Button (
    #[prop(into)] label : String,
    #[prop(default=ButtonVariant::Primary)] variant : ButtonVariant,
    #[prop(default=false)] disabled : bool,
    #[prop(optional)] on_click : Option<Callback<leptos::ev::MouseEvent>>
) -> impl IntoView {

    let (bg_color,color)=match &variant {
        ButtonVariant::Primary => ("#006400","white"),
        ButtonVariant::Secondary => ("#6b7280","white"),
        ButtonVariant::Danger => ("#FF0000","white"),
        ButtonVariant::Ghost => ("transparent","#374151"),

    };

    let style=format!("color:{color};background-color:{bg_color};padding:8px;border:1px solid {bg_color}; \
        border-radius:6px;cursor:pointer;margin:4px;font:inherit;");
    view!{
    <button
    style=style
    disabled=disabled
    on:click=move |event| {
        if let Some(callback)=&on_click{
            callback.run(event);
        }
    }
    >
    {label}
    </button>
    }
}
