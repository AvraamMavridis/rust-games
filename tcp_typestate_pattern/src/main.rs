use states::*;

pub mod states {
    #[derive(Debug, Clone)]
    pub struct Open {
        pub data: Option<String>
    }

    #[derive(Debug, Clone)]
    pub struct Closed {}

    #[derive(Debug, Clone)]
    pub struct Listening {}


    pub trait StateMarker {}
    impl StateMarker for () {}
    impl StateMarker for Open {}
    impl StateMarker for Closed {}
    impl StateMarker for Listening {}
}

struct TcpConnection<S: StateMarker> {
    _state: S,
}

impl TcpConnection<()> {
    fn new() -> TcpConnection<Closed> {
        TcpConnection {
            _state: states::Closed {},
        }
    }

    fn close(self) -> TcpConnection<Closed> {
        TcpConnection {
            _state: states::Closed {}
        }
    }
}

impl TcpConnection<Closed> {
    fn open(self) -> TcpConnection<Open> {
        TcpConnection {
            _state: states::Open {
                data: None
            }
        }
    }

    fn is_open(&self) -> bool {
        false
    }
}

impl TcpConnection<Open> {
    fn listen(self) -> TcpConnection<Listening> {
        TcpConnection {
            _state: states::Listening {}
        }
    }

    fn close(self) -> TcpConnection<Closed> {
        TcpConnection {
            _state: states::Closed {}
        }
    }

    fn is_open(&self) -> bool {
        true
    }

    fn is_listening(&self) -> bool {
        false
    }

    fn send(&mut self, data: &str) {
        self._state.data = Some(String::from(data));
    }

    fn receive(self) -> String {
        match self._state.data.clone() {
            None => String::from(""),
            Some(data) => data
        }
    }
}

impl TcpConnection<Listening> {
    fn is_open(&self) -> bool {
        true
    }

    fn is_listening(&self) -> bool {
        true
    }

    fn accept(&self) {
        println!("I am accepting data")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_connection() {
        let connection = TcpConnection::new();
        let open_connection = connection.open();
        assert!(open_connection.is_open());
    }

    #[test]
    fn test_listen_connection() {
        let connection = TcpConnection::new();
        let open_connection = connection.open(); 
        let listening_connection = open_connection.listen();
        assert!(listening_connection.is_listening());
    }

    #[test]
    fn test_close_connection() {
        let connection = TcpConnection::new();
        let open_connection = connection.open();
        let closed_connection = open_connection.close();
        assert_eq!(closed_connection.is_open(), false);
    }

    #[test]
    fn test_accept_connection() {
        let mut connection = TcpConnection::new();
        let open_connection = connection.open();
        let listening_connection = open_connection.listen();
        listening_connection.accept();
        assert!(listening_connection.is_open());
    }

    #[test]
    fn test_send_receive_data() {
        let connection = TcpConnection::new();
        let mut open_connection = connection.open();
        open_connection.send("Hello, world!");
        let received_data = open_connection.receive();
        assert_eq!(received_data, "Hello, world!");
    }
}
