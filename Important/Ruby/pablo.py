print("Piedra, papel o tijera!!")
jugador1 = input("Ingrese piedra, papel o tijera: ").lower()
jugador2 = input("Ingrese piedra, papel o tijera: ").lower()

def piedra(a,b):
    if a == b :
        print("EMPATE")
    elif a == "piedra" and b == "tijera":
        print(f"{a} es el ganador")
    elif a == "piedra" and b == "papel":
        print(f"{b} es el ganador")

def tijera(a,b):
    if a == b :
        print("EMPATE")
    elif a == "tijera" and b == "piedra":
        print(f"{b} es el ganador")
    elif a == "tijera" and b == "papel":
        print(f"{a} es el ganador")

def papel(a,b):
    if a == b :
        print("EMPATE")
    elif a == "papel" and b == "tijera":
        print(f"{b} es el ganador")
    elif a == "papel" and b == "piedra":
        print(f"{a} es el ganador")

papel(jugador1,jugador2)
piedra(jugador1,jugador2)
tijera(jugador1,jugador2)