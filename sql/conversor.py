def convertir_nombre(nombre):
    (tipo, nombre) = nombre.split("u", 1)
    return f'UFSC {nombre.title()}'

def convertir_coordenadas(punto):
    (latitud, longitud) = punto.split(",")
    latitud = latitud.lstrip("(").rstrip()
    longitud = longitud.lstrip().rstrip(")")

    return latitud, longitud

def convertir_sentencia(sentencia):
    (d, datos) = sentencia.split("VALUES")

    return [item.strip("(").strip(")").rstrip(",").rstrip(";").rstrip(")").strip("'") for item in datos.split()]

def crear_nuevas_sentencias(datos):
    nombre = convertir_nombre(datos[1])
    (latitud, longitud) = convertir_coordenadas(datos[3])
    sentencia_establecimiento = f'SELECT crear_establecimiento(\'{nombre}\', {latitud}, {longitud});'
    sentencia_servidor = f'SELECT crear_servidor(\'{datos[1]}\', \'{datos[2]}\', \'{nombre}\');'
    return f'{sentencia_establecimiento}\n{sentencia_servidor}' 

if __name__ == "__main__":
    with open("datos_prueba_minsal.sql") as fichero:
        datos = (crear_nuevas_sentencias(convertir_sentencia(linea)) for linea in fichero if linea.startswith("INSERT"))
        for resultado in datos:
            print(resultado)
