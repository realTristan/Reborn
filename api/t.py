import requests

r = requests.post("http://localhost:8080/account/login", json={
    "identifier": "test"
})
print(r.status_code)
print(r.text)