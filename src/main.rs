use std::str::FromStr;

use num::Complex;

fn main() {}

/// Analizza la stringa `s` come una coppia di coordinate, come `"400x600"` o `"1.0,0.5"`.
///
/// Nello specifico, `s` dovrebbe avere la forma <left><sep><right>, dove <sep> è
/// il carattere fornito dall'argomento `separator`, e <left> e <right> sono
/// entrambe stringhe che possono essere analizzate da `T::from_str`. `separator` deve essere un
/// carattere ASCII.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Prova a determinare se `c` è nel set di Mandelbrot, usando al massimo `limit`
/// iterazioni per decidere
///
/// Se `c` non è un membro, restituisce `Some(i)`, dove `i` è il numero di
/// iterazioni necessarie affinché `c` lasci il cerchio di raggio 2 centrato sull'origine.
/// Se `c` sembra essere un membro (più precisamente, se abbiamo raggiunto
/// il limite di iterazioni senza riuscire a dimostrare che `c` non è un membro),
/// restituisce `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// Analizza una coppia di numeri in virgola mobile separati da una virgola
/// come un numero complesso.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Data la riga e la colonna di un pixel nell'immagine di output, restituisce il
/// punto corrispondente nel piano del complesso.
///
/// `bounds` e' una coppia che indica l'altezza e la larghezza dell'immagine in pixel.
/// `pixel` e' una coppia (colonna, riga) che indica un particolare pixel in quella immagine.
/// `upper_left` e `lower_right` sono punti nel piano complesso che designano
/// l'area che la nostra immagine copre.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.0 as f64 * height / bounds.1 as f64,
        // perche' la sottrazione qui? pixel.1 cresce mentre andiamo giu'
        // ma la parte immaginaria cresce  mentre andiamo su
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: 0.75 }
    );
}


/// Renderizza un rettangolo dell'insieme di Mandelbrot in un buffer di pixel.
/// 
/// `bounds` da l'altezza e l'ampiezza del buffer `pixels`,
/// il quale ha un singolo pixel in scala di grigi per byte.
/// `upper_left` e `upper_right` specificano punti nel piano complesso corrispondenti 
/// agli angoli del pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            // se escape_time dice che point appartiene al set, viene renderizzato come nero (0)
            // altrimenti render assegna colori più scuri ai numeri che hanno impiegato più tempo 
            // per uscire dal cerchio.
            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}
