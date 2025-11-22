fn main() {
    // === LIST OF CANDIDATES ===
    // Each candidate is stored as a tuple: (name, years of experience)
    let candidates = vec![
        ("John", 3),
        ("Amina", 7),
        ("Chinedu", 10),
        ("Grace", 4),
        ("Emeka", 12),
    ];

    // === VARIABLE TO KEEP TRACK OF HIGHEST EXPERIENCE ===
    // Start with empty name and 0 years
    let mut highest = ("", 0);

    // === LOOP THROUGH THE CANDIDATE LIST ===
    for candidate in candidates {
        let name = candidate.0;
        let years = candidate.1;

        if years > highest.1 {
            highest = (name, years);
        }
    }

    // === SHOW RESULT ===
    println!(
        "The person with the highest programming experience is {} with {} years.",
        highest.0, highest.1
    );
}
