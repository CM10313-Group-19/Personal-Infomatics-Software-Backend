import requests
import json
url = "http://127.0.0.1:8000"

print(requests.post(url + "/signup", json={'email': 'est@email.com', 'password': 'qwerty', 'date_of_birth': '2023-03-01'}).json())
print(requests.get(url + "/login", json={'email': 'est@mail.com', 'password': 'qwerty'}).json())


