struct Vehicle {
    km: usize,
    year: usize,
}

fn main() {
    let car = Vehicle {
        km: 80_000,
        year: 2020,
    };

    match car {
        Vehicle { km, year } if km == 0 && year == 2020 => println!("new 2020"),
        Vehicle { km, .. } if km <= 50_000 => println!("under 50k km"),
        Vehicle { km, .. } if km >= 80_000 => println!("at least 80k km"),
        Vehicle { year, .. } if year == 2020 => println!("made in 2020"),
        Vehicle { .. } => println!("other mileage"),
    }
}
