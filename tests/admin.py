import requests


class RbClient:
    def __init__(self, username = "admin", password = "password", base_url = "http://localhost:8000/api"):
        self.username = username
        self.password = password
        self.base_url = base_url

        self.jwt = None
        self.refresh_token = None

    def _login(self):
        r = requests.post(f"{self.base_url}/auth/login", json={
            "username": self.username,
            "password": self.password,
        })

        if r.status_code != 200:
            print(r.text)
            raise Exception("Couldn't login")

        res = r.json()
        self.jwt = res["token"]
        self.refresh_token = res["refreshToken"]

    def _refresh(self):
        r = requests.post(f"{self.base_url}/auth/refresh", json={"refreshToken": self.refresh_token})

        if r.status_code != 200:
            raise Exception("Couldn't refresh")

        res = r.json()
        self.jwt = res["token"]
        self.refresh_token = res["refreshToken"]

    def _request(self, type_, url, retry=2, *args, **kwargs):
        if self.jwt:
            headers = kwargs.get("headers", {})
            headers["Authorization"] = f"Bearer {self.jwt}"
            kwargs["headers"] = headers
            print(kwargs["headers"])

        r = requests.request(type_, url, *args, **kwargs)

        if r.status_code != 200 and retry > 0:
            if self.refresh_token:
                self._refresh()

            else:
                self._login()

            r = self._request(type_, url, *args, **kwargs, retry=retry - 1)

        return r

    def get(self, url, *args, **kwargs):
        return self._request("GET", f"{self.base_url}{url}", *args, **kwargs)

    def post(self, url, *args, **kwargs):
        return self._request("POST", f"{self.base_url}{url}", *args, **kwargs)



if __name__ == "__main__":
    client = RbClient()

    # print(client.get("/admin/users").json())
    client.post("/sections", json={
        "title": "this is a title"
    })
