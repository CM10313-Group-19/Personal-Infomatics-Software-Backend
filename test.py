import requests

url = "http://127.0.0.1:8000"

print(requests.get(url + "/check_email", json={"email": "email@email.com"}).json())

print(
    requests.post(
        url + "/signup",
        json={
            "email": "email@email.com",
            "password": "qwerty",
            "date_of_birth": "2023-03-01",
        },
    ).json()
)

print(
    requests.get(
        url + "/login", json={"email": "email@email.com", "password": "qwerty"}
    ).json()
)
