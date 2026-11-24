use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div style="font-family:DM Sans;background-color:#f5f5f5;padding:20px 20px 20px 20px;overflow:hidden;margin:20px;border-radius:20px">
            <div style="color:black">
                <h3 style="">{title}</h3>
                {subtitle.map(|s| view! { <p style="">{s}</p> })}
            </div>
            <div style="">{children()}</div>
        </div>
    }
}
