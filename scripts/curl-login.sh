#curl -vv http://lokoda.co.uk/api/login -H "Content-Type: application/json" -d '{"email":"dave.gill@somewhere.com", "password":"password"}'
curl -vv http://127.0.0.1:8888/login -H "Content-Type: application/json" -d '{"email":"dave.gill@somewhere.com", "password":"password"}' | jq
