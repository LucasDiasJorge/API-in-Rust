#!/bin/bash

# URL base do servidor
BASE_URL="http://127.0.0.1:8080"

# Testando a rota de registro
echo "Teste da rota de registro:"
REGISTER_RESPONSE=$(curl -s -X POST $BASE_URL/register -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}')
echo "Resposta do registro: $REGISTER_RESPONSE"

# Testando a rota de login
echo -e "\nTeste da rota de login:"
LOGIN_RESPONSE=$(curl -s -X POST $BASE_URL/login -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}')
echo "Resposta do login: $LOGIN_RESPONSE"

# Testando a rota de registro com usuário existente (para verificar erro)
echo -e "\nTeste de registro com usuário existente:"
REGISTER_DUPLICATE_RESPONSE=$(curl -s -X POST $BASE_URL/register -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}')
echo "Resposta do registro duplicado: $REGISTER_DUPLICATE_RESPONSE"

# Testando a rota de login com credenciais incorretas
echo -e "\nTeste de login com credenciais incorretas:"
LOGIN_INVALID_RESPONSE=$(curl -s -X POST $BASE_URL/login -H "Content-Type: application/json" -d '{"username": "testuser", "password": "wrongpassword"}')
echo "Resposta do login inválido: $LOGIN_INVALID_RESPONSE"

# Testando a rota de login com credenciais corretas
echo -e "\nTeste de login com credenciais corretas:"
LOGIN_VALID_RESPONSE=$(curl -s -X POST $BASE_URL/login -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}')
echo "Resposta do login válido: $LOGIN_VALID_RESPONSE"
