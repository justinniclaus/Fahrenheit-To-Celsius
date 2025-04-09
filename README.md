## Program Overview

- A command-line application that converts between **Fahrenheit** and **Celsius** temperatures
- Demonstrates fundamental Rust concepts including **input handling**, **type conversion**, **conditional logic**, and **string formatting**

## Core Rust Concepts Utilized

### User Input Processing

- **`std::io`** library provides input/output functionality
- **`String`** type represents a growable, UTF-8 encoded text string
    
    ```rust
    let mut input = String::new();
    let mut value = String::new();
    ```
    
- **`stdin()`** function accesses standard input
- **`read_line()`** method reads user input into a string
    
    ```rust
    io::stdin()    
	    .read_line(&mut input)    
	    .expect("Failed to read line.");
    ```
    
- **`&mut`** indicates a mutable reference, allowing functions to modify the original variable

### String Manipulation

- **`trim()`** method removes whitespace (spaces, tabs, newlines) from start and end of string
    
    ```rust
    let input: &str = input.trim();
    ```
    
- **`parse()`** method converts strings to other types (string to number in this case)
    
    ```rust
    let value: f32 = value    
	    .trim()    
	    .parse()    
	    .expect("Failed to convert from string to floating point.");
    ```
    

### Type System

- **Variables** are declared with **`let`** keyword
- **Type annotations** use colon syntax: `let value: f32`
- **`f32`** is a 32-bit floating point type (for decimal numbers)
- **Type inference** allows Rust to determine types when not explicitly stated
- **Mutability** must be declared with **`mut`** keyword

### Error Handling

- **`expect()`** method handles potential errors from operations that might fail
- Provides meaningful error messages if operations fail
    
    ```rust
    .expect("Failed to convert from string to floating point.");
    ```
    

### Conditional Logic

- **`if`/`else`** statements control program flow based on conditions
- **Logical operators** combine multiple conditions
    
    - **`||`** (OR): Either condition can be true
    
    ```rust
    if input == "f" || input == "F" 
    {
        // Fahrenheit to Celsius conversion
    } else if input == "c" || input == "C" 
    {
        // Celsius to Fahrenheit conversion
    } 
    else 
    {
        // Invalid input handling
    }
    ```
    

### String Formatting and Output

- **String interpolation** with curly braces `{}` inserts values into strings
- **Format specifiers** control how values are displayed
    
    - **`{:.1}`** formats to one decimal place
    
    ```rust
    println!("Fahrenheit {:.1} converts to Celsius {:.1}.", value, c);
    ```
    

## Mathematical Operations

- **Temperature conversion formulas**:
    
    - Fahrenheit to Celsius: `C = (F - 32) / 1.8`
    
    ```rust
    let c = (value - 32.0) / 1.8;
    ```
    
    - Celsius to Fahrenheit: `F = (C * 1.8) + 32`
    
    ```rust
    let f = (value * 1.8) + 32.0;
    ```
    

## Program Flow Diagram

```
┌───────────────────┐
│ Program Start     │
└─────────┬─────────┘
          ▼
┌───────────────────┐
│ Ask for temp type │
│ (F or C)          │
└─────────┬─────────┘
          ▼
┌───────────────────┐
│ Read user input   │
└─────────┬─────────┘
          ▼
┌───────────────────┐
│ Ask for temp value│
└─────────┬─────────┘
          ▼
┌───────────────────┐
│ Read numeric value│
└─────────┬─────────┘
          ▼
┌───────────────────┐
│ Check input type  │──┐
└─────────┬─────────┘  │
          │            │
          ▼            ▼
┌───────────────────┐ ┌───────────────────┐
│ If "F": Convert   │ │ If "C": Convert   │
│ F → C             │ │ C → F             │
└─────────┬─────────┘ └─────────┬─────────┘
          │                     │
          ▼                     ▼
┌───────────────────┐ ┌───────────────────┐
│ Display result    │ │ Display result    │
│ with 1 decimal    │ │ with 1 decimal    │
└─────────┬─────────┘ └─────────┬─────────┘
          │                     │
          └──────────┬──────────┘
                     ▼
┌───────────────────┐
│ Program End       │
└───────────────────┘
```

## Complete Program Code

```rust
use std::io;

fn main() {
    let mut input = String::new();
    let mut value = String::new();
    
    println!("Farenheit or Celsius? (Enter 'f' or 'c')");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    
    let input: &str = input.trim();
    
    println!("Enter the temperature value:");
    
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read value.");
    
    let value: f32 = value
        .trim()
        .parse()
        .expect("Failed to convert from string to floating point.");
    
    if input == "f" || input == "F" {
        // Convert Fahrenheit to Celsius
        let c = (value - 32.0) / 1.8;
        println!("Fahrenheit {:.1} converts to Celsius {:.1}.", value, c);
    } else if input == "c" || input == "C" {
        // Convert Celsius to Fahrenheit
        let f = (value * 1.8) + 32.0;
        println!("Celsius {:.1} converts to Fahrenheit {:.1}.", value, f);
    } else {
        println!("Invalid input. Please enter 'f' or 'c' for Fahrenheit or Celsius.");
    }
}
```
