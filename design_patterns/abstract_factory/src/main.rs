// Define the Button trait with a render method
trait Button {
    fn render(&self);
}

// Define the Checkbox trait with a render method and a default toggle_check method
trait Checkbox {
    fn render(&self);
    fn toggle_check(&self) {
        println!("Toggled")
    }
}

// Define concrete implementations for Dark and Light themed buttons and checkboxes
struct DarkButton;
struct DarkCheckbox;
struct LightButton;
struct LightCheckbox;

// Implement the Button trait for DarkButton
impl Button for DarkButton {
    fn render(&self) {
        println!("I am a dark button")
    }
}

// Implement the Button trait for LightButton
impl Button for LightButton {
    fn render(&self) {
        println!("I am a light button")
    }
}

// Implement the Checkbox trait for DarkCheckbox
impl Checkbox for DarkCheckbox {
    fn render(&self) {
        println!("I am a dark checkbox")
    }
}

// Implement the Checkbox trait for LightCheckbox
impl Checkbox for LightCheckbox {
    fn render(&self) {
        println!("I am a light checkbox")
    }
}

// Define the ThemeFactory trait with methods to create buttons and checkboxes
trait ThemeFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

// Define concrete factories for Dark and Light themes
struct DarkFactory;
struct LightFactory;

// Implement the ThemeFactory trait for DarkFactory
impl ThemeFactory for DarkFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(DarkButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(DarkCheckbox)
    }
}

// Implement the ThemeFactory trait for LightFactory
impl ThemeFactory for LightFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(LightButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(LightCheckbox)
    }
}

// Function to render the UI using a given theme factory
fn render_ui(factory: &dyn ThemeFactory) {
    let button: Box<dyn Button> = factory.create_button();
    let checkbox: Box<dyn Checkbox> = factory.create_checkbox();

    button.render();
    checkbox.render();
}

// Main function to demonstrate the abstract factory pattern
fn main() {
    // Create a dark theme factory and render the UI
    let dark_factory = DarkFactory{};
    render_ui(&dark_factory);

    // Create a light theme factory and render the UI
    let light_factory = LightFactory{};
    render_ui(&light_factory);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_button() {
        let dark_factory = DarkFactory{};
        let button = dark_factory.create_button();
        button.render();
    }

    #[test]
    fn test_dark_theme_checkbox() {
        let dark_factory = DarkFactory{};
        let checkbox = dark_factory.create_checkbox();
        checkbox.render();
    }

    #[test]
    fn test_light_theme_button() {
        let light_factory = LightFactory{};
        let button = light_factory.create_button();
        button.render();
    }

    #[test]
    fn test_light_theme_checkbox() {
        let light_factory = LightFactory{};
        let checkbox = light_factory.create_checkbox();
        checkbox.render();
    }

    #[test]
    fn test_render_ui_with_dark_factory() {
        let dark_factory = DarkFactory{};
        render_ui(&dark_factory);
    }

    #[test]
    fn test_render_ui_with_light_factory() {
        let light_factory = LightFactory{};
        render_ui(&light_factory);
    }
}
