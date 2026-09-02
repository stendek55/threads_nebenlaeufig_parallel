use rand::RngExt;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 4 THREADS STARTEN ===");

    // wir erstellen den kanal vor den threads
    let (tx, rx) = mpsc::channel();

    // wir erstellen eine leere liste für die platzierungen.
    // Mutex: garantiert, dass immer nur eine schnecke gleichzeitig in die liste schreibt.
    // Arc: ein atomarer referenzzähler, damit viele threads den besitz der liste teilen können.
    let rangliste = Arc::new(Mutex::new(Vec::new()));

    let schnecken = vec!["Gary", "Turbo", "Flash", "Rocky"];
    let mut handles = vec![];

    for name in schnecken {
        // wir klonen den sender für jede schnecke
        let tx_klon = tx.clone();
        let name_string = name.to_string();

        // für jeden thread klonen wir den arc-zeiger auf die gemeinsame rangliste.
        // es wird nicht die liste geklont, sondern nur die "zutrittsberechtigung".
        let rangliste_klon = Arc::clone(&rangliste);

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
            // .lock() wartet, bis der mutex frei ist, und sperrt ihn dann für andere.
            // .unwrap() fängt den fehler ab, falls ein anderer thread mit dem lock abgestürzt ist.
            let mut daten = rangliste_klon.lock().unwrap();

            // wir sind im exklusiven besitz der liste und tragen die schnecke ein.
            // wer zuerst hier ankommt, wird als erster der liste hinzugefügt!
            daten.push(name_string.clone());

            // am ende des threads fällt die variable 'daten' aus dem scope.
            // der lock wird automatisch gelöst und der mutex wieder freigegeben!

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

    // das finale ergebnis auslesen
    // der hauptthread fordert am ende den lock an, um die rangliste auszugeben.
    println!("\n=== OFFIZIELLES RENNERGEBNIS (Echte Platzierung via Mutex) ===");
    let finale_liste = rangliste.lock().unwrap();
    for (platz, name) in finale_liste.iter().enumerate() {
        println!("{}. Platz: {}", platz + 1, name);
    }
}
