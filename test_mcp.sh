#!/bin/bash
# Script de teste simples para o servidor MCP

echo "Testando servidor MCP do iTerm..."
echo "================================"

# Teste 1: write_to_terminal
echo -e "\n1. Testando write_to_terminal..."
echo '{"id":"test-1","function":"iterm-mcp:write_to_terminal","arguments":{"command":"echo Hello MCP!"}}' | nc localhost 3333

# Aguarda um pouco
sleep 1

# Teste 2: read_terminal_output
echo -e "\n\n2. Testando read_terminal_output..."
echo '{"id":"test-2","function":"iterm-mcp:read_terminal_output","arguments":{"linesOfOutput":5}}' | nc localhost 3333

# Aguarda um pouco
sleep 1

# Teste 3: send_control_character
echo -e "\n\n3. Testando send_control_character..."
echo '{"id":"test-3","function":"iterm-mcp:send_control_character","arguments":{"letter":"C"}}' | nc localhost 3333

# Aguarda um pouco
sleep 1

# Teste 4: Função inexistente
echo -e "\n\n4. Testando função inexistente..."
echo '{"id":"test-4","function":"unknown:function","arguments":{}}' | nc localhost 3333

echo -e "\n\nTestes concluídos!"
