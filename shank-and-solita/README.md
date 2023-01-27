# Shank & Solita

The devs at Metaplex created Shank & Solita for native Solana programs to be able to take advantage of serialization & IDLs just like Anchor programs.

### Shank

Shank is the Rust crate responsible for generating an IDL for your program.   
   
It's super easy to use in your Rust code:   
   
Add this annotation to any struct to mark it as an account:
```rust
#[derive(ShankAccount)]
```
ex:
```rust
#[derive(BorshDeserialize, BorshSerialize, Clone, ShankAccount)]
pub struct Car {
    pub year: u16,
    pub make: String,
    pub model: String,
}
```

Add this annotation to any enum to mark it as an instruction enum:
```rust
#[derive(ShankInstruction)]
```
ex:
```rust
#[derive(BorshDeserialize, BorshSerialize, Clone, ShankInstruction)]
pub enum CarRentalServiceInstruction {
    AddCar(Car),
    BookRental(RentalOrder),
    PickUpCar,
    ReturnCar,
}
```

Then you just need to add the Shank CLI:
```shell
cargo install shank-cli
```
```shell
USAGE:
    shank <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    idl
```

> Note: You do have to make use of `declare_id` in order for Shank to work properly:
```rust
declare_id!("8avNGHVXDwsELJaWMSoUZ44CirQd4zyU9Ez4ZmP4jNjZ");
```

### Solita

Solita is the JavaScript SDK responsible for building client-side SDK types from your program's IDL.

> Note: Solita will work with an IDL from Shank or from Anchor!

