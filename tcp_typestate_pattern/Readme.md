# Introduction
In this exercise, you will implement a simplified version of a TCP connection using the typestate pattern in Rust. The typestate pattern is a powerful design pattern that uses the type system to enforce certain states at compile time, ensuring that certain operations are only performed in valid states.

You will create a `TcpConnection` struct that can transition between different states: `Closed`, `Open`, and `Listening`. The typestate pattern will help ensure that operations like sending and receiving data are only possible when the connection is in the correct state.

## Requirements

### States
- **Closed**: Initial state, no operations allowed except `open` and `listen`.
- **Open**: State when the connection is established, allows sending and receiving data.
- **Listening**: State when the connection is listening for incoming connections, allows accepting connections.

### Transitions
- `Closed -> Open`: Transition by calling `open`.
- `Closed -> Listening`: Transition by calling `listen`.
- `Open -> Closed`: Transition by calling `close`.
- `Listening -> Closed`: Transition by calling `close`.
- `Listening -> Open`: Transition by calling `accept`.

### Operations
- `open`: Establish a connection.
- `listen`: Start listening for incoming connections.
- `accept`: Accept an incoming connection (only valid in `Listening` state).
- `send`: Send data (only valid in `Open` state).
- `receive`: Receive data (only valid in `Open` state).
- `close`: Close the connection.


