fn main() {
    println!("Hello, world!");
}

struct Temperature {
    value: f64
}

impl Temperature {
    fn from_celcious(celcious: f64) -> Self {
        Temperature {
            value: celcious
        }
    }

    fn from_fahrenheit(fahrenheit: f64) -> Self {
        Temperature { value: (fahrenheit - 32.0) * 5.0 / 9.0 }
    }

    fn as_celsius(&self) -> f64 {
        self.value
    }

    fn as_fahrenheit(&self) -> f64 {
        self.value * 9.0 / 5.0 + 32.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_celsius() {
        let temp = Temperature::from_celcious(0.0);
        assert_eq!(temp.value, 0.0);
    }

    #[test]
    fn test_from_fahrenheit() {
        let temp = Temperature::from_fahrenheit(32.0);
        assert_eq!(temp.value, 0.0);
    }

    #[test]
    fn test_as_celsius() {
        let temp = Temperature::from_celcious(0.0);
        assert_eq!(temp.as_celsius(), 0.0);
    }

    #[test]
    fn test_as_fahrenheit() {
        let temp = Temperature::from_celcious(0.0);
        assert_eq!(temp.as_fahrenheit(), 32.0);
    }
}
