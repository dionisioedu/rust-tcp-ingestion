import socket
import json
import time
import random

HOST = 'localhost'  # ou IP do seu servidor, se estiver remoto
PORT = 7878         # mesma porta que seu container expôs

def generate_message(sensor_id):
    message = {
        "source": f"sensor-{sensor_id}",
        "value": round(random.uniform(10.0, 100.0), 2),
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    }
    return json.dumps(message)

def main():
    for i in range(1, 501):  # 500 mensagens!
        try:
            with socket.create_connection((HOST, PORT)) as sock:
                message = generate_message(i)
                sock.sendall(message.encode('utf-8'))
                print(f"[{i}] Sent: {message}")
                time.sleep(0.01)  # 10ms entre envios para simular fluxo real
        except Exception as e:
            print(f"Error sending message {i}: {e}")

if __name__ == "__main__":
    main()
