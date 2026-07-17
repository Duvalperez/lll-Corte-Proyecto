use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// --- CONSTANTES DE CONFIGURACIÓN ---
const MAX_SIMBOLOS: usize = 50;  
const MAX_EXPRESION: usize = 100; 

// --- ESTRUCTURAS DE DATOS ---

#[derive(Copy, Clone, PartialEq, Debug)]
enum TipoToken {
    OPERANDO,
    OPERADOR,
    DESCONOCIDO,
}

#[derive(Copy, Clone, Debug)]
struct Simbolo {
    caracter: char,
    tipo: TipoToken,
    precedencia: i32,
}

#[derive(Copy, Clone, Debug)]
struct Token {
    caracter: char,
    tipo: TipoToken,
    precedencia: i32,
}

// Estructura de Pila Estática para la conversión
struct PilaOperadores {
    elementos: [Token; MAX_EXPRESION],
    tope: usize,
}

impl PilaOperadores {
    fn new() -> Self {
        PilaOperadores {
            elementos: [Token { caracter: '\0', tipo: TipoToken::DESCONOCIDO, precedencia: -1 }; MAX_EXPRESION],
            tope: 0,
        }
    }

    fn esta_vacia(&self) -> bool {
        self.tope == 0
    }

    fn push(&mut self, token: Token) -> Result<(), &'static str> {
        if self.tope >= MAX_EXPRESION {
            return Err("Desbordamiento de pila (Stack Overflow)");
        }
        self.elementos[self.tope] = token;
        self.tope += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<Token> {
        if self.esta_vacia() {
            None
        } else {
            self.tope -= 1;
            Some(self.elementos[self.tope])
        }
    }

    fn ver_tope(&self) -> Option<&Token> {
        if self.esta_vacia() {
            None
        } else {
            Some(&self.elementos[self.tope - 1])
        }
    }
}

// --- BLOQUES DE FUNCIONES ---

// BLOQUE 1: Carga y análisis inicial del archivo de texto
fn cargar_datos_desde_archivo(
    ruta: &str,
    tabla: &mut [Simbolo; MAX_SIMBOLOS],
    cant_simbolos: &mut usize,
    expresion: &mut [char; MAX_EXPRESION],
    long_expresion: &mut usize,
) -> Result<(), String> {
    let path = Path::new(ruta);
    let file = File::open(&path).map_err(|e| format!("No se pudo abrir el archivo '{}': {}", ruta, e))?;
    let reader = io::BufReader::new(file);

    let mut lineas = reader.lines();

    // Extraer la primera línea (Expresión)
    if let Some(primer_resultado) = lineas.next() {
        let linea = primer_resultado.map_err(|e| e.to_string())?;
        let linea_limpia = linea.trim();
        
        if !linea_limpia.starts_with("EXPRESION=") {
            return Err("El archivo de entrada no comienza con 'EXPRESION='".to_string());
        }
        
        let valor_expresion = &linea_limpia["EXPRESION=".len()..];
        let mut idx = 0;
        for c in valor_expresion.chars() {
            if idx >= MAX_EXPRESION {
                return Err(format!("La expresión excede el tamaño máximo permitido de {} caracteres.", MAX_EXPRESION));
            }
            expresion[idx] = c;
            idx += 1;
        }
        *long_expresion = idx;
    } else {
        return Err("El archivo de entrada está vacío.".to_string());
    }

    // Extraer la tabla de símbolos
    for linea_resultado in lineas {
        let linea = linea_resultado.map_err(|e| e.to_string())?;
        let linea_limpia = linea.trim();
        if linea_limpia.is_empty() {
            continue;
        }

        let partes: Vec<&str> = linea_limpia.split(',').collect();
        if partes.len() != 3 {
            return Err(format!("Línea con formato incorrecto en tabla de símbolos: '{}'", linea_limpia));
        }

        let caracter = partes[0].chars().next().ok_or("Símbolo vacío en la definición")?;
        
        let tipo = match partes[1] {
            "OPERANDO" => TipoToken::OPERANDO,
            "OPERADOR" => TipoToken::OPERADOR,
            _ => return Err(format!("Tipo de token desconocido: '{}'", partes[1])),
        };

        let precedencia = partes[2].parse::<i32>()
            .map_err(|_| format!("Precedencia no válida en línea: '{}'", linea_limpia))?;

        if *cant_simbolos >= MAX_SIMBOLOS {
            return Err(format!("La tabla de símbolos excede la capacidad estática de {}.", MAX_SIMBOLOS));
        }

        tabla[*cant_simbolos] = Simbolo { caracter, tipo, precedencia };
        *cant_simbolos += 1;
    }

    Ok(())
}

