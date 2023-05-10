import requests

r = requests.put("http://localhost:8080/account/register", json={
    "identifier": "test",
    "username": "test"
})
print(r.status_code)
print(r.text)