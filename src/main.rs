use leptos::*;

fn main() {
    mount_to_body(|cx| view! {
        cx,
        <div>
            <h1>" Hello, world!"</h1>
            <p>" This is a paragraph. "</p>
        </div>
    })
}
