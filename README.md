# Writing image files
La crate `image` fornisce funzioni per leggere e scrivere una grande varietà di formati di immagini,
così come qualche funzione di manipolazione di immagini di base.
In particolare, include un encoder per PNG file format, questo programma lo usa per salvare il risultato
finale del calcolo.

### Warning
In questo momento (16/11/2024), la compilazione di `image` da un warning sul crate `bitflags v0.7.0`
dicendo che sarà incompatibile con le future versioni di Rust.
**Controllare l'aggiornamento del suddetto crate.**

## Eseguire i test
1) Posizionarsi nella root del progetto clonato
2) Eseguire il seguente comando
```command
cargo test
```
## Compilazione ed esecuzione del programma
1) Posizionarsi nella root del progetto clonato
2) Eseguire il seguente comando
```command
cargo build --release
time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
```
```command
Usage: target/debug/mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT
Example: target/debug/mandelbrot mandel.png 1000x750 -1.20,0.35 -1,0.20
```
