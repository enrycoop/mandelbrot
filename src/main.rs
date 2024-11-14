use std::str::FromStr;

use num::Complex;

fn main() {
    
}

/// Analizza la stringa `s` come una coppia di coordinate, come `"400x600"` o `"1.0,0.5"`.
///
/// Nello specifico, `s` dovrebbe avere la forma <left><sep><right>, dove <sep> è
/// il carattere fornito dall'argomento `separator`, e <left> e <right> sono
/// entrambe stringhe che possono essere analizzate da `T::from_str`. `separator` deve essere un
/// carattere ASCII.
fn parse_pair<T: FromStr>(s: &str, separator:char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
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
        Some((re,im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), 
               Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

