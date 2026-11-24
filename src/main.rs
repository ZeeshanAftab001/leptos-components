pub mod components;

use components::button::{Button, ButtonVariant};
use components::card::Card;
use leptos::prelude::*;

#[component]
fn Counter() -> impl IntoView {
    view! {

        <div style="display:flex;flex-direction:row">
        <Card title="Sapiens: A Brief History of Humankind"
        subtitle="xplores human evolution and how cognitive,
        agricultural, and scientific revolutions shaped modern".to_string()>
            <Button label="Secondary" variant=ButtonVariant::Secondary/>
        </Card>
        </div>

    }
}

fn main() {
    leptos::mount::mount_to_body(Counter);
}
