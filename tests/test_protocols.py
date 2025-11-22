import socket

HOST = "127.0.0.1" 
PORT = 25565  

stuff_to_send = [
	"Traveling in a land down under\n"
	"USER guest tolmoon tolsun :Ronnie Reagan\n",
	"NICK YouMakeMeNervous\n",
	"PASS I_come_from_a_land_down_under\n"
]

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
	s.connect((HOST, PORT))

	for st in stuff_to_send:
		print(f"sending \"{st.strip()}\"...")
		s.sendall(bytes(st, 'utf-8'))
		
		res = s.recv(1024)

	print('finished')