# Simple Storage Smart Contract

## Overview

This is a basic Rust smart contract implemented using the ink! framework, demonstrating a simple storage mechanism with fundamental blockchain contract operations.

## Features

- Initialize contract with a default or custom initial value
- Get current stored value
- Increment stored value
- Set a new value

## Prerequisites

Before you begin, ensure you have the following installed:

- Rust (https://www.rust-lang.org/tools/install)
- cargo-contract CLI
  ```bash
  cargo install cargo-contract
  ```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/sanjaykohli/Smart_Contract.git
   cd simple-storage
   ```

2. Build the contract:
   ```bash
   cargo contract build
   ```

## Running Tests

To run the unit tests:
```bash
cargo test
```

## Contract Methods

### Constructors
- `new(init_value: u32)`: Creates a new contract with a specified initial value
- `default()`: Creates a new contract with an initial value of 0

### Messages
- `get()`: Returns the current stored value
- `increment()`: Increases the stored value by 1
- `set(new_value: u32)`: Sets a new value for storage

## Example Usage

```rust
// Create a new contract with initial value 5
let contract = SimpleStorage::new(5);

// Get current value
let current_value = contract.get(); // Returns 5

// Increment value
contract.increment(); // Value is now 6

// Set a new value
contract.set(10); // Value is now 10
```

## Development

This contract is built with:
- Rust programming language
- ink! smart contract framework

## Testing

Comprehensive unit tests are included to verify:
- Default contract initialization
- Increment functionality
- Value setting mechanism

## License

[Choose an appropriate license, e.g., MIT, Apache, etc.]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
