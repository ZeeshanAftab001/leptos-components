pub mod components;

use components::alert::Alert;
use components::badge::{Badge, BadgeColor};
use components::card::Card;
use leptos::prelude::*;

#[component]
fn Counter() -> impl IntoView {
    view! {

        <div style="display:flex;flex-direction:row">
        <Card title="Sapiens: A Brief History of Humankind"
        subtitle="explores human evolution and how cognitive,
        agricultural, and scientific revolutions shaped modern".to_string()>
        <Badge text="ongoing" variant=BadgeColor::Green />
        <Badge text="shipped" variant=BadgeColor::Blue />
        <Badge text="cancelled" variant=BadgeColor::Red />
        <Alert message="Do you want to continue?"/>
        </Card>
        </div>

    }
}

fn main() {
    leptos::mount::mount_to_body(Counter);
}
