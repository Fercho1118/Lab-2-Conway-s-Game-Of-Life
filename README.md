# Conway’s Game of Life

Este proyecto implementa el famoso Conway’s Game of Life usando Rust y la librería gráfica raylib.
El renderizado se hace píxel por píxel mediante un framebuffer personalizado.

## ¿Qué hace?

- Simula las reglas clásicas del Game of Life.

- Renderiza en tiempo real usando raylib.

- Muestra un patrón inicial creativo y dinámico con:

  - Pulsars en cruz central

  - Spaceships (gliders, LWSS, MWSS, HWSS)

  - Osciladores (Blinkers, Beacons, Toads)

  - Still lifes (Block, Boat, Loaf, Beehive, Tub)

- La simulación evoluciona automáticamente cada cierto número de frames.


## Resultado final

![Demo](./game_of_life.gif)

## Requisitos

- Rust 1.88+
- CMake instalado
- Librería `raylib` como dependencia (agregada vía Cargo)

## Cómo correr

```bash
cargo run
```

## Contacto

- **Estudiante:** Fernando Rueda  
- **Carnet:** 23748  
- **Email:** rue3748@uvg.edu.gt

