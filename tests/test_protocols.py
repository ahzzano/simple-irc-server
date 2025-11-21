import socket

HOST = "127.0.0.1" 
PORT = 25565  

stuff_to_send = [
	"Hello World!"
]

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))

    for st in stuff_to_send:
    	s.sendall(bytes(st, 'utf-8'))