// BLOQUE 2: Análisis Léxico (Mapea caracteres a tokens buscando en la tabla de símbolos)
fn generar_tabla_tokens(
    expresion: &[char; MAX_EXPRESION],
    long_expresion: usize,
    tabla_simbolos: &[Simbolo; MAX_SIMBOLOS],
    cant_simbolos: usize,
    tokens_resultado: &mut [Token; MAX_EXPRESION],
) -> Result<(), String> {
    for i in 0..long_expresion {
        let caracter = expresion[i];
        let mut encontrado = false;

        for j in 0..cant_simbolos {
            if tabla_simbolos[j].caracter == caracter {
                tokens_resultado[i] = Token {
                    caracter: tabla_simbolos[j].caracter,
                    tipo: tabla_simbolos[j].tipo,
                    precedencia: tabla_simbolos[j].precedencia,
                };
                encontrado = true;
                break;
            }
        }

        if !encontrado {
            return Err(format!(
                "Carácter '{}' en la posición {} no está definido en la tabla de símbolos.",
                caracter, i
            ));
        }
    }
    Ok(())
}

// BLOQUE 3: Salida en consola de la tabla de tokens
fn imprimir_tabla_tokens(tokens: &[Token; MAX_EXPRESION], long_expresion: usize) {
    println!("=== Tabla de tokens ===");
    println!("{:<10} {:<15} {:<12}", "Token", "Tipo", "Precedencia");
    println!("-------------------------------------------");
    for i in 0..long_expresion {
        let tipo_str = match tokens[i].tipo {
            TipoToken::OPERANDO => "OPERANDO",
            TipoToken::OPERADOR => "OPERADOR",
            _ => "DESCONOCIDO",
        };
        println!("{:<10} {:<15} {:<12}", tokens[i].caracter, tipo_str, tokens[i].precedencia);
    }
    println!("-------------------------------------------");
}

// Auxiliares estáticos de inversión de arreglos
fn invertir_tokens(tokens: &[Token; MAX_EXPRESION], long: usize, resultado: &mut [Token; MAX_EXPRESION]) {
    for i in 0..long {
        resultado[i] = tokens[long - 1 - i];
    }
}

fn invertir_caracteres(caracteres: &mut [char; MAX_EXPRESION], long: usize) {
    let mut izq = 0;
    if long == 0 { return; }
    let mut der = long - 1;
    while izq < der {
        let temp = caracteres[izq];
        caracteres[izq] = caracteres[der];
        caracteres[der] = temp;
        izq += 1;
        der -= 1;
    }
}

// BLOQUE 4: Algoritmo de traducción de Infijo a Postfijo
fn convertir_a_postfijo(
    tokens: &[Token; MAX_EXPRESION],
    long_expresion: usize,
    postfijo_resultado: &mut [char; MAX_EXPRESION],
    long_postfija: &mut usize,
) -> Result<(), &'static str> {
    let mut pila = PilaOperadores::new();
    let mut idx_salida = 0;

    for i in 0..long_expresion {
        let token = tokens[i];
        match token.tipo {
            TipoToken::OPERANDO => {
                postfijo_resultado[idx_salida] = token.caracter;
                idx_salida += 1;
            }
            TipoToken::OPERADOR => {
                while !pila.esta_vacia() {
                    let tope = pila.ver_tope().unwrap();
                    if tope.precedencia >= token.precedencia {
                        let op_sacado = pila.pop().unwrap();
                        postfijo_resultado[idx_salida] = op_sacado.caracter;
                        idx_salida += 1;
                    } else {
                        break;
                    }
                }
                pila.push(token)?;
            }
            TipoToken::DESCONOCIDO => {}
        }
    }

    while !pila.esta_vacia() {
        let op_sacado = pila.pop().unwrap();
        postfijo_resultado[idx_salida] = op_sacado.caracter;
        idx_salida += 1;
    }

    *long_postfija = idx_salida;
    Ok(())
}

