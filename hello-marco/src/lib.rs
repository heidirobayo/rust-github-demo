/* A Marco Polo Game
  If the name is given, the program will respond with polo.
  Otherwise, it will respond "What's is your name?"".
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What's your name?".to_string()
    }
}
