use rand::RngExt;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 4 THREADS STARTEN ===");

    // wir erstellen den kanal vor den threads
    let (tx, rx) = mpsc::channel();

    let schnecken = vec!["Gary", "Turbo", "Flash", "Rocky"];
    let mut handles = vec![];

    for name in schnecken {
        // wir klonen den sender für jede schnecke
        let tx_klon = tx.clone();
        let name_string = name.to_string();

        // thread::spawn startet einen echten os-thread.
        // das move-schlüsselwort übergibt den besitz des namens an den thread (send-trait).
        let handle = thread::spawn(move || {
            let mut rng = rand::rng();

            for schritt in 1..4 {
                let kriech_zeit = rng.random_range(50..200);

                // der thread schläft, ohne die anderen threads zu blockieren
                thread::sleep(Duration::from_millis(kriech_zeit));
                // println!("[{}] ist im Ziel angekommen!", name_string);

                // nachricht durch das kabel schicken
                let nachricht = format!("{} hat Schritt {} geschafft", name_string, schritt);
                tx_klon.send(nachricht).unwrap();
            }

            name_string // rückgabewert des threads
        });

        handles.push(handle);
    }

    // extrem wichtig: den originalen sender im hauptthread zerstören,
    // damit die nachfolgende empfänger-schleife weiß, wann schluss ist.
    drop(tx);

    println!("-> Schiedsrichter: Ich lese jetzt die Live-Funksprüche aus...");

    // diese schleife liest alle funksprüche live und in der echten
    // reihenfolge aus, bis die schnecken aufhören zu senden.
    for funkspruch in rx {
        println!("📻 [FUNK] {}", funkspruch);
    }

    println!("\n-> Schiedsrichter: Der Funkkanal ist leer. Jetzt hole ich die Join-Handles ab:");

    // hier bleibt deine bestehende logik erhalten, um die threads sauber zu beenden
    // der hauptthread wartet hier auf alle 4 arbeiter
    for h in handles {
        let beendet = h.join().unwrap();
        println!(
            "Schiedsrichter hat die Ankunft von {} protokolliert.",
            beendet
        );
    }
}
