# threads_nebenlaeufig_parallel – das illegale schneckenrennen 🐌🏁

dieses projekt dient als ausführliches übungsbeispiel für **kapitel 16 („furchtlose nebenläufigkeit“)** aus dem offiziellen rust-buch. es demonstriert die kernkonzepte von paralleler programmierung, nachrichtenaustausch und gemeinsamem zustand anhand eines simulierten schneckenrennens.

---

## 🛠️ features & lerninhalte

das programm vereint die drei wichtigsten säulen der nebenläufigkeit in rust in einer einzigen, nahtlosen `main.rs`:

* **kapitel 16.1: threads & join-handles**
  vier schnecken (threads) werden über `thread::spawn` gestartet und laufen physisch parallel auf den cpu-kernen. der hauptthread wartet am ende mittels `.join()`, bis alle arbeiter sicher im ziel angekommen sind.
* **kapitel 16.2: nachrichtenaustausch über kanäle (mpsc)**
  die schnecken nutzen geklonte sender (`mpsc::channel`), um ihre kriech-fortschritte live und unblockiert an das funkgerät des schiedsrichters (hauptthread) zu senden.
* **kapitel 16.3: gemeinsamer zustand (shared state)**
  um die unbestechliche, echte platzierung im ziel zu ermitteln, tragen sich die schnecken selbstständig in eine gemeinsame liste ein. diese wird absolut thread-sicher über einen atomaren referenzzähler (`Arc`) und eine gegenseitige sperre (`Mutex`) verwaltet.

---

## 🚀 voraussetzungen & start

um das projekt auszuführen, wird die rust-toolchain sowie das `rand`-crate benötigt.

### 1. abhängigkeiten (cargo.toml)
das projekt verwendet das aktuelle `rand`-crate für die zufälligen kriechgeschwindigkeiten:

```toml
[dependencies]
rand = "0.10"
```

### 2. programm starten
klone das repository und jage den code über dein terminal hoch:

```bash
cargo run
```

---

## 🔬 verhaltens-beobachtung im terminal

wenn du das programm mehrfach ausführst, kannst du zwei faszinierende dinge beobachten, die die furchtlose nebenläufigkeit von rust beweisen:

1. **die live-funksprüche** trudeln in einer völlig unvorhersehbaren reihenfolge ein. das betriebssystem schiebt die threads so hin und her, wie gerade cpu-zeit frei ist.
2. **die offizielle rangliste am ende** lügt nie. selbst wenn zwei schnecken fast zeitgleich ins ziel schießen, sorgt der `Mutex` dafür, dass kein daten-race entsteht. wer den lock zuerst bekommt, steht unanfechtbar auf dem treppchen.
---
# das geheimnis von send und sync in rust

die beiden traits `Send` und `Sync` sind das fundament der „furchtlosen nebenläufigkeit“ (fearless concurrency) in rust. es handelt sich um marker-traits, die dem compiler mitteilen, wie ein datentyp in einer multi-thread-umgebung verwendet werden darf.

das geniale: du musst diese traits fast nie selbst implementieren. der compiler prüft die regeln automatisch und verweigert den dienst (`compile error`), bevor ein laufzeitfehler entstehen kann.

---

## 1. Send – der besitzerwechsel zwischen threads

das trait `Send` bedeutet: **ein datentyp darf seinen besitzer wechseln und von einem thread in einen anderen verschoben werden.**

wenn du `thread::spawn(move || ...)` benutzt, reißt du variablen aus dem hauptthread heraus und übergibst sie dem neuen thread. das erlaubt der compiler nur, wenn alle involvierten daten das trait `Send` erfüllen.

* **was ist send?** fast alle datentypen in rust (z. b. `i32`, `String`, `Vec`, eigene strukturen aus diesen typen).
* **was ist NICHT send?** ein bekanntes beispiel ist `Rc` (der normale referenzzähler). `Rc` ist für single-thread-code optimiert. wenn du ein `Rc` in einen anderen thread verschieben würdest, könnten zwei threads gleichzeitig den zähler erhöhen. da `Rc` das nicht atomar (thread-sicher) tut, würde der zähler korrumpieren. der compiler markiert `Rc` als `!Send` (nicht Send) und blockiert den code.
* **die lösung für threads:** `Arc` (atomarer referenzzähler). `Arc` erhöht den zähler über atomare operationen und ist daher `Send`.

---

## 2. Sync – das gemeinsame nutzen über referenzen

das trait `Sync` bedeutet: **mehrere threads dürfen gleichzeitig über unveränderliche referenzen (`&T`) auf dieselbe variable zugreifen.**

es gilt die mathematische faustregel in rust: ein typ `T` ist genau dann `Sync`, wenn die referenz darauf (`&T`) `Send` ist.

* **was ist sync?** alle typen, die primitiv sind oder deren inhalt sich nicht ohne kontrollmechanismen von außen ändern lässt (z. b. `i32`, `String`, oder ein `Mutex`).
* **was ist NICHT sync?** typen, die „interne mutabilität“ erlauben, wie `RefCell` oder `Cell`. diese typen ermöglichen es, daten über eine normale, unveränderliche referenz (`&T`) zu manipulieren. wenn das zwei threads gleichzeitig tun, entsteht ein daten-race. `RefCell` ist daher `!Sync`.
* **die lösung für threads:** `Mutex`. ein `Mutex` sorgt dafür, dass immer nur ein thread zeitgleich exklusiven zugriff auf die daten erhält. dadurch wird der inhalt um den `Mutex` herum für alle threads sicher teilbar (`Sync`).

---

## 📊 das zusammenspiel im überblick

| typ | Send (thread wechseln) | Sync (referenz teilen) | einsatzzweck |
| :--- | :---: | :---: | :--- |
| `i32`, `String` | ja | ja | normale, unveränderliche daten |
| `Rc<T>` | nein | nein | referenzzählung (nur im selben thread) |
| `Arc<T>` | ja | ja (wenn T auch Sync ist) | referenzzählung über thread-grenzen |
| `RefCell<T>` | ja | nein | verändern von daten über `&T` (nur im selben thread) |
| `Mutex<T>` | ja | ja (wenn T auch Send ist) | sicheres verändern von daten über thread-grenzen |

---

## 🔍 anwendung im schneckenrennen

im code des schneckenrennens wurde folgende struktur verwendet:

```rust
let rangliste = Arc::new(Mutex::new(Vec::new()));
```

hier greifen die regeln wie zahnräder ineinander:
1. das `Vec` in der mitte ist `Send`, aber **nicht** `Sync` (weil zwei threads nicht gleichzeitig ohne schutz reinschreiben dürfen).
2. der `Mutex` wird darum gelegt. ein `Mutex<Vec>` ist `Sync`, weil das `Vec` darin `Send` ist. jetzt ist die liste sicher teilbar.
3. der `Arc` wird darum gelegt. ein `Arc<Mutex<Vec>>` ist `Send` und `Sync`. 

erst jetzt erlaubt der compiler, die variable zu klonen und per `move` in die vier schnecken-threads zu übergeben.

---

## 📚 lizenz

bereitgestellt unter der mit-lizenz. programmiert im neovim zur fehlerfreien code-erstellung.

