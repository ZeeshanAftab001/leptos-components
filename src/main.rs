pub mod components;

use leptos::prelude::*;
use components::button::Button;
use components::button::ButtonVariant;

#[component]
fn Counter() -> impl IntoView {

    let (count, set_count) = signal(0);

    view!{

        <Button label="Click Me!" variant=ButtonVariant::Ghost on_click=Callback::new(move |_| leptos::logging::log!("{}", count.get()))/>
        <h1>{move || count.get()}</h1>
        <Button label="Increase" on_click=Callback::new(move |_| set_count.update(|x| *x+=1))/>
        <Button label="Decrease" variant=ButtonVariant::Danger on_click=Callback::new(move |_| set_count.update(|x| *x-=1))/>

    }
}

fn main(){
    leptos::mount::mount_to_body(Counter);
}
