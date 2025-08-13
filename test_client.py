#!/usr/bin/env python3
"""Script de teste para o servidor MCP do iTerm"""

import json
import socket
import sys

def send_mcp_message(host, port, message):
    """Envia uma mensagem MCP para o servidor e recebe a resposta"""
    try:
        # Conecta ao servidor
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((host, port))
            
            # Envia a mensagem JSON com newline
            message_str = json.dumps(message) + '\n'
            s.sendall(message_str.encode('utf-8'))
            
            # Recebe a resposta
            response = b''
            while True:
                chunk = s.recv(1024)
                if not chunk:
                    break
                response += chunk
                if b'\n' in response:
                    break
            
            # Decodifica e retorna a resposta
            response_str = response.decode('utf-8').strip()
            return json.loads(response_str)
    
    except Exception as e:
        print(f"Erro ao enviar mensagem: {e}")
        return None

def main():
    host = '127.0.0.1'
    port = 3333
    
    print(f"Testando servidor MCP em {host}:{port}")
    print("-" * 50)
    
    # Teste 1: Escrever no terminal
    print("\n1. Testando write_to_terminal...")
    message = {
        "id": "test-1",
        "function": "iterm-mcp:write_to_terminal",
        "arguments": {
            "command": "echo 'Hello from MCP server!'"
        }
    }
    response = send_mcp_message(host, port, message)
    print(f"Resposta: {json.dumps(response, indent=2)}")
    
    # Teste 2: Ler output do terminal
    print("\n2. Testando read_terminal_output...")
    message = {
        "id": "test-2",
        "function": "iterm-mcp:read_terminal_output",
        "arguments": {
            "linesOfOutput": 5
        }
    }
    response = send_mcp_message(host, port, message)
    print(f"Resposta: {json.dumps(response, indent=2)}")
    
    # Teste 3: Enviar caractere de controle
    print("\n3. Testando send_control_character...")
    message = {
        "id": "test-3",
        "function": "iterm-mcp:send_control_character",
        "arguments": {
            "letter": "C"
        }
    }
    response = send_mcp_message(host, port, message)
    print(f"Resposta: {json.dumps(response, indent=2)}")
    
    # Teste 4: Função inexistente
    print("\n4. Testando função inexistente...")
    message = {
        "id": "test-4",
        "function": "iterm-mcp:unknown_function",
        "arguments": {}
    }
    response = send_mcp_message(host, port, message)
    print(f"Resposta: {json.dumps(response, indent=2)}")
    
    # Teste 5: JSON inválido
    print("\n5. Testando JSON inválido...")
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((host, port))
            s.sendall(b'{"invalid json\n')
            response = s.recv(1024).decode('utf-8').strip()
            print(f"Resposta: {response}")
    except Exception as e:
        print(f"Erro: {e}")

if __name__ == "__main__":
    main()
