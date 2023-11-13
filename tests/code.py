import math

def calcular_numero_paginas(cantidad_imagenes, espacios_en_grid):
    return math.ceil(cantidad_imagenes / espacios_en_grid)

def agregar_imagenes_al_grid(grid, imagenes):
    espacios_en_grid = 9  # Puedes cambiar esto si tu grid tiene un tamaño diferente
    numero_paginas = calcular_numero_paginas(len(imagenes), espacios_en_grid)
    for pagina in [0,1,2]:
        inicio = pagina * espacios_en_grid
        fin = (pagina + 1) * espacios_en_grid
        imagenes_pagina = imagenes[inicio:fin]

        # Lógica para agregar imágenes al grid (debes adaptar esto según tu implementación GTK)
        for imagen in imagenes_pagina:
            # Agregar imagen al grid (sustituye esto con tu lógica GTK)
            print(f"Agregando imagen al grid, pagina {pagina}")

        # Eliminar las imágenes ya agregadas del vector
        del imagenes[inicio:fin]

# Ejemplo de uso
imagenes_encontradas = ["imagen1", "imagen2", "imagen3", "imagen4", "imagen5", "imagen6", "imagen7", "imagen8", "imagen9", "imagen10", "imagen11", "imagen12", "imagen13", "imagen14", "imagen15", "imagen16", "imagen17"]

# Llamar a la función para agregar imágenes al grid
agregar_imagenes_al_grid(None, imagenes_encontradas)
