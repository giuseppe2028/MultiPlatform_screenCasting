# Multiplatform Screen Casing

Questa applicazione, scritta intermanete in rust, è un progetto universitario con lo scopo di realizzaare tutte le features elencate nel file [Readme](./README.MD)

## Installazione ed esecuzione su Windows, MacOS e Ubuntu

Per poter eseguire l'applicazione su tutti e tre i sistemi operativi, bisona aver installato la seguente libreria:

- [ffmpeg](https://www.ffmpeg.org/download.html) si seguano le istruzioni a partire dalla [pagina ufficiale](https://www.ffmpeg.org/) per il download

## Installazione ed esecuzione su Ubuntu e altri Linux like

In base alla propria tipologia e versione del sistema operativo Linux like, bisogna accertarsi di avere librerie installate correttamente.
Un esempio di librerie che potrebbero mancare sono quelle relative alla compilazione di linguaggi c e rust, alcune librerie di sistema Linux like per compilare codice, librerie preseti nel file delle [dipendenze](./Cargo.toml) che siano compatibili con sistemi di tipo x11 e verificare tutti i percorsi di configurazione corretti delle librerie

## Procedura da terminale di comando

Se si vuole eseguire l'applicazione da terminale, bisogna avere un ambiente di lavoro rust completo (Rust + Cargo):

1. aprire una finestra di terminale nella root dell'applicazione.
2. fare update di rust con il comando:

    ```sh
    rustup update
    ```

3. fare clean di cargo (per eventuale sicurezza):

   ```sh
    cargo clean
    ```

4. fare update di cargo per installare tutte le dipendenze con il comando:

   ```sh
    cargo update
    ```

5. eseguire l'applicazione in ambiente release con il comando:

   ```sh
    cargo run --release
    ```

6. A questo punto siete in grado di usare l'aplicazione

## Procedura con eseguibile

Se avete l'eseguibile dell'applicazione, potete direttamente aprire ed eseguire l'applicazione senza un ulteriore intervento precedente
