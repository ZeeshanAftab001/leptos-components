use leptos::prelude::*;


#[component]
pub fn Greet(#[prop(default="World".to_string())] name : String) -> impl IntoView {
    view!{
        <p>{format!("Hello,{}",name)}</p>
    }
}
