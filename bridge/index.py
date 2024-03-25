import requests
import json

url = "https://billowing-fittest-feather.btc-testnet.quiknode.pro/4613ae0ad4238d1261f99cc2cd8baa48f6b96d83/"

payload = json.dumps({
  "method": "getblock",
  "params": [
    "00000000003094b8263777a801e959d8280ef5cfc7f94a6a2f936f0ffb9d4803"
  ]
})
headers = {
  'Content-Type': 'application/json'
}

response = requests.request("POST", url, headers=headers, data=payload)

print(response.text)
