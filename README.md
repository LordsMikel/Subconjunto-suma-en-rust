# Subconjunto Suma Eficiente en Rust

Este repositorio contiene una implementación eficiente del algoritmo de subconjunto suma en Rust utilizando tablas de resultados previos. El algoritmo resuelve el problema de determinar si existe un subconjunto de un conjunto dado cuya suma sea igual a un objetivo especificado.

## ¿Cómo funciona?

El algoritmo utiliza programación dinámica y tabulación para evitar recalcular subproblemas. Aquí hay una explicación del proceso paso a paso:

1. Se crea una tabla `dp` para almacenar los resultados previos. La tabla `dp` tiene dimensiones `(n + 1) x (target + 1)`, donde `n` es la longitud del vector de entrada y `target` es el objetivo especificado. Se inicializan todos los elementos de la tabla en `false`, excepto `dp[0][0]`, que se inicializa en `true`.

2. Se utilizan dos bucles `for` anidados para recorrer los elementos del vector de entrada y los posibles valores objetivo. El bucle exterior itera sobre los elementos del vector, y el bucle interior itera en reversa desde `target` hasta el elemento actual.

3. Para cada elemento y valor objetivo, se verifica si es posible construir la suma actual utilizando un subconjunto anterior. Esto se hace mediante la comprobación del valor en `dp[i - 1][j - num]`, donde `i` es el índice actual, `j` es el valor objetivo y `num` es el elemento actual del vector. Si la comprobación es verdadera, se establece `dp[i][j]` en `true` porque es posible construir la suma actual agregando el elemento actual al subconjunto anterior.

4. Una vez que se han completado los bucles anidados, se devuelve el valor en `dp[n][target]`, donde `n` es la longitud del vector de entrada y `target` es el objetivo especificado. Este valor indica si existe o no un subconjunto que sume el objetivo.

## Complejidad del algoritmo

La implementación utiliza una estrategia eficiente basada en programación dinámica y tabulación. La complejidad del algoritmo es `O(n * target)`, donde `n` es la longitud del vector de entrada y `target` es el objetivo especificado. La tabla `dp` tiene `n * target` elementos, y cada elemento se calcula en tiempo constante. Esto permite resolver el problema de manera eficiente incluso para conjuntos grandes y objetivos grandes.

## Uso

El código proporcionado en este repositorio es un ejemplo funcional de la implementación del algoritmo en Rust. Para ejecutar el ejemplo, asegúrate de tener Rust instalado en tu sistema y luego sigue estos pasos:

1. Clona este repositorio en tu máquina local.

2. Navega al directorio del repositorio en tu terminal.

3. Ejecuta el siguiente comando para compilar y ejecutar el código:

   ```bash
   cargo run
   ````
