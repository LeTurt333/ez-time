# ez-time

**What is this?:** Some helpers for manipulating `cosmwasm_std::Timestamp`'s inside Smart Contracts

**Use Case:** Nice for Query Responses & Response attributes

**Example:** Say your contract stores a map of `Order`s in state. `Order` is a struct that has a field `expiration: Timestamp`. You have a query function `get_expiration` that returns an Order's expiration time.

Rather than dealing with the timestamp conversion on front-end, in your Query logic you could call `order.expiration.eztime_string()?;` which will convert a valid Timestamp into a String like `January 11, 2021 | 10:44:59`, which you could then pass off in your Query Response instead


### Usage

- Add dependencies located in `cargo.toml` to your Contract if they are not already there
- Create a new file `src/utils.rs` in your contract repo
- Copy/paste everything in `src/ez-time.rs` to your `src/utils.rs`
- Add `pub mod utils;` to your `src/lib.rs` in your contract repo
- Import utils where you'd like to use them - `use crate::utils::*`
- Now it's ez-time

```rust
pub fn execute_thing(
    deps: Deps,
    env: Env,
    //..
) -> Result<Response, ContractError> {

    let my_timestamp = env.block.time;

    let my_eztime = my_timestamp.eztime_string()?;

    //..rest of logic

    Ok(Response::new()
        .add_attribute("method", "I executed a thing")
        .add_attribute("At this time", my_eztime)
        //..
    )
}
```