use dioxus_core::prelude::*;

fn main() {}
struct SomeContext {
    items: Vec<String>,
}

struct Props {
    name: String,
}

#[allow(unused)]
static Example: FC<Props> = |cpt| {
    todo!()

    // let value = cx.use_context(|c: &SomeContext| c.items.last().unwrap());

    // cx.render(LazyNodes::new(move |bump| {
    //     builder::ElementBuilder::new(bump, "button")
    //         .on("click", move |_| {
    //             println!("Value is {}", cx.name);
    //             println!("Value is {}", value.as_str());
    //             println!("Value is {}", *value);
    //         })
    //         .on("click", move |_| {
    //             println!("Value is {}", cx.name);
    //         })
    //         .finish()
    // }))
    // cx.render(html! {
    //     <div>
    //         <button onclick={move |_| println!("Value is {}", value)} />
    //         <button onclick={move |_| println!("Value is {}", value)} />
    //         <button onclick={move |_| println!("Value is {}", value)} />
    //         <div>
    //             <p> "Value is: {val}" </p>
    //         </div>
    //     </div>
    // })
};
