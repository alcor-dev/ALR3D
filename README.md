# ALR3D

El objetivo del proyecto es tener un pequeño renderizador ligero basado en [wGPU](https://wgpu.rs/) y escrito en [Rust](https://www.rust-lang.org/).
El proyecto sirvió como __TFG__ y me ayudó a entender un poco mejor como funciona tanto un pipeline 3D como 
ciertas prácticas usadas en Rust

# Objetivos

Podríamos dividir este proyecto en dos futuros subproyectos que podrían servir para un mayor aprendizaje tanto
en 3D como en programación propia de Rust:


- Visualizador 3D de archivos de formato __STL__
  - Pensado como visualizador ligero de archivos de formato __STL__
  - __STL__ es el formato más comúnmente usado dentro de impresión 3D
  - Sería un programa con muy poco gasto de recursos de la máquina que permita ver las figuritas
  - ¿Quizás evolucionar esto a un slicer? Un poco complejo pero quizás un objetivo a futuro
  

- Visualizador 3D de archivos de formato FBX
  - Pensado como visualizador ligero de archivos de formato FBX
  - FBX es el formato más usado hoy día a la hora de trabajar
  - FBX puede guardar mucha información, siendo lo importante, modelos 3D, animaciones, texturas, etc.
  - La idea es que pueda ser un visualizador rápido donde puedas ver las animaciones asociadas a los modelos contenidos
  - Quizás incluso añadir una barra que permita hacer scrubbing para poder estudiar las animaciones

# Ideas descartadas

Evidentemente incluso había barajado más posibilidades tales como:
- [x] Motor de juegos 3D
- [x] Visualizador online como prueba para webAssembly
- [x] Locuras varias

Pero al faltar experiencia en desarrollo, es preferible esperar a conseguir más conocimientos antes de volcarme en proyectos que pueda
dejar tirados hacia la mitad de su desarrollo por burnout, cansancio y/o frustración
