use rand::RngExt;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 4 THREADS STARTEN ===");

    let schnecken = vec!["Gary", "Turbo", "Flash", "Rocky"];
    let mut handles = vec![];

    for name in schnecken {
        let name_string = name.to_string();

        // thread::spawn startet einen echten os-thread.
        // das move-schlüsselwort übergibt den besitz des namens an den thread (send-trait).
        let handle = thread::spawn(move || {
            let mut rng = rand::rng();
            let kriech_zeit = rng.random_range(50..200);

            // der thread schläft, ohne die anderen threads zu blockieren
            thread::sleep(Duration::from_millis(kriech_zeit));
            println!("[{}] ist im Ziel angekommen!", name_string);

            name_string // rückgabewert des threads
        });

        handles.push(handle);
    }

    // der hauptthread wartet hier auf alle 4 arbeiter
    for h in handles {
        let beendet = h.join().unwrap();
        println!(
            "Schiedsrichter hat die Ankunft von {} protokolliert.",
            beendet
        );
    }
}
