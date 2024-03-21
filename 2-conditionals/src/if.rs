fn main() {
    identifier = "Ferris";
    habitat = if identifier == "foo" {
        "ocean"
    } else if identifier == "Ferris" {
        "ocean && land"
    } else if identifier == "Ziggy" {
        "Desert"
    } else {
        "Unknown"
    };

    println!(identifier + "lives at" + habitat);
}
