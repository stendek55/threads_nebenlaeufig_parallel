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

## 📚 lizenz

bereitgestellt unter der mit-lizenz. programmiert im neovim zur fehlerfreien code-erstellung.

