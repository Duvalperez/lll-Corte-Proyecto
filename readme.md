
# Procesador de Expresiones Aritméticas Estático en Rust 🦀

Este proyecto consiste en un **Mini-Traductor y Procesador de Expresiones Aritméticas** desarrollado en el lenguaje de programación de sistemas **Rust**. 

El sistema es capaz de analizar léxicamente expresiones en notación infija leídas desde un archivo plano, validar su sintaxis frente a un diccionario de símbolos dinámico y traducirlas a sus equivalentes en **Notación Postfija (Polaca Inversa)** y **Notación Prefija (Polaca)**.

Desarrollado bajo especificaciones académicas estrictas para la cátedra de **Lenguajes de Programación** del Decanato de Ciencias y Tecnología de la **Universidad Centroccidental Lisandro Alvarado (UCLA)**.

---

## 📌 Recursos del Proyecto

*   **Presentación del Proyecto:** Puedes visualizar las diapositivas de la defensa del proyecto directamente en Canva a través del siguiente enlace:  
    👉 [Presentación en Canva](https://www.canva.com/design/DAHPm5sKuI8/qHPQdledd5NbFjPGnizERA/edit)

---

## 🚀 Características Principales

*   **Gestión Estricta de Memoria en el Stack:** Cumplimiento del 100% de la prohibición de asignación dinámica de memoria (`Heap`). No se utilizan colecciones dinámicas (`Vec`, `String`, etc.). Toda la lógica de análisis y traducción opera sobre arreglos nativos fijos pre-asignados en la pila de llamadas de la CPU, logrando una latencia de asignación de 0 ns.
*   **Gestión de Tamaño Lógico:** Evita el procesamiento de basura o celdas vacías (`'\0'`) mediante variables de control que limitan el recorrido del arreglo a su tamaño útil real en memoria.
*   **Análisis Léxico Robusto:** Validación de caracteres en tiempo de ejecución. Si se detecta un símbolo no definido en la tabla de entrada, el compilador interrumpe la ejecución de forma segura con un error controlado sin provocar pánicos en el sistema (`panic!`).
*   **Arquitectura Desacoplada:** El motor de precedencias es dinámico. Las reglas de precedencia, operadores y operandos se leen directamente de la tabla de símbolos del archivo de entrada, permitiendo escalar o alterar la semántica gramatical sin recompilar el código fuente.

---

## 🛠️ Arquitectura de Datos (Abstracción en el Stack)

Para garantizar la seguridad y eficiencia en la memoria estática de Rust, el sistema se basa en tipos de datos con semántica de copia estructurada mediante la derivación de los traits `Copy` y `Clone`:

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
enum TipoToken {
    OPERANDO,
    OPERADOR,
    DESCONOCIDO,
}

#[derive(Copy, Clone, Debug)]
struct Token {
    caracter: char,
    tipo: TipoToken,
    precedencia: i32,
}

struct PilaOperadores {
    elementos: [Token; 100],
    tope: usize, // Puntero lógico del Stack
}
📐 Flujo del Algoritmo (Shunting-yard)
El ciclo de vida de un token es procesado de forma lineal a través de una máquina de estados estática que determina su desvío inmediato a la salida o su apilamiento temporal basado en precedencias:

Plaintext
       [ INICIO ]
           │
           ▼
 ┌──────────────────┐
 │ Leer Sig. Token  │ ◄─────────┐
 └─────────┬────────┘           │
           │ (Iteración)        │
           ▼                    │
 ¿Es Operador u Operando?       │
     │                          │
     ├── OPERANDO ──► [ Enviar a la Salida ]
     │                          │
     └── OPERADOR ──► ¿Precedencia tope >= Token actual?
                          ├── SÍ ─► [ Pop de la Pila a Salida ]
                          └── NO ─► [ Push del Token a la Pila ]
                                        │
           ┌────────────────────────────┘
           ▼
    ¿Quedan más tokens? ────────────────┘
           │
           └── NO ──► [ Vaciar Pila a la Salida ]
                            │
                            ▼
                         [ FIN ]
📋 Estructura Obligatoria de entrada.txt
El archivo entrada.txt debe ubicarse en la raíz del proyecto (junto a Cargo.toml). El formato requiere la expresión matemática en la primera línea, seguida de las especificaciones léxicas y de precedencia de cada símbolo:

Plaintext
EXPRESION=A+B*C
A,OPERANDO,0
B,OPERANDO,0
C,OPERANDO,0
+,OPERADOR,1
-,OPERADOR,1
*,OPERADOR,2
/,OPERADOR,2
⚙️ Instrucciones de Instalación y Ejecución
Prerrequisitos
Debe tener instalado el compilador oficial de Rust y su gestor de paquetes Cargo. Si no los tiene, puede instalarlos mediante:

Linux / macOS:

Bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
Windows: Descargue e instale el ejecutable desde rustup.rs.

Inicialización y Corrida
Clona este repositorio o descarga los archivos del proyecto.

Asegúrate de que el archivo entrada.txt esté presente en la raíz del directorio.

Ejecuta el programa en modo de desarrollo:

Bash
cargo run
Para compilar una versión optimizada de alto rendimiento para producción (sin información de depuración):

Bash
cargo build --release
El ejecutable optimizado se generará en la ruta target/release/.

🛡️ Robustez y Casos de Borde Evaluados
Archivo Inexistente: Se interceptan los fallos de lectura de disco (E/S) del sistema operativo de manera limpia empleando combinadores funcionales como map_err, evitando la interrupción abrupta del sistema.

Límite de Expresión: La pila interna (PilaOperadores) valida proactivamente los límites físicos del arreglo antes de cada operación push, impidiendo desbordamientos de búfer (Buffer Overflows).

Errores Léxicos Controlados: Si la expresión contiene símbolos ausentes en el diccionario provisto (por ejemplo, A#B), el analizador léxico corta la ejecución informando el carácter inválido y su índice exacto en la cadena.

👨‍💻 Autores
Auris Rodríguez (V-32.273.690) - Universidad Centroccidental Lisandro Alvarado (UCLA)

Duval Pérez (V-30.895.305) - Universidad Centroccidental Lisandro Alvarado (UCLA)

Cátedra de Lenguajes de Programación • Lapso Académico 2026-1
