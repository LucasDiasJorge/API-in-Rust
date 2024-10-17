#!/bin/bash

# Test user registration
echo "Testing registration..."
curl -X POST http://127.0.0.1:8080/register -H "Content-Type: application/json" -d '{"username":"testuser", "password":"password123"}'

# Test user login
echo -e "\nTesting login..."
response=$(curl -X POST http://127.0.0.1:8080/login -H "Content-Type: application/json" -d '{"username":"testuser", "password":"password123"}')
token=$(echo $response | jq -r '.message')

# Test protected route
echo -e "\nTesting protected route..."
curl -X POST http://127.0.0.1:8080/json -H "Authorization: Bearer $token" -H "Content-Type: application/json" -d '{"name":"John", "age":30}'