// BLOQUE 5: Algoritmo de traducción de Infijo a Prefijo
fn convertir_a_prefijo(
    tokens: &[Token; MAX_EXPRESION],
    long_expresion: usize,
    prefijo_resultado: &mut [char; MAX_EXPRESION],
    long_prefija: &mut usize,
) -> Result<(), &'static str> {
    // 1. Invertir la expresión de entrada (tokens)
    let mut tokens_invertidos: [Token; MAX_EXPRESION] = [
        Token { caracter: '\0', tipo: TipoToken::DESCONOCIDO, precedencia: -1 }; 
        MAX_EXPRESION
    ];
    invertir_tokens(tokens, long_expresion, &mut tokens_invertidos);

    // 2. Procesar tokens invertidos con algoritmo Shunting-yard modificado
    let mut pila = PilaOperadores::new();
    let mut idx_salida = 0;

    for i in 0..long_expresion {
        let token = tokens_invertidos[i];
        match token.tipo {
            TipoToken::OPERANDO => {
                prefijo_resultado[idx_salida] = token.caracter;
                idx_salida += 1;
            }
            TipoToken::OPERADOR => {
                while !pila.esta_vacia() {
                    let tope = pila.ver_tope().unwrap();
                    // Modificación clave: Para prefijo, solo sacamos si la precedencia del tope es ESTRICTAMENTE mayor.
                    if tope.precedencia > token.precedencia {
                        let op_sacado = pila.pop().unwrap();
                        prefijo_resultado[idx_salida] = op_sacado.caracter;
                        idx_salida += 1;
                    } else {
                        break;
                    }
                }
                pila.push(token)?;
            }
            TipoToken::DESCONOCIDO => {}
        }
    }

    while !pila.esta_vacia() {
        let op_sacado = pila.pop().unwrap();
        prefijo_resultado[idx_salida] = op_sacado.caracter;
        idx_salida += 1;
    }

    // 3. Invertir la cadena de salida acumulada para obtener la notación prefija final
    invertir_caracteres(prefijo_resultado, idx_salida);
    *long_prefija = idx_salida;
    Ok(())
}

// BLOQUE 6: Impresión de las expresiones formateadas
fn imprimir_expresiones(
    infija: &[char; MAX_EXPRESION], 
    long_infija: usize, 
    postfija: &[char; MAX_EXPRESION], 
    long_postfija: usize,
    prefija: &[char; MAX_EXPRESION],
    long_prefija: usize
) {
    print!("Expresion infija:   ");
    for i in 0..long_infija {
        print!("{}", infija[i]);
    }
    println!();

    print!("Expresion postfija: ");
    for i in 0..long_postfija {
        print!("{}", postfija[i]);
    }
    println!();

    print!("Expresion prefija:  ");
    for i in 0..long_prefija {
        print!("{}", prefija[i]);
    }
    println!("\n===========================================");
}

// --- HILO DE EJECUCIÓN PRINCIPAL ---
fn main() {
    let ruta_archivo = "entrada.txt";
    println!("=== Mini-Traductor Infijo a Postfijo/Prefijo (UCLA C7) ===");
    println!("Intentando procesar el archivo: '{}'...\n", ruta_archivo);

    // Inicialización de estructuras estáticas
    let mut tabla_simbolos: [Simbolo; MAX_SIMBOLOS] = [
        Simbolo { caracter: '\0', tipo: TipoToken::DESCONOCIDO, precedencia: -1 }; 
        MAX_SIMBOLOS
    ];
    let mut cant_simbolos = 0;

    let mut expresion_infija: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let mut long_expresion = 0;

    // 1. Cargar archivo
    if let Err(err) = cargar_datos_desde_archivo(
        ruta_archivo, 
        &mut tabla_simbolos, 
        &mut cant_simbolos, 
        &mut expresion_infija, 
        &mut long_expresion
    ) {
        eprintln!("[ERROR CRÍTICO] Error al cargar archivo: {}", err);
        return;
    }

    if long_expresion == 0 {
        eprintln!("[ERROR CRÍTICO] La expresión ingresada está vacía.");
        return;
    }

    // 2. Analizar léxicamente y generar Tokens
    let mut tokens: [Token; MAX_EXPRESION] = [
        Token { caracter: '\0', tipo: TipoToken::DESCONOCIDO, precedencia: -1 }; 
        MAX_EXPRESION
    ];

    if let Err(err) = generar_tabla_tokens(
        &expresion_infija, 
        long_expresion, 
        &tabla_simbolos, 
        cant_simbolos, 
        &mut tokens
    ) {
        eprintln!("[ERROR LÉXICO] {}", err);
        return;
    }

    // 3. Imprimir la Tabla de Tokens
    imprimir_tabla_tokens(&tokens, long_expresion);

    // 4. Convertir a Postfijo
    let mut expresion_postfija: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let mut long_postfija = 0;

    if let Err(err) = convertir_a_postfijo(
        &tokens, 
        long_expresion, 
        &mut expresion_postfija, 
        &mut long_postfija
    ) {
        eprintln!("[ERROR EN CONVERSIÓN POSTFIJA] {}", err);
        return;
    }

    // 5. Convertir a Prefijo
    let mut expresion_prefija: [char; MAX_EXPRESION] = ['\0'; MAX_EXPRESION];
    let mut long_prefija = 0;

    if let Err(err) = convertir_a_prefijo(
        &tokens, 
        long_expresion, 
        &mut expresion_prefija, 
        &mut long_prefija
    ) {
        eprintln!("[ERROR EN CONVERSIÓN PREFIJA] {}", err);
        return;
    }

    // 6. Mostrar resultados
    imprimir_expresiones(
        &expresion_infija, 
        long_expresion, 
        &expresion_postfija, 
        long_postfija,
        &expresion_prefija,
        long_prefija
    );
}