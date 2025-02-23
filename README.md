# Buscador de Archivos en Rust

Un buscador de archivos eficiente implementado en Rust que utiliza paralelismo para buscar archivos en todas las unidades disponibles del sistema.

## Características

- Búsqueda en paralelo utilizando múltiples hilos
- Insensible a mayúsculas/minúsculas
- Muestra el tiempo de búsqueda en tiempo real
- Detecta automáticamente todas las unidades disponibles
- Interfaz de línea de comandos simple

## Requisitos

- Rust 1.70 o superior
- Cargo (incluido con Rust)

## Dependencias

- walkdir = "2.4.0" - Para recorrer directorios
- colored = "2.1.0" - Para colorear la salida
- sysinfo = "0.30.5" - Para detectar unidades de disco
- rayon = "1.8" - Para paralelismo

## Uso

```bash
cargo run -- <nombre_archivo>
```

Por ejemplo:
```bash
cargo run -- documento.txt
```

## Estructura del Proyecto

- `src/main.rs` - Código fuente principal
- `Cargo.toml` - Archivo de configuración y dependencias
- `README.md` - Este archivo

## Control de Versiones

El proyecto usa Git para control de versiones. Cada commit representa una versión estable del código.
